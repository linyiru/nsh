use std::{env,fs::{self,File},process::{Command,Stdio},thread,time::Duration};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=env::args().collect();
 if a.len()!=6{return Err("capture NSH RUNTIME TESTDIR OUT WINDOWS_JS".into());}
 fs::create_dir(&a[4])?;
 let log_path=format!("{}/native.log",a[4]);let log=File::create(&log_path)?;
 let mut child=Command::new(&a[1]).args(["-v","test",&a[3],"--target","macos","--display","1"])
 .env("NSH_RUNTIME",&a[2]).stdout(Stdio::from(log.try_clone()?)).stderr(Stdio::from(log)).spawn()?;
 let mut offset=0;
 loop {
  let content=fs::read_to_string(&log_path)?;
  let remainder=&content[offset..];
  if let Some(last)=remainder.rfind('\n'){
   for line in remainder[..last].lines(){println!("{line}");
    if let Some((_,name))=line.split_once("@@capture ").or_else(||line.split_once("@@video ")){
     let q=Command::new("osascript").args(["-l","JavaScript",&a[5]]).output()?;
     if !q.status.success(){let _=child.kill();return Err(format!("window query failed: {}",String::from_utf8_lossy(&q.stderr)).into());}
     let id=String::from_utf8(q.stderr)?.trim().to_string();
     fs::write(format!("{}/{}-window.json",a[4],name), &q.stdout)?;
     let video=line.contains("@@video");let file=format!("{}/{}.{}",a[4],name,if video{"mov"}else{"png"});
     let mut cmd=Command::new("screencapture");cmd.args(["-x","-o","-l",&id]);
     if video{cmd.args(["-v","-V","5"]);}
     if !cmd.arg(&file).status()?.success(){let _=child.kill();return Err(format!("capture failed: {file}").into());}
    }
   }
   offset+=last+1;
  }
  if let Some(status)=child.try_wait()?{if !status.success(){return Err("native tests failed".into());}break;}
  thread::sleep(Duration::from_millis(50));
 }
 Ok(())
}
