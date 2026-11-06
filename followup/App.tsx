import {mount,Canvas,View,Text,createAnimatedValue,createSignal,Animated,Easing,pixelRatio,cleanup} from "nsh";
import {Window} from "nsh/desktop";
function Sample(p) {
 return <View style={{width:280,height:150,padding:20,gap:8}}><Text>{p.kind}</Text>
 <View style={{transform:p.kind==='parent transform'?'translate(9px, 4px) rotate(-9deg) scale(0.85)':undefined}}>
 <Canvas style={{width:240,height:100,backgroundColor:'#f5e4d1',opacity:p.kind==='opacity'?0.5:1,borderRadius:12,overflow:'hidden',transform:p.kind==='view transform'?'translate(7px, 3px) rotate(12deg) scale(0.8)':undefined}}
 draw={cx=>{
  if(p.kind==='saved fallback')cx.save();
  if(p.kind==='clip'){cx.beginPath();cx.rect(12,12,210,66);cx.clip();}
  cx.fillStyle='#2a8370';cx.beginPath();cx.moveTo(10,80);
  for(let i=0;i<40;i++)cx.lineTo(10+i*5,45+20*Math.sin(i));
  cx.lineTo(205,80);cx.closePath();cx.fill();
  if(p.kind==='erase'){cx.globalCompositeOperation='destination-out';cx.fillRect(10,20,30,50);cx.globalCompositeOperation='source-over';}
  cx.fillStyle='#dc2f65';cx.globalAlpha=0.4;cx.beginPath();cx.arc(180,40,p.r,0,Math.PI*2);cx.fill();
  cx.globalAlpha=1;cx.beginPath();cx.arc(180,40,3,0,Math.PI*2);cx.fill();
  if(p.kind==='saved fallback')cx.restore();
 }}/></View></View>;
}
function App(){
 Window.setTitle('NSH Canvas Review');
 const r=createAnimatedValue(7);
 const loop=Animated.loop(Animated.timing(r,{toValue:120,duration:2400,easing:Easing.linear}));
 cleanup(()=>loop.stop());
 const step=v=>{loop.stop();r.set(v);};
 return <View style={{flex:1,backgroundColor:'#fff',padding:20,gap:12}}>
 <Text>Canvas review · pixel ratio {pixelRatio()}</Text>
 <View style={{flexDirection:'row',gap:24}}>
 <View onPress={()=>step(35)}><Text>Grow</Text></View><View onPress={()=>step(120)}><Text>Large</Text></View><View onPress={()=>step(7)}><Text>Reset</Text></View>
 <View onPress={()=>{r.set(7);loop.start();}}><Text>Animate</Text></View>
 <View onPress={()=>Window.setFrame({origin:{x:200,y:100}})}><Text>Screen1</Text></View>
 <View onPress={()=>Window.setFrame({origin:{x:4000,y:-600}})}><Text>Screen2</Text></View>
 <View onPress={()=>Window.setFrame({origin:{x:4000,y:800}})}><Text>Screen3</Text></View>
 </View>
 <View style={{flexDirection:'row'}}><Sample kind="plain" r={r}/><Sample kind="view transform" r={r}/><Sample kind="parent transform" r={r}/></View>
 <View style={{flexDirection:'row'}}><Sample kind="clip" r={r}/><Sample kind="opacity" r={r}/><Sample kind="erase" r={r}/></View>
 <View style={{flexDirection:'row'}}><Sample kind="saved fallback" r={r}/></View>
 </View>;
}
mount(App);
