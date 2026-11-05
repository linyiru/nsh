import {open,test} from "nsh/test";
for(let i=0;i<3;i++)test("stocks "+i,async()=>{
console.log("@@initial "+i);
const app=await open("/Volumes/DevSSD/nsh-canvas-perf/examples/gallery/App.tsx",{platform:"macos",width:1200,height:800,clock:{time:1700000000000}});
await app.waitFor("Composite");
console.log("@@updates "+i);
await app.clock.runFor(1000);
console.log("@@end "+i);
});
