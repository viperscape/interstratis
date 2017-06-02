use std::process::{Command,Child};
use std::thread;
use std::time::Duration;

fn spawn(path: &str) -> Option<Child> {
    let r = Command::new(path).spawn();
    match r {
        Ok(h) => {
            Some(h)
        },
        Err(e) => {
            println!("spawn-err:{:?}",e);
            None
        }
    }
}

pub fn main () {
    thread::sleep(Duration::new(1,0));
    
    if Command::new("cargo")
        .arg("build")
        .arg("--bin")
        .arg("interstratis")
        .status().expect("failed to build").success() {
        
            spawn("../target/debug/interstratis.exe");
        }
}
