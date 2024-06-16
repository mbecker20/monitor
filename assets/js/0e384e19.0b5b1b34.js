"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[976],{619:(e,n,r)=>{r.r(n),r.d(n,{assets:()=>c,contentTitle:()=>i,default:()=>h,frontMatter:()=>s,metadata:()=>a,toc:()=>d});var t=r(4848),o=r(8453);const s={slug:"/intro"},i="What is Monitor?",a={id:"intro",title:"What is Monitor?",description:"Monitor is a web app to provide structure for managing your servers, builds, deployments, and automated procedures.",source:"@site/docs/intro.md",sourceDirName:".",slug:"/intro",permalink:"/docs/intro",draft:!1,unlisted:!1,editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs/intro.md",tags:[],version:"current",frontMatter:{slug:"/intro"},sidebar:"docs",next:{title:"Resources",permalink:"/docs/resources"}},c={},d=[{value:"Docker",id:"docker",level:2},{value:"Architecture and Components",id:"architecture-and-components",level:2},{value:"Core",id:"core",level:3},{value:"Periphery",id:"periphery",level:3},{value:"Core API",id:"core-api",level:2},{value:"Permissioning",id:"permissioning",level:2}];function l(e){const n={a:"a",h1:"h1",h2:"h2",h3:"h3",li:"li",ol:"ol",p:"p",...(0,o.R)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h1,{id:"what-is-monitor",children:"What is Monitor?"}),"\n",(0,t.jsx)(n.p,{children:"Monitor is a web app to provide structure for managing your servers, builds, deployments, and automated procedures."}),"\n",(0,t.jsx)(n.p,{children:"With Monitor you can:"}),"\n",(0,t.jsxs)(n.ol,{children:["\n",(0,t.jsx)(n.li,{children:"Connect all of your remote servers (across clouds + on premise) to be viewable from a unified 'monitor' (hence the name)."}),"\n",(0,t.jsx)(n.li,{children:"Build application source into auto-versioned Docker images."}),"\n",(0,t.jsx)(n.li,{children:"Create, start, stop, and restart Docker containers on the connected servers, and view their status and logs."}),"\n",(0,t.jsx)(n.li,{children:"Manage repositories on connected servers."}),"\n",(0,t.jsx)(n.li,{children:"Keep a record of all the actions that are performed and by whom."}),"\n"]}),"\n",(0,t.jsx)(n.h2,{id:"docker",children:"Docker"}),"\n",(0,t.jsxs)(n.p,{children:["Monitor is opinionated by design, and uses ",(0,t.jsx)(n.a,{href:"https://docs.docker.com/",children:"docker"})," as the container engine for building and deploying."]}),"\n",(0,t.jsx)(n.h2,{id:"architecture-and-components",children:"Architecture and Components"}),"\n",(0,t.jsx)(n.p,{children:"Monitor is composed of a single core and any amount of connected servers running the periphery application."}),"\n",(0,t.jsx)(n.h3,{id:"core",children:"Core"}),"\n",(0,t.jsx)(n.p,{children:"Monitor Core is a web server hosting the Core API and browser UI. All user interaction with the connected servers flow through the Core. It is the stateful part of the system, with the application state stored on an instance of MongoDB."}),"\n",(0,t.jsx)(n.h3,{id:"periphery",children:"Periphery"}),"\n",(0,t.jsx)(n.p,{children:"Monitor Periphery is a small stateless web server that runs on all connected servers. It exposes an API called by Monitor Core to perform actions on the server, get system usage, and container status / logs. It is only intended to be reached from the core, and has an address whitelist to limit the IPs allowed to call this API."}),"\n",(0,t.jsx)(n.h2,{id:"core-api",children:"Core API"}),"\n",(0,t.jsxs)(n.p,{children:["Monitor exposes powerful functionality over the Core's REST and Websocket API, enabling infrastructure engineers to manage their infrastructure programmatically. There is a ",(0,t.jsx)(n.a,{href:"https://crates.io/crates/monitor_client",children:"rust crate"})," to simplify programmatic interaction with the API, but in general this can be accomplished using any programming language that can make REST requests."]}),"\n",(0,t.jsx)(n.h2,{id:"permissioning",children:"Permissioning"}),"\n",(0,t.jsxs)(n.p,{children:["Monitor is a system designed to be used by many users, whether they are developers, operations personnel, or administrators. The ability to affect an applications state is very powerful, so monitor has a granular permissioning system to only provide this functionality to the intended users. The permissioning system is explained in detail in the ",(0,t.jsx)(n.a,{href:"/docs/permissioning",children:"permissioning"})," section."]}),"\n",(0,t.jsxs)(n.p,{children:["User sign-on is possible using username / password, or with Oauth (Github and Google). See ",(0,t.jsx)(n.a,{href:"/docs/core-setup",children:"Core Setup"}),"."]})]})}function h(e={}){const{wrapper:n}={...(0,o.R)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(l,{...e})}):l(e)}},8453:(e,n,r)=>{r.d(n,{R:()=>i,x:()=>a});var t=r(6540);const o={},s=t.createContext(o);function i(e){const n=t.useContext(s);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function a(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(o):e.components||o:i(e.components),t.createElement(s.Provider,{value:n},e.children)}}}]);