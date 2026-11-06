use std::ffi::c_void;
#[repr(C)] #[derive(Debug,Clone,Copy)] struct Point { x:f64,y:f64 }
#[repr(C)] #[derive(Debug,Clone,Copy)] struct Size { width:f64,height:f64 }
#[repr(C)] #[derive(Debug,Clone,Copy)] struct Rect { origin:Point,size:Size }
#[link(name="CoreGraphics",kind="framework")]
unsafe extern "C" {
 fn CGGetActiveDisplayList(max:u32,ids:*mut u32,count:*mut u32)->i32;
 fn CGDisplayBounds(id:u32)->Rect;
 fn CGDisplayCopyDisplayMode(id:u32)->*mut c_void;
 fn CGDisplayModeGetWidth(mode:*mut c_void)->usize;
 fn CGDisplayModeGetPixelWidth(mode:*mut c_void)->usize;
 fn CGDisplayModeRelease(mode:*mut c_void);
 fn CGPreflightScreenCaptureAccess()->bool;
}
fn main(){unsafe {
 let mut ids=[0u32;16];let mut count=0;
 assert_eq!(CGGetActiveDisplayList(16,ids.as_mut_ptr(),&mut count),0);
 println!("active_displays={count} screen_capture_access={}",CGPreflightScreenCaptureAccess());
 for id in &ids[..count as usize] {
  let mode=CGDisplayCopyDisplayMode(*id);
  println!("display={id} bounds={:?} mode_width={} pixel_width={}",CGDisplayBounds(*id),CGDisplayModeGetWidth(mode),CGDisplayModeGetPixelWidth(mode));
  CGDisplayModeRelease(mode);
 }
}}
