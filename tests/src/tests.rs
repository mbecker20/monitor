use anyhow::{anyhow, Context};
use monitor_client::{
    futures_util::StreamExt,
    tokio_tungstenite::tungstenite::Message,
    types::{
        AwsBuilderBuildConfig, AwsBuilderConfig, Build, BuildBuilder, Command, Conversion,
        Deployment, DeploymentWithContainerState, DockerBuildArgs, DockerBuildArgsBuilder, Server,
        SystemStats, Update,
    },
    MonitorClient,
};

pub async fn create_test_setup(
    monitor: &MonitorClient,
    group_name: &str,
) -> anyhow::Result<(Server, Deployment, Build)> {
    let mut servers = monitor.list_servers(None).await?;
    let server = if servers.is_empty() {
        monitor
            .create_server(&format!("{group_name}_server"), "http://localhost:8000")
            .await
            .context("failed at create server")?
    } else {
        servers.pop().unwrap().server
    };
    let mut deployments = monitor.list_deployments(None).await?;
    let deployment = if deployments.is_empty() {
        monitor
            .create_deployment(&format!("{group_name}_deployment"), &server.id)
            .await
            .context("failed at create deployment")?
    } else {
        deployments.pop().unwrap().deployment
    };
    let mut builds = monitor.list_builds(None).await?;
    let build = if builds.is_empty() {
        monitor
            .create_build(&format!("{group_name}_build"))
            .await
            .context("failed at create build")?
    } else {
        builds.pop().unwrap()
    };
    Ok((server, deployment, build))
}

pub async fn get_server_stats(monitor: &MonitorClient) -> anyhow::Result<SystemStats> {
    let servers = monitor
        .list_servers(None)
        .await
        .context("failed at list servers")?;
    let server = &servers.get(0).ok_or(anyhow!("no servers"))?.server;
    let stats = monitor
        .get_server_stats(&server.id, None)
        .await
        .context("failed at get server stats")?;
    Ok(stats)
}

pub async fn subscribe_to_server_stats(monitor: &MonitorClient) -> anyhow::Result<()> {
    let servers = monitor
        .list_servers(None)
        .await
        .context("failed at list servers")?;
    let server = &servers.get(0).ok_or(anyhow!("no servers"))?.server;
    let (mut recv, _, _) = monitor.subscribe_to_stats_ws(&server.id, None).await?;
    while let Ok(stats) = recv.recv().await {
        println!("{stats:#?}");
    }
    Ok(())
}

pub async fn deploy_mongo(
    monitor: &MonitorClient,
) -> anyhow::Result<(Update, DeploymentWithContainerState)> {
    let servers = monitor
        .list_servers(None)
        .await
        .context("failed at list servers")?;
    let server = &servers.get(0).ok_or(anyhow!("no servers"))?.server;
    let mut deployment = monitor.create_deployment("mongo_test", &server.id).await?;
    println!("created deployment");
    deployment.docker_run_args.image = "mongo".to_string();
    deployment.docker_run_args.ports.push(Conversion {
        local: "27020".to_string(),
        container: "27017".to_string(),
    });
    let deployment = monitor.update_deployment(deployment).await?;
    println!("updated deployment");
    let update = monitor.deploy_container(&deployment.id).await?;
    let container = monitor.get_deployment(&deployment.id).await?;
    Ok((update, container))
}

pub async fn test_build(monitor: &MonitorClient) -> anyhow::Result<Update> {
    let servers = monitor
        .list_servers(None)
        .await
        .context("failed at list servers")?;
    let server = &servers.get(0).ok_or(anyhow!("no servers"))?.server;
    let mut build = monitor.create_build("old_periphery").await?;
    println!("created build. updating...");
    build.repo = Some("mbecker20/monitor".to_string());
    // build.branch = Some("");
    build.pre_build = Some(Command {
        path: ".".to_string(),
        command: "yarn && cd periphery && yarn build".to_string(),
    });
    build.docker_build_args = Some(DockerBuildArgs {
        build_path: "periphery".to_string(),
        ..Default::default()
    });
    let build = monitor.update_build(build).await?;
    println!("updated build.");
    let update = monitor.build(&build.id).await?;
    Ok(update)
}

pub async fn test_updates(monitor: &MonitorClient) -> anyhow::Result<()> {
    let updates = monitor.list_updates(None, 0).await?;
    println!("ALL UPDATES: {updates:#?}");
    let builds = monitor.list_builds(None).await?;
    let build = builds.get(0).unwrap();
    let build_updates = monitor.list_updates(build, 0).await?;
    println!("{build_updates:#?}");
    Ok(())
}

pub async fn test_aws_build(monitor: &MonitorClient) -> anyhow::Result<()> {
    let build = BuildBuilder::default()
        .name("test_monitor".to_string())
        .repo(Some(String::from("mbecker20/monitor")))
        .branch(Some(String::from("main")))
        .docker_account(Some(String::from("mbecker2020")))
        .docker_build_args(
            DockerBuildArgsBuilder::default()
                .build_path(".".to_string())
                .dockerfile_path("Dockerfile.core".to_string().into())
                .build()
                .context("failed to construct DockerBuildArgs struct")?
                .into(),
        )
        .aws_config(AwsBuilderBuildConfig::default().into())
        .build()
        .context("failed to construct Build struct")?;
    let build = monitor.create_full_build(&build).await?;
    println!("{build:#?}\n");
    let update = monitor.build(&build.id).await?;
    println!("{update:#?}");
    Ok(())
}
