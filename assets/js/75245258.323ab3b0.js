"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[531],{4016:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>m,frontMatter:()=>c,metadata:()=>a,toc:()=>d});var o=n(4848),r=n(8453),s=n(5871);const c={},i="Deploy Containers",a={id:"deploy-containers/index",title:"Deploy Containers",description:"Komodo can deploy any docker images that it can access with the configured docker accounts.",source:"@site/docs/deploy-containers/index.mdx",sourceDirName:"deploy-containers",slug:"/deploy-containers/",permalink:"/docs/deploy-containers/",draft:!1,unlisted:!1,editUrl:"https://github.com/mbecker20/komodo/tree/main/docsite/docs/deploy-containers/index.mdx",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Image Versioning",permalink:"/docs/build-images/versioning"},next:{title:"Configuration",permalink:"/docs/deploy-containers/configuration"}},l={},d=[];function u(e){const t={code:"code",h1:"h1",header:"header",p:"p",...(0,r.R)(),...e.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(t.header,{children:(0,o.jsx)(t.h1,{id:"deploy-containers",children:"Deploy Containers"})}),"\n",(0,o.jsxs)(t.p,{children:["Komodo can deploy any docker images that it can access with the configured docker accounts.\nIt works by parsing the deployment configuration into a ",(0,o.jsx)(t.code,{children:"docker run"})," command, which is then run on the target system.\nThe configuration is stored on MongoDB, and records of all actions (update config, deploy, stop, etc.) are stored as well."]}),"\n","\n",(0,o.jsx)(s.A,{})]})}function m(e={}){const{wrapper:t}={...(0,r.R)(),...e.components};return t?(0,o.jsx)(t,{...e,children:(0,o.jsx)(u,{...e})}):u(e)}},5871:(e,t,n)=>{n.d(t,{A:()=>v});var o=n(6540),r=n(4164),s=n(6972),c=n(8774),i=n(4586);const a=["zero","one","two","few","many","other"];function l(e){return a.filter((t=>e.includes(t)))}const d={locale:"en",pluralForms:l(["one","other"]),select:e=>1===e?"one":"other"};function u(){const{i18n:{currentLocale:e}}=(0,i.A)();return(0,o.useMemo)((()=>{try{return function(e){const t=new Intl.PluralRules(e);return{locale:e,pluralForms:l(t.resolvedOptions().pluralCategories),select:e=>t.select(e)}}(e)}catch(t){return console.error(`Failed to use Intl.PluralRules for locale "${e}".\nDocusaurus will fallback to the default (English) implementation.\nError: ${t.message}\n`),d}}),[e])}function m(){const e=u();return{selectMessage:(t,n)=>function(e,t,n){const o=e.split("|");if(1===o.length)return o[0];o.length>n.pluralForms.length&&console.error(`For locale=${n.locale}, a maximum of ${n.pluralForms.length} plural forms are expected (${n.pluralForms.join(",")}), but the message contains ${o.length}: ${e}`);const r=n.select(t),s=n.pluralForms.indexOf(r);return o[Math.min(s,o.length-1)]}(n,t,e)}}var p=n(6654),h=n(1312),f=n(1107);const g={cardContainer:"cardContainer_fWXF",cardTitle:"cardTitle_rnsV",cardDescription:"cardDescription_PWke"};var x=n(4848);function y(e){let{href:t,children:n}=e;return(0,x.jsx)(c.A,{href:t,className:(0,r.A)("card padding--lg",g.cardContainer),children:n})}function j(e){let{href:t,icon:n,title:o,description:s}=e;return(0,x.jsxs)(y,{href:t,children:[(0,x.jsxs)(f.A,{as:"h2",className:(0,r.A)("text--truncate",g.cardTitle),title:o,children:[n," ",o]}),s&&(0,x.jsx)("p",{className:(0,r.A)("text--truncate",g.cardDescription),title:s,children:s})]})}function k(e){let{item:t}=e;const n=(0,s.Nr)(t),o=function(){const{selectMessage:e}=m();return t=>e(t,(0,h.T)({message:"1 item|{count} items",id:"theme.docs.DocCard.categoryDescription.plurals",description:"The default description for a category card in the generated index about how many items this category includes"},{count:t}))}();return n?(0,x.jsx)(j,{href:n,icon:"\ud83d\uddc3\ufe0f",title:t.label,description:t.description??o(t.items.length)}):null}function w(e){let{item:t}=e;const n=(0,p.A)(t.href)?"\ud83d\udcc4\ufe0f":"\ud83d\udd17",o=(0,s.cC)(t.docId??void 0);return(0,x.jsx)(j,{href:t.href,icon:n,title:t.label,description:t.description??o?.description})}function C(e){let{item:t}=e;switch(t.type){case"link":return(0,x.jsx)(w,{item:t});case"category":return(0,x.jsx)(k,{item:t});default:throw new Error(`unknown item type ${JSON.stringify(t)}`)}}function b(e){let{className:t}=e;const n=(0,s.$S)();return(0,x.jsx)(v,{items:n.items,className:t})}function v(e){const{items:t,className:n}=e;if(!t)return(0,x.jsx)(b,{...e});const o=(0,s.d1)(t);return(0,x.jsx)("section",{className:(0,r.A)("row",n),children:o.map(((e,t)=>(0,x.jsx)("article",{className:"col col--6 margin-bottom--lg",children:(0,x.jsx)(C,{item:e})},t)))})}},8453:(e,t,n)=>{n.d(t,{R:()=>c,x:()=>i});var o=n(6540);const r={},s=o.createContext(r);function c(e){const t=o.useContext(s);return o.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function i(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:c(e.components),o.createElement(s.Provider,{value:t},e.children)}}}]);