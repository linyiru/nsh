import {mount,Canvas, View, Text, createAnimatedValue} from "nsh";
function Sample(p) {
 return <View><Text>{p.kind}</Text><Canvas style={{width:240,height:100,backgroundColor:"#f5e4d1",opacity:p.kind === "opacity" ? 0.5 : 1, borderRadius:12,overflow:"hidden"}} draw={cx=>{
  if(p.kind === "saved") cx.save();
  if(p.kind === "clip") {cx.beginPath();cx.rect(12,12,210,66);cx.clip();}
  cx.fillStyle="#2a8370";cx.beginPath();cx.moveTo(10,80);
  for(let i=0;i<40;i++) cx.lineTo(10+i*5, 45+20*Math.sin(i));
  cx.lineTo(205,80);cx.closePath();cx.fill();
  if(p.kind === "erase") {cx.globalCompositeOperation="destination-out";cx.fillRect(10,20,30,50);cx.globalCompositeOperation="source-over";}
  cx.fillStyle="#dc2f65";cx.globalAlpha=0.4;cx.beginPath();cx.arc(180,40,p.r,0,Math.PI*2);cx.fill();
  cx.globalAlpha=1;cx.beginPath();cx.arc(180,40,3,0,Math.PI*2);cx.fill();
  if(p.kind === "saved") cx.restore();
  if(p.kind === "copy") {cx.globalCompositeOperation="copy";cx.fillRect(210,60,10,10);}
 }}/></View>;
}
export default function App(){const r=createAnimatedValue(7);return <View style={{padding:20,gap:10,backgroundColor:"#fff",flex:1}}>
<View style={{flexDirection:"row",gap:20}}><View onPress={()=>r.set(35)}><Text>Grow</Text></View><View onPress={()=>r.set(120)}><Text>Large</Text></View><View onPress={()=>r.set(7)}><Text>Reset</Text></View></View>
<View style={{flexDirection:"row",gap:20}}><Sample kind="plain" r={r}/><Sample kind="opacity" r={r}/><Sample kind="clip" r={r}/></View>
<View style={{flexDirection:"row",gap:20}}><Sample kind="erase" r={r}/><Sample kind="saved" r={r}/><Sample kind="copy" r={r}/></View>
</View>;}

mount(App);
