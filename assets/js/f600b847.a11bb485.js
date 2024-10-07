"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[238],{2945:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>a,contentTitle:()=>l,default:()=>h,frontMatter:()=>i,metadata:()=>c,toc:()=>d});var o=n(4848),s=n(8453),r=n(5871);const i={},l="Setup Komodo",c={id:"setup/index",title:"Setup Komodo",description:"To run Komodo, you will need Docker. See the docker install docs.",source:"@site/docs/setup/index.mdx",sourceDirName:"setup",slug:"/setup/",permalink:"/docs/setup/",draft:!1,unlisted:!1,editUrl:"https://github.com/mbecker20/komodo/tree/main/docsite/docs/setup/index.mdx",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Resources",permalink:"/docs/resources"},next:{title:"MongoDB",permalink:"/docs/setup/mongo"}},a={},d=[{value:"Deploy with Docker Compose",id:"deploy-with-docker-compose",level:3},{value:"First login",id:"first-login",level:3},{value:"Https",id:"https",level:3}];function u(e){const t={a:"a",admonition:"admonition",code:"code",h1:"h1",h3:"h3",header:"header",li:"li",p:"p",ul:"ul",...(0,s.R)(),...e.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(t.header,{children:(0,o.jsx)(t.h1,{id:"setup-komodo",children:"Setup Komodo"})}),"\n",(0,o.jsxs)(t.p,{children:["To run Komodo, you will need Docker. See ",(0,o.jsx)(t.a,{href:"https://docs.docker.com/engine/install/",children:"the docker install docs"}),"."]}),"\n",(0,o.jsx)(t.h3,{id:"deploy-with-docker-compose",children:"Deploy with Docker Compose"}),"\n",(0,o.jsxs)(t.ul,{children:["\n",(0,o.jsx)(t.li,{children:(0,o.jsx)(t.a,{href:"/docs/setup/mongo",children:"Using MongoDB"})}),"\n",(0,o.jsx)(t.li,{children:(0,o.jsx)(t.a,{href:"/docs/setup/postgres",children:"Using Postgres"})}),"\n",(0,o.jsx)(t.li,{children:(0,o.jsx)(t.a,{href:"/docs/setup/sqlite",children:"Using Sqlite"})}),"\n"]}),"\n",(0,o.jsx)(t.admonition,{type:"info",children:(0,o.jsxs)(t.p,{children:["Komodo is able to support Postgres and Sqlite by utilizing the ",(0,o.jsx)(t.a,{href:"https://www.ferretdb.com/",children:"FerretDB Mongo Adapter"}),"."]})}),"\n",(0,o.jsx)(t.h3,{id:"first-login",children:"First login"}),"\n",(0,o.jsxs)(t.p,{children:["Core should now be accessible on the specified port, so navigating to ",(0,o.jsx)(t.code,{children:"http://<address>:<port>"})," will display the login page."]}),"\n",(0,o.jsx)(t.p,{children:"The first user to log in will be auto enabled and made an admin. Any additional users to create accounts will be disabled by default, and must be enabled by an admin."}),"\n",(0,o.jsx)(t.h3,{id:"https",children:"Https"}),"\n",(0,o.jsxs)(t.p,{children:["Komodo Core only supports http, so a reverse proxy like ",(0,o.jsx)(t.a,{href:"https://caddyserver.com/",children:"caddy"})," should be used for https."]}),"\n","\n",(0,o.jsx)(r.A,{})]})}function h(e={}){const{wrapper:t}={...(0,s.R)(),...e.components};return t?(0,o.jsx)(t,{...e,children:(0,o.jsx)(u,{...e})}):u(e)}},5871:(e,t,n)=>{n.d(t,{A:()=>D});var o=n(6540),s=n(4164),r=n(6972),i=n(8774),l=n(4586);const c=["zero","one","two","few","many","other"];function a(e){return c.filter((t=>e.includes(t)))}const d={locale:"en",pluralForms:a(["one","other"]),select:e=>1===e?"one":"other"};function u(){const{i18n:{currentLocale:e}}=(0,l.A)();return(0,o.useMemo)((()=>{try{return function(e){const t=new Intl.PluralRules(e);return{locale:e,pluralForms:a(t.resolvedOptions().pluralCategories),select:e=>t.select(e)}}(e)}catch(t){return console.error(`Failed to use Intl.PluralRules for locale "${e}".\nDocusaurus will fallback to the default (English) implementation.\nError: ${t.message}\n`),d}}),[e])}function h(){const e=u();return{selectMessage:(t,n)=>function(e,t,n){const o=e.split("|");if(1===o.length)return o[0];o.length>n.pluralForms.length&&console.error(`For locale=${n.locale}, a maximum of ${n.pluralForms.length} plural forms are expected (${n.pluralForms.join(",")}), but the message contains ${o.length}: ${e}`);const s=n.select(t),r=n.pluralForms.indexOf(s);return o[Math.min(r,o.length-1)]}(n,t,e)}}var p=n(6654),m=n(1312),f=n(1107);const x={cardContainer:"cardContainer_fWXF",cardTitle:"cardTitle_rnsV",cardDescription:"cardDescription_PWke"};var g=n(4848);function j(e){let{href:t,children:n}=e;return(0,g.jsx)(i.A,{href:t,className:(0,s.A)("card padding--lg",x.cardContainer),children:n})}function y(e){let{href:t,icon:n,title:o,description:r}=e;return(0,g.jsxs)(j,{href:t,children:[(0,g.jsxs)(f.A,{as:"h2",className:(0,s.A)("text--truncate",x.cardTitle),title:o,children:[n," ",o]}),r&&(0,g.jsx)("p",{className:(0,s.A)("text--truncate",x.cardDescription),title:r,children:r})]})}function b(e){let{item:t}=e;const n=(0,r.Nr)(t),o=function(){const{selectMessage:e}=h();return t=>e(t,(0,m.T)({message:"1 item|{count} items",id:"theme.docs.DocCard.categoryDescription.plurals",description:"The default description for a category card in the generated index about how many items this category includes"},{count:t}))}();return n?(0,g.jsx)(y,{href:n,icon:"\ud83d\uddc3\ufe0f",title:t.label,description:t.description??o(t.items.length)}):null}function w(e){let{item:t}=e;const n=(0,p.A)(t.href)?"\ud83d\udcc4\ufe0f":"\ud83d\udd17",o=(0,r.cC)(t.docId??void 0);return(0,g.jsx)(y,{href:t.href,icon:n,title:t.label,description:t.description??o?.description})}function k(e){let{item:t}=e;switch(t.type){case"link":return(0,g.jsx)(w,{item:t});case"category":return(0,g.jsx)(b,{item:t});default:throw new Error(`unknown item type ${JSON.stringify(t)}`)}}function v(e){let{className:t}=e;const n=(0,r.$S)();return(0,g.jsx)(D,{items:n.items,className:t})}function D(e){const{items:t,className:n}=e;if(!t)return(0,g.jsx)(v,{...e});const o=(0,r.d1)(t);return(0,g.jsx)("section",{className:(0,s.A)("row",n),children:o.map(((e,t)=>(0,g.jsx)("article",{className:"col col--6 margin-bottom--lg",children:(0,g.jsx)(k,{item:e})},t)))})}},8453:(e,t,n)=>{n.d(t,{R:()=>i,x:()=>l});var o=n(6540);const s={},r=o.createContext(s);function i(e){const t=o.useContext(r);return o.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function l(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:i(e.components),o.createElement(r.Provider,{value:t},e.children)}}}]);