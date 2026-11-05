import {open,test} from "nsh/test";
test("canvas compositing",async()=>{
const app=await open("/Volumes/DevSSD/nsh-perf-results/canvas/fixture/App.tsx",{platform:"macos",width:840,height:400,clock:{time:1700000000000}});
await app.waitFor("Grow");
await app.screenshot("/Volumes/DevSSD/nsh-perf-results/canvas/pr/pictures-final/fixture-initial.png");
for(const step of ["Grow","Large","Reset"]){await app.press(step);await app.wait(100);await app.screenshot("/Volumes/DevSSD/nsh-perf-results/canvas/pr/pictures-final/fixture-"+step+".png");}
},120000);
