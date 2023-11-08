"use strict";(self.webpackChunkdocsite=self.webpackChunkdocsite||[]).push([[371],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>f});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function u(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var s=r.createContext({}),c=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},p=function(e){var t=c(e.components);return r.createElement(s.Provider,{value:t},e.children)},l="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,i=e.originalType,s=e.parentName,p=u(e,["components","mdxType","originalType","parentName"]),l=c(n),m=a,f=l["".concat(s,".").concat(m)]||l[m]||d[m]||i;return n?r.createElement(f,o(o({ref:t},p),{},{components:n})):r.createElement(f,o({ref:t},p))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=n.length,o=new Array(i);o[0]=m;var u={};for(var s in t)hasOwnProperty.call(t,s)&&(u[s]=t[s]);u.originalType=e,u[l]="string"==typeof e?e:a,o[1]=u;for(var c=2;c<i;c++)o[c]=n[c];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},7559:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>d,frontMatter:()=>i,metadata:()=>u,toc:()=>c});var r=n(7462),a=(n(7294),n(3905));const i={},o="authenticating requests",u={unversionedId:"api/authenticating-requests",id:"api/authenticating-requests",title:"authenticating requests",description:"monitor uses the JSON Web Token (JWT) standard to authenticate all requests to subroutes under /api.",source:"@site/docs/api/authenticating-requests.md",sourceDirName:"api",slug:"/api/authenticating-requests",permalink:"/api/authenticating-requests",draft:!1,editUrl:"https://github.com/mbecker20/monitor/tree/main/docsite/docs/api/authenticating-requests.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"types",permalink:"/api/types"},next:{title:"login",permalink:"/api/login"}},s={},c=[],p={toc:c},l="wrapper";function d(e){let{components:t,...n}=e;return(0,a.kt)(l,(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"authenticating-requests"},"authenticating requests"),(0,a.kt)("p",null,"monitor uses the ",(0,a.kt)("inlineCode",{parentName:"p"},"JSON Web Token (JWT)")," standard to authenticate all requests to subroutes under ",(0,a.kt)("inlineCode",{parentName:"p"},"/api"),".\nusers can acquire a ",(0,a.kt)("inlineCode",{parentName:"p"},"JWT")," using a ",(0,a.kt)("a",{parentName:"p",href:"/api/login"},"login method"),"."),(0,a.kt)("p",null,"to authenticate requests, pass the ",(0,a.kt)("inlineCode",{parentName:"p"},"JWT")," under the ",(0,a.kt)("inlineCode",{parentName:"p"},"Authorization")," header:"),(0,a.kt)("p",null,(0,a.kt)("inlineCode",{parentName:"p"},"Authorization: Bearer <JWT>")))}d.isMDXComponent=!0}}]);