#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};

use std::process::Command;
use std::time::{Duration,Instant};
use std::env;

use std::sync::{Arc,Mutex};


pub fn main () {
    let port = env::var("STRATIS_SERVICE").expect("STRATIS_SERVICE path missing");
    let key = env::var("STRATIS_REBOOT").expect("STRATIS_REBOOT path missing");

    let r = run_stratis();
    println!("Rebuilt interstratis server {:?}",r);

    let mut server = Nickel::new();
    let last_cycle = Arc::new(Mutex::new(Instant::now()));
    apply_routes(&mut server, key, &last_cycle);

    println!("Running inter-service");
    let r = server.listen("0.0.0.0".to_owned() + &port);
    println!("Listening: {:?}",r);
}


fn run_stratis () -> bool {
    let dir = env::var("STRATIS_DIR").expect("STRATIS_DIR path missing");
    // run service script to pull repo, etc.
    let _ = Command::new("./interstratis.sh")
        .current_dir(&dir)
        .status().expect("failed to build").success();

    // spawn executable manually
    Command::new("./target/debug/interstratis")
        .current_dir(&dir)
        .spawn().is_ok()
}

fn apply_routes(server: &mut Nickel, key_: String, last_cycle_: &Arc<Mutex<Instant>>) {
    let last_cycle = last_cycle_.clone();
    server.get("/cycle/:key", middleware! {
        |req, _res|
        if let Some(key) = req.param("key") {
            if key == &key_ {
                if let Ok(mut last) = last_cycle.lock() {
                    if last.elapsed() > Duration::new(60,0) { //only reboot every so often
                        *last = Instant::now(); //update
                        let r = run_stratis();
                        println!("Rebuilt interstratis server {:?}",r);
                    }
                }
            }
        }
        
        ""
    });
}
