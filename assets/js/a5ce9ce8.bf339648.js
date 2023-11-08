"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[953],{3905:(e,t,r)=>{r.d(t,{Zo:()=>l,kt:()=>f});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function s(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function a(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var c=n.createContext({}),d=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):s(s({},t),e)),r},l=function(e){var t=d(e.components);return n.createElement(c.Provider,{value:t},e.children)},p="mdxType",u={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},h=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,i=e.originalType,c=e.parentName,l=a(e,["components","mdxType","originalType","parentName"]),p=d(r),h=o,f=p["".concat(c,".").concat(h)]||p[h]||u[h]||i;return r?n.createElement(f,s(s({ref:t},l),{},{components:r})):n.createElement(f,s({ref:t},l))}));function f(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=r.length,s=new Array(i);s[0]=h;var a={};for(var c in t)hasOwnProperty.call(t,c)&&(a[c]=t[c]);a.originalType=e,a[p]="string"==typeof e?e:o,s[1]=a;for(var d=2;d<i;d++)s[d]=r[d];return n.createElement.apply(null,s)}return n.createElement.apply(null,r)}h.displayName="MDXCreateElement"},1885:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>s,default:()=>u,frontMatter:()=>i,metadata:()=>a,toc:()=>d});var n=r(7462),o=(r(7294),r(3905));const i={},s="adding the server to monitor",a={unversionedId:"connecting-servers/add-server",id:"connecting-servers/add-server",title:"adding the server to monitor",description:"The easiest way to add the server is with the GUI. On the home page, click the `+` button to the right of the server search bar, configure the name and address of the server. The address is the full http/s url to the periphery server, eg `http8000`.",source:"@site/docs/connecting-servers/add-server.md",sourceDirName:"connecting-servers",slug:"/connecting-servers/add-server",permalink:"/connecting-servers/add-server",draft:!1,editUrl:"https://github.com/mbecker20/monitor/tree/main/docsite/docs/connecting-servers/add-server.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"setup monitor periphery",permalink:"/connecting-servers/setup-periphery"},next:{title:"building images",permalink:"/build-images"}},c={},d=[],l={toc:d},p="wrapper";function u(e){let{components:t,...r}=e;return(0,o.kt)(p,(0,n.Z)({},l,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"adding-the-server-to-monitor"},"adding the server to monitor"),(0,o.kt)("p",null,"The easiest way to add the server is with the GUI. On the home page, click the ",(0,o.kt)("inlineCode",{parentName:"p"},"+")," button to the right of the server search bar, configure the name and address of the server. The address is the full http/s url to the periphery server, eg ",(0,o.kt)("inlineCode",{parentName:"p"},"http://12.34.56.78:8000"),"."),(0,o.kt)("p",null,"Once it is added, you can use access the GUI to modify some config, like the alerting thresholds for cpu, memory and disk usage. A server can also be temporarily disabled, this will prevent alerting if it goes offline."),(0,o.kt)("p",null,"Since no state is stored on the periphery servers, you can easily redirect all deployments to be hosted on a different server. Just update the address to point to the new server."))}u.isMDXComponent=!0}}]);