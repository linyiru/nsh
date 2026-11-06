use std::{hint::black_box,time::Instant};
use nsh_core::canvas::{CanvasOp,DamageScan,Num,PathOp,FillRule};
fn main(){
 for count in [40,10000] {
  let c=Num::Const;
  let mut ops=vec![CanvasOp::BeginPath,CanvasOp::Path(PathOp::MoveTo(c(0.0),c(60.0)))];
  for i in 0..count {ops.push(CanvasOp::Path(PathOp::LineTo(c(i as f32*0.036),c(60.0+30.0*(i as f32*0.006).sin()))));}
  ops.extend([CanvasOp::Stroke(None),CanvasOp::BeginPath,CanvasOp::Path(PathOp::Arc{x:c(360.0),y:c(60.0),radius:Num::Slot(0),start:c(0.0),end:c(std::f32::consts::TAU),ccw:false}),CanvasOp::Fill(None,FillRule::NonZero)]);
  let mut scan=DamageScan::default();let changed=vec![0u16];let mut values=vec![3.0f32];
  let iterations=if count==40{100000}else{2000};
  for round in 0..8 {
   let start=Instant::now();
   for i in 0..iterations {values[0]=3.0+(i%10) as f32;
    black_box(scan.damage(black_box(&ops),|s|black_box(&values).get(s as usize).copied().unwrap_or(0.0),|s|black_box(&changed).contains(&s)));
   }
   println!("{{\"points\":{count},\"round\":{round},\"ns_per_scan\":{:.3}}}",start.elapsed().as_nanos() as f64/iterations as f64);
  }
 }
}
