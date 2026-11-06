//! Local experiment runner: same app bytecode, fresh host process and data
//! directory per run. std-only; libproc CPU counters are converted from Mach ticks.
use std::{
    env,
    fs::{self, File},
    io,
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
#[repr(C)]
#[derive(Default)]
struct Usage {
    uuid: [u8; 16],
    counters: [u64; 10],
}
#[repr(C)]
#[derive(Default)]
struct Timebase {
    numer: u32,
    denom: u32,
}
#[link(name = "proc")]
unsafe extern "C" {
    fn proc_pid_rusage(pid: i32, flavor: i32, usage: *mut Usage) -> i32;
}
unsafe extern "C" {
    fn mach_timebase_info(info: *mut Timebase) -> i32;
}
struct Host(Child);
impl Drop for Host {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn usage(pid: u32) -> io::Result<Usage> {
    let mut u = Usage::default();
    if unsafe { proc_pid_rusage(pid as i32, 0, &mut u) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(u)
}
fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(f64::total_cmp);
    let n = xs.len();
    if n % 2 == 0 {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    } else {
        xs[n / 2]
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = env::args().collect();
    if a.len() < 6 {
        return Err("usage: measure OUT BUNDLE DEMO SECONDS HOST [HOST ...]".into());
    }
    if matches!(a[3].as_str(), "Stocks" | "Canvas") {
        let capabilities = fs::read_to_string(PathBuf::from(&a[2]).with_file_name("capabilities.json"))?;
        if !capabilities.contains("NSH_GALLERY_DEMO") {
            return Err("gallery bundle must include its environment capabilities".into());
        }
    }
    let out = PathBuf::from(&a[1]);
    fs::create_dir(&out)?;
    let seconds: u64 = a[4].parse()?;
    assert!(seconds >= 6);
    let mut base = Timebase::default();
    assert_eq!(unsafe { mach_timebase_info(&mut base) }, 0);
    for (i, host_path) in a[5..].iter().enumerate() {
        let log_path = out.join(format!("{i}.log"));
        let log = File::create(&log_path)?;
        let host = Host(
            Command::new(host_path)
                .args([
                    "--bundle",
                    &a[2],
                    "--window-size",
                    "1200x800",
                    "--display",
                    "1",
                    "--fps",
                    "--trace",
                    "--data-dir",
                ])
                .arg(out.join(format!("data-{i}")))
                .env("NSH_GALLERY_DEMO", &a[3])
                .stdout(Stdio::from(log.try_clone()?))
                .stderr(Stdio::from(log))
                .spawn()?,
        );
        thread::sleep(Duration::from_secs(5));
        let first = usage(host.0.id())?;
        let begin = Instant::now();
        let mut mem = Vec::new();
        for _ in 0..seconds {
            thread::sleep(Duration::from_secs(1));
            mem.push(usage(host.0.id())?.counters[7] as f64 / 1e6);
        }
        let last = usage(host.0.id())?;
        let elapsed = begin.elapsed().as_secs_f64();
        let ticks = (last.counters[0] + last.counters[1]) - (first.counters[0] + first.counters[1]);
        let cpu =
            ticks as f64 * f64::from(base.numer) / f64::from(base.denom) / 1e9 / elapsed * 100.0;
        drop(host);
        let log = fs::read_to_string(&log_path)?;
        if log.contains("ERROR") {
            return Err(format!("runtime error: {}", log_path.display()).into());
        }
        let first_commit = log.lines().find(|line| line.contains("perf commit:")).ok_or("missing initial commit")?;
        let words: Vec<_> = first_commit.split_whitespace().collect();
        let node_index = words.iter().position(|&w| w == "nodes").ok_or("missing initial node count")?;
        let nodes: usize = words[node_index - 1].parse()?;
        let expected = match a[3].as_str() { "Stocks" => 1249, "Canvas" => 592, "Heavy" => 3, _ => return Err("unknown workload".into()) };
        if nodes != expected {
            return Err(format!("wrong workload: {} has {nodes} initial nodes, expected {expected}", a[3]).into());
        }
        let mut hz = Vec::new();
        let mut fps = Vec::new();
        let mut p50 = Vec::new();
        let mut p95 = Vec::new();
        let mut over = Vec::new();
        let mut commits = Vec::new();
        let mut seen = 0;
        for line in log.lines() {
            let words: Vec<_> = line.split_whitespace().collect();
            let Some(k) = words.iter().position(|&w| w == "fps") else {
                continue;
            };
            seen += 1;
            if seen <= 5 {
                continue;
            }
            let rates: Vec<_> = words[k + 1].split('/').collect();
            fps.push(rates[0].parse::<f64>()?);
            hz.push(rates[1].parse::<f64>()?);
            let frame: Vec<_> = words[k + 3].split('/').collect();
            p50.push(frame[1].parse()?);
            p95.push(frame[2].parse()?);
            if let Some(k) = words.iter().position(|&w| w == "over") {
                over.push(words[k + 1].trim_end_matches('%').parse()?);
            }
            if let Some(k) = words.iter().position(|&w| w == "commits") {
                commits.push(words[k + 1].parse()?);
            }
        }
        if fps.len() < seconds as usize - 3 {
            return Err(format!("too few HUD samples: {}", log_path.display()).into());
        }
        if a[3] == "Stocks" && (median(fps.clone()) < 30.0 || median(commits.clone()) < 10.0) {
            return Err("Stocks is not active".into());
        }
        let row = format!(
            "{{\"host\":{:?},\"hz\":{},\"hz_min\":{},\"hz_max\":{},\"run\":{i},\"cpu\":{cpu:.4},\"footprint_MB\":{:.4},\"fps\":{},\"p50_ms\":{},\"p95_ms\":{},\"over_percent\":{},\"commits\":{}}}",
            host_path,
            median(hz.clone()),
            hz.iter().copied().reduce(f64::min).unwrap(),
            hz.iter().copied().reduce(f64::max).unwrap(),
            median(mem),
            median(fps),
            median(p50),
            median(p95),
            median(over),
            median(commits)
        );
        fs::write(out.join(format!("{i}.json")), &row)?;
        println!("{row}");
    }
    Ok(())
}
