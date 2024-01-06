use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, Context};
use aws_config::BehaviorVersion;
use aws_sdk_ec2::{
  config::Region,
  types::{
    BlockDeviceMapping, EbsBlockDevice,
    InstanceNetworkInterfaceSpecification, InstanceStateChange,
    InstanceStateName, InstanceStatus, InstanceType, ResourceType,
    Tag, TagSpecification, VolumeType,
  },
  Client,
};
use monitor_types::requests::write::LaunchAwsServerConfig;

use crate::state::State;

const POLL_RATE_SECS: u64 = 2;
const MAX_POLL_TRIES: usize = 30;

pub struct Ec2Instance {
  pub instance_id: String,
  pub ip: String,
}

impl State {
  async fn create_ec2_client(&self, region: String) -> Client {
    // There may be a better way to pass these keys to client
    std::env::set_var(
      "AWS_ACCESS_KEY_ID",
      &self.config.aws.access_key_id,
    );
    std::env::set_var(
      "AWS_SECRET_ACCESS_KEY",
      &self.config.aws.secret_access_key,
    );
    let region = Region::new(region);
    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
      .region(region)
      .load()
      .await;
    Client::new(&config)
  }

  pub async fn launch_ec2_instance(
    &self,
    name: &str,
    config: impl Into<LaunchAwsServerConfig>,
  ) -> anyhow::Result<Ec2Instance> {
    let LaunchAwsServerConfig {
      region,
      instance_type,
      volumes,
      ami_id,
      subnet_id,
      security_group_ids,
      key_pair_name,
      assign_public_ip,
    } = config.into();
    let instance_type = InstanceType::from(instance_type.as_str());
    if let InstanceType::Unknown(t) = instance_type {
      return Err(anyhow!("unknown instance type {t:?}"));
    }
    let client = self.create_ec2_client(region.clone()).await;
    let mut req = client
      .run_instances()
      .image_id(ami_id)
      .instance_type(instance_type)
      .network_interfaces(
        InstanceNetworkInterfaceSpecification::builder()
          .subnet_id(subnet_id)
          .associate_public_ip_address(assign_public_ip)
          .set_groups(security_group_ids.to_vec().into())
          .device_index(0)
          .build(),
      )
      .key_name(key_pair_name)
      .tag_specifications(
        TagSpecification::builder()
          .tags(Tag::builder().key("Name").value(name).build())
          .resource_type(ResourceType::Instance)
          .build(),
      )
      .min_count(1)
      .max_count(1);

    for volume in volumes {
      let mut ebs = EbsBlockDevice::builder()
        .volume_size(volume.size_gb)
        .set_iops(volume.iops)
        .set_throughput(volume.throughput);
      if let Some(volume_type) = &volume.volume_type {
        ebs = ebs.volume_type(
          VolumeType::from_str(volume_type)
            .context("invalid volume type")?,
        );
      }
      req = req.block_device_mappings(
        BlockDeviceMapping::builder()
          .set_device_name(volume.device_name.clone().into())
          .set_ebs(ebs.build().into())
          .build(),
      )
    }

    let res = req
      .send()
      .await
      .context("failed to start builder ec2 instance")?;

    let instance = res
      .instances()
      .first()
      .context("instances array is empty")?;

    let instance_id = instance
      .instance_id()
      .context("instance does not have instance_id")?
      .to_string();

    for _ in 0..MAX_POLL_TRIES {
      let state_name =
        get_ec2_instance_state_name(&client, &instance_id).await?;
      if state_name == Some(InstanceStateName::Running) {
        let ip = if assign_public_ip {
          get_ec2_instance_public_ip(&client, &instance_id).await?
        } else {
          instance
            .private_ip_address()
            .ok_or(anyhow!("instance does not have private ip"))?
            .to_string()
        };
        return Ok(Ec2Instance { instance_id, ip });
      }
      tokio::time::sleep(Duration::from_secs(POLL_RATE_SECS)).await;
    }
    Err(anyhow!("instance not running after polling"))
  }

  pub async fn terminate_ec2_instance(
    &self,
    region: String,
    instance_id: &str,
  ) -> anyhow::Result<InstanceStateChange> {
    let client = self.create_ec2_client(region).await;
    let res = client
      .terminate_instances()
      .instance_ids(instance_id)
      .send()
      .await
      .context("failed to terminate instance from aws")?
      .terminating_instances()
      .first()
      .context("terminating instances is empty")?
      .to_owned();
    Ok(res)
  }
}

async fn get_ec2_instance_status(
  client: &Client,
  instance_id: &str,
) -> anyhow::Result<Option<InstanceStatus>> {
  let status = client
    .describe_instance_status()
    .instance_ids(instance_id)
    .send()
    .await
    .context("failed to get instance status from aws")?
    .instance_statuses()
    .first()
    .cloned();
  Ok(status)
}

async fn get_ec2_instance_state_name(
  client: &Client,
  instance_id: &str,
) -> anyhow::Result<Option<InstanceStateName>> {
  let status = get_ec2_instance_status(client, instance_id).await?;
  if status.is_none() {
    return Ok(None);
  }
  let state = status
    .unwrap()
    .instance_state()
    .ok_or(anyhow!("instance state is None"))?
    .name()
    .ok_or(anyhow!("instance state name is None"))?
    .to_owned();
  Ok(Some(state))
}

async fn get_ec2_instance_public_ip(
  client: &Client,
  instance_id: &str,
) -> anyhow::Result<String> {
  let ip = client
    .describe_instances()
    .instance_ids(instance_id)
    .send()
    .await
    .context("failed to get instance status from aws")?
    .reservations()
    .first()
    .context("instance reservations is empty")?
    .instances()
    .first()
    .context("instances is empty")?
    .public_ip_address()
    .context("instance has no public ip")?
    .to_string();

  Ok(ip)
}