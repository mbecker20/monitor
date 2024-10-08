# Sync Resources

Komodo is able to create, update, delete, and deploy resources declared in TOML files by diffing them against the existing resources, 
and apply updates based on the diffs. Push the files to a remote git repo and create a `ResourceSync` pointing to the repo,
and the core backend will poll for any updates (you can also manually trigger an update poll / execution in the UI).

File detection is additive and recursive, so you can spread out your resource declarations across any number of files
and use any nesting of folders to organize resources inside a root folder. Additionally, you can create multiple `ResourceSyncs`
and each sync will be handled independently. This allows different syncs to manage resources on a "per-project" basis.

The UI will display the computed sync actions and only execute them upon manual confirmation.
Or the sync execution git webhook may be configured on the git repo to
automatically execute syncs upon pushes to the configured branch.

## Example Declarations

### Server

- [Server config schema](https://docs.rs/komodo_client/latest/komodo_client/entities/server/struct.ServerConfig.html)

```toml
[[server]] # Declare a new server
name = "server-prod"
description = "the prod server"
tags = ["prod"]
config.address = "http://localhost:8120"
config.region = "AshburnDc1"
config.enabled = true # default: false
```

### Builder and build

- [Builder config schema](https://docs.rs/komodo_client/latest/komodo_client/entities/builder/enum.BuilderConfig.html)
- [Build config schema](https://docs.rs/komodo_client/latest/komodo_client/entities/build/struct.BuildConfig.html)

```toml
[[builder]] # Declare a builder
name = "builder-01"
tags = []
config.type = "Aws"
config.params.region = "us-east-2"
config.params.ami_id = "ami-0e9bd154667944680"
# These things come from your specific setup
config.params.subnet_id = "subnet-xxxxxxxxxxxxxxxxxx"
config.params.key_pair_name = "xxxxxxxx"
config.params.assign_public_ip = true
config.params.use_public_ip = true
config.params.security_group_ids = [
  "sg-xxxxxxxxxxxxxxxxxx",
  "sg-xxxxxxxxxxxxxxxxxx"
]

##

[[build]]
name = "test_logger"
description = "Logs randomly at INFO, WARN, ERROR levels to test logging setups"
tags = ["test"]
config.builder_id = "builder-01"
config.repo = "mbecker20/test_logger"
config.branch = "master"
config.git_account = "mbecker20"
config.image_registry.type = "Standard"
config.image_registry.params.domain = "github.com" # or your custom domain
config.image_registry.params.account = "your_username"
config.image_registry.params.organization = "your_organization" # optinoal
# Set docker labels
config.labels = """
org.opencontainers.image.source = https://github.com/mbecker20/test_logger
org.opencontainers.image.description = Logs randomly at INFO, WARN, ERROR levels to test logging setups
org.opencontainers.image.licenses = GPL-3.0"""
```

### Deployments

- [Deployment config schema](https://docs.rs/komodo_client/latest/komodo_client/entities/deployment/struct.DeploymentConfig.html)

```toml
[[variable]] # Declare variables
name = "OTLP_ENDPOINT"
value = "http://localhost:4317"

##

[[deployment]] # Declare a deployment
name = "test-logger-01"
description = "test logger deployment 1"
tags = ["test"]
# sync will deploy the container: 
#  - if it is not running.
#  - has relevant config updates.
#  - the attached build has new version.
deploy = true
config.server_id = "server-01"
config.image.type = "Build"
config.image.params.build = "test_logger"
# set the volumes / bind mounts
config.volumes = """
/data/logs = /etc/logs
/data/config = /etc/config"""
# Set the environment variables
config.environment = """
OTLP_ENDPOINT = [[OTLP_ENDPOINT]] # interpolate variables into the envs. (they also support comments using '#')
VARIABLE_1 = value_1
VARIABLE_2 = value_2"""
# Set Docker labels
config.labels = "deployment.type = logger"

##

[[deployment]]
name = "test-logger-02"
description = "test logger deployment 2"
tags = ["test"]
deploy = true
# Create a dependency on test-logger-01. This deployment will only be deployed after test-logger-01 is deployed.
# Additionally, any sync deploy of test-logger-01 will also trigger sync deploy of this deployment.
after = ["test-logger-01"]
config.server_id = "server-01"
config.image.type = "Build"
config.image.params.build = "test_logger"
config.volumes = """
/data/logs = /etc/logs
/data/config = /etc/config"""
config.environment = """
VARIABLE_1 = value_1
VARIABLE_2 = value_2"""
# Set Docker labels
config.labels = "deployment.type = logger"
```

### Stack

- [Stack config schema](https://docs.rs/komodo_client/latest/komodo_client/entities/stack/struct.StackConfig.html)

```toml
[[stack]]
name = "test-stack"
description = "stack test"
deploy = true
after = ["test-logger-01"] # Stacks can depend on deployments, and vice versa.
tags = ["test"]
config.server_id = "server-prod"
config.file_paths = ["mongo.yaml", "redis.yaml"]
config.git_provider = "git.mogh.tech"
config.git_account = "mbecker20" # clone private repo by specifying account
config.repo = "mbecker20/stack_test"
```

### Procedure

- [Procedure config schema](https://docs.rs/komodo_client/latest/komodo_client/entities/procedure/struct.ProcedureConfig.html)

```toml
[[procedure]]
name = "test-procedure"
description = "Do some things in a specific order"
tags = ["test"]

# Each stage will be executed one after the other (in sequence)
[[procedure.config.stage]]
name = "Build stuff"
enabled = true
# The executions within a stage will be run in parallel. The stage completes when all executions finish.
executions = [
  { execution.type = "RunBuild", execution.params.build = "test_logger", enabled = true },
  { execution.type = "PullRepo", execution.params.repo = "komodo-periphery", enabled = true },
]

[[procedure.config.stage]]
name = "Deploy test logger 1"
enabled = true
executions = [
  { execution.type = "Deploy", execution.params.deployment = "test-logger-01", enabled = true }
]

[[procedure.config.stage]]
name = "Deploy test logger 2"
enabled = true
executions = [
  { execution.type = "Deploy", execution.params.deployment = "test-logger-02", enabled = true }
]
```

### Repo

- [Repo config schema](https://docs.rs/komodo_client/latest/komodo_client/entities/repo/struct.RepoConfig.html)

```toml
[[repo]]
name = "komodo-periphery"
description = "Builds new versions of the periphery binary. Requires Rust installed on the host."
tags = ["komodo"]
config.server_id = "server-01"
config.git_provider = "git.mogh.tech" # use an alternate git provider (default is github.com)
config.git_account = "mbecker20"
config.repo = "mbecker20/komodo"
# Run an action after the repo is pulled
config.on_pull.path = "."
config.on_pull.command = """
/root/.cargo/bin/cargo build -p komodo_periphery --release && \
cp ./target/release/periphery /root/periphery"""
```

### User Group:

- [UserGroup schema](https://docs.rs/komodo_client/latest/komodo_client/entities/toml/struct.UserGroupToml.html)

```toml
[[user_group]]
name = "groupo"
users = ["mbecker20", "karamvirsingh98"]
# Attach base level of Execute on all builds
all.Build = "Execute"
all.Alerter = "Write"
permissions = [
  # Attach permissions to specific resources by name
  { target.type = "Repo", target.id = "komodo-periphery", level = "Execute" },
  # Attach permissions to many resources with name matching regex (this uses '^(.+)-(.+)$' as regex expression)
  { target.type = "Server", target.id = "\\^(.+)-(.+)$\\", level = "Read" },
  { target.type = "Deployment", target.id = "\\^immich\\", level = "Execute" },
]
```