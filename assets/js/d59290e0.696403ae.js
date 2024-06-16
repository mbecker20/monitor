"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[383],{766:(e,n,o)=>{o.r(n),o.d(n,{assets:()=>l,contentTitle:()=>s,default:()=>h,frontMatter:()=>i,metadata:()=>c,toc:()=>a});var t=o(4848),r=o(8453);const i={},s="Core Setup",c={id:"core-setup",title:"Core Setup",description:"To run Monitor Core, you will need:",source:"@site/docs/core-setup.md",sourceDirName:".",slug:"/core-setup",permalink:"/docs/core-setup",draft:!1,unlisted:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/core-setup.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Resources",permalink:"/docs/resources"},next:{title:"Connecting Servers",permalink:"/docs/connecting-servers"}},l={},a=[{value:"Mongo",id:"mongo",level:2},{value:"1. Create the configuration file",id:"1-create-the-configuration-file",level:2},{value:"2. Start monitor core",id:"2-start-monitor-core",level:2},{value:"First login",id:"first-login",level:2},{value:"Tls",id:"tls",level:2}];function d(e){const n={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",li:"li",p:"p",pre:"pre",ul:"ul",...(0,r.R)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h1,{id:"core-setup",children:"Core Setup"}),"\n",(0,t.jsx)(n.p,{children:"To run Monitor Core, you will need:"}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsx)(n.li,{children:"A valid configuration file."}),"\n",(0,t.jsx)(n.li,{children:"An instance of MongoDB to which Core can connect."}),"\n",(0,t.jsxs)(n.li,{children:["Docker must be installed on the host. See ",(0,t.jsx)(n.a,{href:"https://docs.docker.com/engine/install/",children:"the install docs"}),"."]}),"\n"]}),"\n",(0,t.jsx)(n.h2,{id:"mongo",children:"Mongo"}),"\n",(0,t.jsx)(n.p,{children:"Mongo can be run locally using the docker cli:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-sh",children:'docker run --name monitor-mongo \\\n\t--network host \\\n\t-v /local/storage/path:/data/db \\\n\t-e MONGO_INITDB_ROOT_USERNAME="admin" \\\n\t-e MONGO_INITDB_ROOT_PASSWORD="admin" \\\n\tmongo:latest\n'})}),"\n",(0,t.jsxs)(n.p,{children:["You should replace the username and password with your own.\nSee ",(0,t.jsx)(n.a,{href:"https://hub.docker.com/_/mongo",children:"the image docs"})," for more details."]}),"\n",(0,t.jsxs)(n.admonition,{type:"note",children:[(0,t.jsx)(n.p,{children:"The disk space requirements of Monitor are dominated by the storage of system stats.\nThis depends on the number of connected servers (more system stats being produces / stored), stats collection frequency, and your stats pruning configuration.\nIf you need to save on space, you can configure these fields in your core config:"}),(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsxs)(n.li,{children:["Stats poll frequency can be reduced using, for example, ",(0,t.jsx)(n.code,{children:'monitoring_interval = "15-sec"'})]}),"\n",(0,t.jsxs)(n.li,{children:["Pruning can be tuned more aggresively using, for example, ",(0,t.jsx)(n.code,{children:"keep_stats_for_days = 7"}),"."]}),"\n"]})]}),"\n",(0,t.jsx)(n.h2,{id:"1-create-the-configuration-file",children:"1. Create the configuration file"}),"\n",(0,t.jsxs)(n.p,{children:["Create a configuration file on the system, for example at ",(0,t.jsx)(n.code,{children:"~/.config/monitor/core.config.toml"}),", and copy the ",(0,t.jsx)(n.a,{href:"https://github.com/mbecker20/monitor/blob/main/config_example/core.config.example.toml",children:"example config"}),". Fill in all the necessary information before continuing."]}),"\n",(0,t.jsx)(n.admonition,{type:"note",children:(0,t.jsxs)(n.p,{children:["To enable OAuth2 login, you must create a client on the respective OAuth provider,\nfor example ",(0,t.jsx)(n.a,{href:"https://developers.google.com/identity/protocols/oauth2",children:"google"}),"\nor ",(0,t.jsx)(n.a,{href:"https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps",children:"github"}),".\nMonitor uses the ",(0,t.jsx)(n.code,{children:"web application"})," login flow.\nThe redirect uri is ",(0,t.jsx)(n.code,{children:"<base_url>/auth/google/callback"})," for google and ",(0,t.jsx)(n.code,{children:"<base_url>/auth/github/callback"})," for github."]})}),"\n",(0,t.jsx)(n.admonition,{type:"note",children:(0,t.jsxs)(n.p,{children:["Most configuration can additionally be passed using environment variables, which override the value in the config file.\nSee ",(0,t.jsx)(n.a,{href:"https://docs.rs/monitor_client/latest/monitor_client/entities/config/core/index.html",children:"config docs"}),"."]})}),"\n",(0,t.jsx)(n.h2,{id:"2-start-monitor-core",children:"2. Start monitor core"}),"\n",(0,t.jsxs)(n.p,{children:["Monitor core is distributed via Github Container Registry under the package ",(0,t.jsx)(n.a,{href:"https://github.com/mbecker20/monitor/pkgs/container/monitor_core",children:"mbecker20/monitor_core"}),"."]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-sh",children:"docker run -d --name monitor-core \\\n\t--network host \\\n\t-v $HOME/.monitor/core.config.toml:/config/config.toml \\\n\tghcr.io/mbecker20/monitor_core\n"})}),"\n",(0,t.jsx)(n.h2,{id:"first-login",children:"First login"}),"\n",(0,t.jsxs)(n.p,{children:["Core should now be accessible on the specified port, so navigating to ",(0,t.jsx)(n.code,{children:"http://<address>:<port>"})," will display the login page."]}),"\n",(0,t.jsx)(n.p,{children:"The first user to log in will be auto enabled and made admin. any additional users to create accounts will be disabled by default."}),"\n",(0,t.jsx)(n.h2,{id:"tls",children:"Tls"}),"\n",(0,t.jsxs)(n.p,{children:["Core itself only supports http, so a reverse proxy like ",(0,t.jsx)(n.a,{href:"https://caddyserver.com/",children:"caddy"})," should be used for https."]})]})}function h(e={}){const{wrapper:n}={...(0,r.R)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(d,{...e})}):d(e)}},8453:(e,n,o)=>{o.d(n,{R:()=>s,x:()=>c});var t=o(6540);const r={},i=t.createContext(r);function s(e){const n=t.useContext(i);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function c(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:s(e.components),t.createElement(i.Provider,{value:n},e.children)}}}]);