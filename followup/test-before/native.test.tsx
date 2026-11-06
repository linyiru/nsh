import {open,test,expect} from "nsh/test";
test("transformed canvases and screen movement",async()=>{
 const app=await open("/Volumes/DevSSD/nsh-perf-results/canvas/followup/App.tsx",{platform:"macos",width:900,height:620,clock:{time:1700000000000},colorScheme:"light"});
 await app.waitFor("Grow");
 for(const step of ["Reset","Grow","Large","Reset","Screen2","Screen3","Screen1"]){
  await app.press(step);await app.wait(200);
  console.log("@@capture "+step+"-"+captures++);
  await new Promise(r=>setTimeout(r,1800));
 }
 await app.press("Animate");
 console.log("@@video transitions");
 await new Promise(r=>setTimeout(r,7000));
 await app.press("Reset");
},90000);
let captures=0;
