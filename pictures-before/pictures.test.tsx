import {open,test,expect} from "nsh/test";
test("before pictures",async()=>{
 const app=await open("/Volumes/DevSSD/nsh-canvas-perf/examples/gallery/App.tsx",{platform:"macos",width:1200,height:800,clock:{time:1700000000000},colorScheme:"light"});
 await app.waitFor("Composite");
 await app.press("label:Pause the market");
 await app.wait(900);
 await app.screenshot("/Volumes/DevSSD/nsh-perf-results/canvas/pr/pictures-before/light.png");
 await app.setColorScheme("dark");
 await app.screenshot("/Volumes/DevSSD/nsh-perf-results/canvas/pr/pictures-before/dark.png");
 await app.press("1D");
 await app.screenshot("/Volumes/DevSSD/nsh-perf-results/canvas/pr/pictures-before/candles.png");
 await app.press("Intraday");
 await app.setViewport(1000,700);
 await app.screenshot("/Volumes/DevSSD/nsh-perf-results/canvas/pr/pictures-before/narrow.png");
},120000);
