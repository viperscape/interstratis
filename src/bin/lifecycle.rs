use std::process::Command;
use std::thread;
use std::time::Duration;
use std::env;

pub fn main () {
    let stratis_service = env::var("STRATIS_SERVICE").expect("STRATIS_SERVICE path missing");
    thread::sleep(Duration::new(1,0));
    
    let r = Command::new("./inter-service.sh")
        .current_dir(&stratis_service)
        .status().expect("failed to build").success();

    println!("Rebuilt interstratis service {:?}",r);
}
