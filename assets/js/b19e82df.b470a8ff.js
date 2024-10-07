"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[842],{8929:(e,t,o)=>{o.r(t),o.d(t,{assets:()=>c,contentTitle:()=>l,default:()=>m,frontMatter:()=>s,metadata:()=>u,toc:()=>i});var n=o(4848),r=o(8453),a=o(5627);const s={},l="MongoDB",u={id:"setup/mongo",title:"MongoDB",description:"1. Copy komodo/mongo.compose.yaml and komodo/compose.env to your host:",source:"@site/docs/setup/mongo.mdx",sourceDirName:"setup",slug:"/setup/mongo",permalink:"/docs/setup/mongo",draft:!1,unlisted:!1,editUrl:"https://github.com/mbecker20/komodo/tree/main/docsite/docs/setup/mongo.mdx",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Setup Komodo",permalink:"/docs/setup/"},next:{title:"Postgres",permalink:"/docs/setup/postgres"}},c={},i=[];function d(e){const t={code:"code",h1:"h1",header:"header",li:"li",ol:"ol",pre:"pre",...(0,r.R)(),...e.components};return(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)(t.header,{children:(0,n.jsx)(t.h1,{id:"mongodb",children:"MongoDB"})}),"\n",(0,n.jsxs)(t.ol,{children:["\n",(0,n.jsxs)(t.li,{children:["Copy ",(0,n.jsx)(t.code,{children:"komodo/mongo.compose.yaml"})," and ",(0,n.jsx)(t.code,{children:"komodo/compose.env"})," to your host:"]}),"\n"]}),"\n",(0,n.jsx)(t.pre,{children:(0,n.jsx)(t.code,{className:"language-bash",children:"wget -P komodo https://raw.githubusercontent.com/mbecker20/komodo/main/compose/mongo.compose.yaml && \\\n  wget -P komodo https://raw.githubusercontent.com/mbecker20/komodo/main/compose/compose.env\n"})}),"\n",(0,n.jsxs)(t.ol,{start:"2",children:["\n",(0,n.jsxs)(t.li,{children:["Edit the variables in ",(0,n.jsx)(t.code,{children:"komodo/compose.env"}),"."]}),"\n",(0,n.jsx)(t.li,{children:"Deploy:"}),"\n"]}),"\n",(0,n.jsx)(t.pre,{children:(0,n.jsx)(t.code,{className:"language-bash",children:"docker compose -p komodo -f komodo/mongo.compose.yaml --env-file komodo/compose.env up -d\n"})}),"\n","\n",(0,n.jsx)(a.A,{file_name:"mongo.compose.yaml"})]})}function m(e={}){const{wrapper:t}={...(0,r.R)(),...e.components};return t?(0,n.jsx)(t,{...e,children:(0,n.jsx)(d,{...e})}):d(e)}},5627:(e,t,o)=>{o.d(t,{A:()=>A});var n=o(6540),r=o(6695),a=o(4164),s=o(3104),l=o(6347),u=o(205),c=o(7485),i=o(1682),d=o(679);function m(e){return n.Children.toArray(e).filter((e=>"\n"!==e)).map((e=>{if(!e||(0,n.isValidElement)(e)&&function(e){const{props:t}=e;return!!t&&"object"==typeof t&&"value"in t}(e))return e;throw new Error(`Docusaurus error: Bad <Tabs> child <${"string"==typeof e.type?e.type:e.type.name}>: all children of the <Tabs> component should be <TabItem>, and every <TabItem> should have a unique "value" prop.`)}))?.filter(Boolean)??[]}function p(e){const{values:t,children:o}=e;return(0,n.useMemo)((()=>{const e=t??function(e){return m(e).map((e=>{let{props:{value:t,label:o,attributes:n,default:r}}=e;return{value:t,label:o,attributes:n,default:r}}))}(o);return function(e){const t=(0,i.XI)(e,((e,t)=>e.value===t.value));if(t.length>0)throw new Error(`Docusaurus error: Duplicate values "${t.map((e=>e.value)).join(", ")}" found in <Tabs>. Every value needs to be unique.`)}(e),e}),[t,o])}function h(e){let{value:t,tabValues:o}=e;return o.some((e=>e.value===t))}function b(e){let{queryString:t=!1,groupId:o}=e;const r=(0,l.W6)(),a=function(e){let{queryString:t=!1,groupId:o}=e;if("string"==typeof t)return t;if(!1===t)return null;if(!0===t&&!o)throw new Error('Docusaurus error: The <Tabs> component groupId prop is required if queryString=true, because this value is used as the search param name. You can also provide an explicit value such as queryString="my-search-param".');return o??null}({queryString:t,groupId:o});return[(0,c.aZ)(a),(0,n.useCallback)((e=>{if(!a)return;const t=new URLSearchParams(r.location.search);t.set(a,e),r.replace({...r.location,search:t.toString()})}),[a,r])]}function f(e){const{defaultValue:t,queryString:o=!1,groupId:r}=e,a=p(e),[s,l]=(0,n.useState)((()=>function(e){let{defaultValue:t,tabValues:o}=e;if(0===o.length)throw new Error("Docusaurus error: the <Tabs> component requires at least one <TabItem> children component");if(t){if(!h({value:t,tabValues:o}))throw new Error(`Docusaurus error: The <Tabs> has a defaultValue "${t}" but none of its children has the corresponding value. Available values are: ${o.map((e=>e.value)).join(", ")}. If you intend to show no default tab, use defaultValue={null} instead.`);return t}const n=o.find((e=>e.default))??o[0];if(!n)throw new Error("Unexpected error: 0 tabValues");return n.value}({defaultValue:t,tabValues:a}))),[c,i]=b({queryString:o,groupId:r}),[m,f]=function(e){let{groupId:t}=e;const o=function(e){return e?`docusaurus.tab.${e}`:null}(t),[r,a]=(0,d.Dv)(o);return[r,(0,n.useCallback)((e=>{o&&a.set(e)}),[o,a])]}({groupId:r}),g=(()=>{const e=c??m;return h({value:e,tabValues:a})?e:null})();(0,u.A)((()=>{g&&l(g)}),[g]);return{selectedValue:s,selectValue:(0,n.useCallback)((e=>{if(!h({value:e,tabValues:a}))throw new Error(`Can't select invalid tab value=${e}`);l(e),i(e),f(e)}),[i,f,a]),tabValues:a}}var g=o(2303);const v={tabList:"tabList__CuJ",tabItem:"tabItem_LNqP"};var k=o(4848);function x(e){let{className:t,block:o,selectedValue:n,selectValue:r,tabValues:l}=e;const u=[],{blockElementScrollPositionUntilNextRender:c}=(0,s.a_)(),i=e=>{const t=e.currentTarget,o=u.indexOf(t),a=l[o].value;a!==n&&(c(t),r(a))},d=e=>{let t=null;switch(e.key){case"Enter":i(e);break;case"ArrowRight":{const o=u.indexOf(e.currentTarget)+1;t=u[o]??u[0];break}case"ArrowLeft":{const o=u.indexOf(e.currentTarget)-1;t=u[o]??u[u.length-1];break}}t?.focus()};return(0,k.jsx)("ul",{role:"tablist","aria-orientation":"horizontal",className:(0,a.A)("tabs",{"tabs--block":o},t),children:l.map((e=>{let{value:t,label:o,attributes:r}=e;return(0,k.jsx)("li",{role:"tab",tabIndex:n===t?0:-1,"aria-selected":n===t,ref:e=>u.push(e),onKeyDown:d,onClick:i,...r,className:(0,a.A)("tabs__item",v.tabItem,r?.className,{"tabs__item--active":n===t}),children:o??t},t)}))})}function y(e){let{lazy:t,children:o,selectedValue:r}=e;const s=(Array.isArray(o)?o:[o]).filter(Boolean);if(t){const e=s.find((e=>e.props.value===r));return e?(0,n.cloneElement)(e,{className:(0,a.A)("margin-top--md",e.props.className)}):null}return(0,k.jsx)("div",{className:"margin-top--md",children:s.map(((e,t)=>(0,n.cloneElement)(e,{key:t,hidden:e.props.value!==r})))})}function j(e){const t=f(e);return(0,k.jsxs)("div",{className:(0,a.A)("tabs-container",v.tabList),children:[(0,k.jsx)(x,{...t,...e}),(0,k.jsx)(y,{...t,...e})]})}function w(e){const t=(0,g.A)();return(0,k.jsx)(j,{...e,children:m(e.children)},String(t))}const I={tabItem:"tabItem_Ymn6"};function V(e){let{children:t,hidden:o,className:n}=e;return(0,k.jsx)("div",{role:"tabpanel",className:(0,a.A)(I.tabItem,n),hidden:o,children:t})}function A(e){let{file_name:t}=e;return(0,k.jsxs)(w,{children:[(0,k.jsx)(V,{value:t,children:(0,k.jsx)(r.A,{title:`https://github.com/mbecker20/komodo/blob/main/compose/${t}`,url:`https://raw.githubusercontent.com/mbecker20/komodo/main/compose/${t}`,language:"yaml"})}),(0,k.jsx)(V,{value:"compose.env",children:(0,k.jsx)(r.A,{title:"https://github.com/mbecker20/komodo/blob/main/compose/compose.env",url:"https://raw.githubusercontent.com/mbecker20/komodo/main/compose/compose.env",language:"bash"})})]})}},6695:(e,t,o)=>{o.d(t,{A:()=>s});var n=o(6540),r=o(1432),a=o(4848);function s(e){let{url:t,language:o,title:s}=e;const[l,u]=(0,n.useState)("");return(0,n.useEffect)((()=>{!async function(e,t){const o=await fetch(e);t(await o.text())}(t,u)}),[]),(0,a.jsx)(r.A,{title:s??t,language:o,showLineNumbers:!0,children:l})}}}]);