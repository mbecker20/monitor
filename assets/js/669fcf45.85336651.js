"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[552],{7478:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>a,contentTitle:()=>s,default:()=>u,frontMatter:()=>o,metadata:()=>l,toc:()=>c});var r=t(4848),i=t(8453);const o={},s="Sync Resources",l={id:"sync-resources",title:"Sync Resources",description:"Monitor is able to create, update, delete, and deploy resources declared in TOML files by diffing them against the existing resources,",source:"@site/docs/sync-resources.md",sourceDirName:".",slug:"/sync-resources",permalink:"/docs/sync-resources",draft:!1,unlisted:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/sync-resources.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Container Management",permalink:"/docs/deploy-containers/lifetime-management"},next:{title:"Permissioning Resources",permalink:"/docs/permissioning"}},a={},c=[{value:"Example Declarations",id:"example-declarations",level:2},{value:"Server",id:"server",level:3},{value:"Builder and build",id:"builder-and-build",level:3},{value:"Deployments",id:"deployments",level:3},{value:"Stack",id:"stack",level:3},{value:"Procedure",id:"procedure",level:3},{value:"Repo",id:"repo",level:3},{value:"User Group:",id:"user-group",level:3}];function d(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,i.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(n.h1,{id:"sync-resources",children:"Sync Resources"}),"\n",(0,r.jsxs)(n.p,{children:["Monitor is able to create, update, delete, and deploy resources declared in TOML files by diffing them against the existing resources,\nand apply updates based on the diffs. Push the files to a remote git repo and create a ",(0,r.jsx)(n.code,{children:"ResourceSync"})," pointing to the repo,\nand the core backend will poll for any updates (you can also manually trigger an update poll / execution in the UI)."]}),"\n",(0,r.jsxs)(n.p,{children:["File detection is additive and recursive, so you can spread out your resource declarations across any number of files\nand use any nesting of folders to organize resources inside a root folder. Additionally, you can create multiple ",(0,r.jsx)(n.code,{children:"ResourceSyncs"}),'\nand each sync will be handled independently. This allows different syncs to manage resources on a "per-project" basis.']}),"\n",(0,r.jsx)(n.p,{children:"The UI will display the computed sync actions and only execute them upon manual confirmation.\nOr the sync execution git webhook may be configured on the git repo to\nautomatically execute syncs upon pushes to the configured branch."}),"\n",(0,r.jsx)(n.h2,{id:"example-declarations",children:"Example Declarations"}),"\n",(0,r.jsx)(n.h3,{id:"server",children:"Server"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/server/struct.ServerConfig.html",children:"Server config schema"})}),"\n"]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[[server]] # Declare a new server\nname = "server-01"\ndescription = "the main mogh server"\ntags = ["monitor"]\nconfig.address = "http://localhost:8120"\nconfig.region = "AshburnDc1"\nconfig.enabled = true # default: false\n'})}),"\n",(0,r.jsx)(n.h3,{id:"builder-and-build",children:"Builder and build"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/builder/struct.BuilderConfig.html",children:"Builder config schema"})}),"\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/build/struct.BuildConfig.html",children:"Build config schema"})}),"\n"]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[[builder]] # Declare a builder\nname = "builder-01"\ntags = []\nconfig.type = "Aws"\nconfig.params.region = "us-east-2"\nconfig.params.ami_id = "ami-0e9bd154667944680"\n# These things come from your specific setup\nconfig.params.subnet_id = "subnet-xxxxxxxxxxxxxxxxxx"\nconfig.params.key_pair_name = "xxxxxxxx"\nconfig.params.assign_public_ip = true\nconfig.params.use_public_ip = true\nconfig.params.security_group_ids = [\n  "sg-xxxxxxxxxxxxxxxxxx",\n  "sg-xxxxxxxxxxxxxxxxxx"\n]\n\n##\n\n[[build]]\nname = "test_logger"\ndescription = "Logs randomly at INFO, WARN, ERROR levels to test logging setups"\ntags = ["test"]\nconfig.builder_id = "builder-01"\nconfig.repo = "mbecker20/test_logger"\nconfig.branch = "master"\nconfig.git_account = "mbecker20"\nconfig.image_registry.type = "Standard"\nconfig.image_registry.params.domain = "github.com" # or your custom domain\nconfig.image_registry.params.account = "your_username"\nconfig.image_registry.params.organization = "your_organization" # optinoal\n# Set docker labels\nconfig.labels = """\norg.opencontainers.image.source = https://github.com/mbecker20/test_logger\norg.opencontainers.image.description = Logs randomly at INFO, WARN, ERROR levels to test logging setups\norg.opencontainers.image.licenses = GPL-3.0"""\n'})}),"\n",(0,r.jsx)(n.h3,{id:"deployments",children:"Deployments"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/deployment/struct.DeploymentConfig.html",children:"Deployment config schema"})}),"\n"]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[[variable]] # Declare variables\nname = "OTLP_ENDPOINT"\nvalue = "http://localhost:4317"\n\n##\n\n[[deployment]] # Declare a deployment\nname = "test-logger-01"\ndescription = "test logger deployment 1"\ntags = ["test"]\n# sync will deploy the container: \n#  - if it is not running.\n#  - has relevant config updates.\n#  - the attached build has new version.\ndeploy = true\nconfig.server_id = "server-01"\nconfig.image.type = "Build"\nconfig.image.params.build = "test_logger"\n# set the volumes / bind mounts\nconfig.volumes = """\n/data/logs = /etc/logs\n/data/config = /etc/config"""\n# Set the environment variables\nconfig.environment = """\nOTLP_ENDPOINT = [[OTLP_ENDPOINT]] # interpolate variables into the envs. (they also support comments using \'#\')\nVARIABLE_1 = value_1\nVARIABLE_2 = value_2"""\n# Set Docker labels\nconfig.labels = "deployment.type = logger"\n\n##\n\n[[deployment]]\nname = "test-logger-02"\ndescription = "test logger deployment 2"\ntags = ["test"]\ndeploy = true\n# Create a dependency on test-logger-01. This deployment will only be deployed after test-logger-01 is deployed.\n# Additionally, any sync deploy of test-logger-01 will also trigger sync deploy of this deployment.\nafter = ["test-logger-01"]\nconfig.server_id = "server-01"\nconfig.image.type = "Build"\nconfig.image.params.build = "test_logger"\nconfig.volumes = """\n/data/logs = /etc/logs\n/data/config = /etc/config"""\nconfig.environment = """\nVARIABLE_1 = value_1\nVARIABLE_2 = value_2"""\n# Set Docker labels\nconfig.labels = "deployment.type = logger"\n'})}),"\n",(0,r.jsx)(n.h3,{id:"stack",children:"Stack"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/stack/struct.StackConfig.html",children:"Stack config schema"})}),"\n"]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[[stack]]\nname = "test-stack"\ndescription = "stack test"\ndeploy = true\nafter = ["test-logger-01"] # Stacks can depend on deployments, and vice versa.\ntags = ["test"]\nconfig.server_id = "monitor-01"\nconfig.file_paths = ["mongo.yaml", "redis.yaml"]\nconfig.git_provider = "git.mogh.tech"\nconfig.git_account = "mbecker20" # clone private repo by specifying account\nconfig.repo = "mbecker20/stack_test"\n'})}),"\n",(0,r.jsx)(n.h3,{id:"procedure",children:"Procedure"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/procedure/struct.ProcedureConfig.html",children:"Procedure config schema"})}),"\n"]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[[procedure]]\nname = "test-procedure"\ndescription = "Do some things in a specific order"\ntags = ["test"]\n\n# Each stage will be executed one after the other (in sequence)\n[[procedure.config.stage]]\nname = "Build stuff"\nenabled = true\n# The executions within a stage will be run in parallel. The stage completes when all executions finish.\nexecutions = [\n  { execution.type = "RunBuild", execution.params.build = "test_logger", enabled = true },\n  { execution.type = "PullRepo", execution.params.repo = "monitor-periphery", enabled = true },\n]\n\n[[procedure.config.stage]]\nname = "Deploy test logger 1"\nenabled = true\nexecutions = [\n  { execution.type = "Deploy", execution.params.deployment = "test-logger-01", enabled = true }\n]\n\n[[procedure.config.stage]]\nname = "Deploy test logger 2"\nenabled = true\nexecutions = [\n  { execution.type = "Deploy", execution.params.deployment = "test-logger-02", enabled = true }\n]\n'})}),"\n",(0,r.jsx)(n.h3,{id:"repo",children:"Repo"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/repo/struct.RepoConfig.html",children:"Repo config schema"})}),"\n"]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[[repo]]\nname = "monitor-periphery"\ndescription = "Builds new versions of the periphery binary. Requires Rust installed on the host."\ntags = ["monitor"]\nconfig.server_id = "server-01"\nconfig.git_provider = "git.mogh.tech" # use an alternate git provider (default is github.com)\nconfig.git_account = "mbecker20"\nconfig.repo = "mbecker20/monitor"\n# Run an action after the repo is pulled\nconfig.on_pull.path = "."\nconfig.on_pull.command = "/root/.cargo/bin/cargo build -p monitor_periphery --release && cp ./target/release/periphery /root/periphery"\n'})}),"\n",(0,r.jsx)(n.h3,{id:"user-group",children:"User Group:"}),"\n",(0,r.jsxs)(n.ul,{children:["\n",(0,r.jsx)(n.li,{children:(0,r.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/toml/struct.UserGroupToml.html",children:"UserGroup schema"})}),"\n"]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{className:"language-toml",children:'[[user_group]]\nname = "groupo"\nusers = ["mbecker20", "karamvirsingh98"]\n# Attach base level of Execute on all builds\nall.Build = "Execute"\nall.Alerter = "Write"\npermissions = [\n  # Attach permissions to specific resources by name\n  { target.type = "Repo", target.id = "monitor-periphery", level = "Execute" },\n  # Attach permissions to many resources with name matching regex (this uses \'^(.+)-(.+)$\' as regex expression)\n  { target.type = "Server", target.id = "\\\\^(.+)-(.+)$\\\\", level = "Read" },\n  { target.type = "Deployment", target.id = "\\\\^immich\\\\", level = "Execute" },\n]\n'})})]})}function u(e={}){const{wrapper:n}={...(0,i.R)(),...e.components};return n?(0,r.jsx)(n,{...e,children:(0,r.jsx)(d,{...e})}):d(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>s,x:()=>l});var r=t(6540);const i={},o=r.createContext(i);function s(e){const n=r.useContext(o);return r.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function l(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:s(e.components),r.createElement(o.Provider,{value:n},e.children)}}}]);