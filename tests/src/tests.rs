use anyhow::{anyhow, Context};
use monitor_client::{
    types::{Build, Conversion, Deployment, Server, SystemStats, Update},
    MonitorClient,
};

pub async fn create_test_setup(
    monitor: &MonitorClient,
    group_name: &str,
) -> anyhow::Result<(Server, Deployment, Build)> {
    let server = monitor
        .create_server(&format!("{group_name}_server"), "http://localhost:9001")
        .await
        .context("failed at create server")?;
    let deployment = monitor
        .create_deployment(&format!("{group_name}"), &server.id)
        .await
        .context("failed at create deployment")?;
    let build = monitor
        .create_build(&format!("{group_name}_build"), &server.id)
        .await
        .context("failed at create build")?;
    Ok((server, deployment, build))
}

pub async fn get_server_stats(monitor: &MonitorClient) -> anyhow::Result<SystemStats> {
    let servers = monitor
        .list_servers()
        .await
        .context("failed at list servers")?;
    let server = servers.get(0).ok_or(anyhow!("no servers"))?;
    let stats = monitor
        .get_server_stats(&server.id)
        .await
        .context("failed at get server stats")?;
    Ok(stats)
}

pub async fn deploy_mongo(monitor: &MonitorClient) -> anyhow::Result<Update> {
    let servers = monitor
        .list_servers()
        .await
        .context("failed at list servers")?;
    let server = servers.get(0).ok_or(anyhow!("no servers"))?;
    println!("got server");
    let mut deployment = monitor.create_deployment("mongo_test", &server.id).await?;
    println!("created deployment");
    deployment.docker_run_args.image = "mongo".to_string();
    deployment.docker_run_args.ports.push(Conversion {
        local: "27020".to_string(),
        container: "27017".to_string(),
    });
    let deployment = monitor.update_deployment(deployment).await?;
    println!("updated deployment");
    let update = monitor.deploy(&deployment.id).await?;
    Ok(update)
}
