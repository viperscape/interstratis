#[macro_use]
extern crate rouille;

use rouille::{Response};

use std::fs;
use std::path::PathBuf;

fn main() {
    rouille::start_server("localhost:6060", move |rqs| {
        router!(rqs,
                (GET) (/) => {
                    if let Ok(paths) = fs::read_dir("./stories/") {
                        let mut stories: Vec<PathBuf> = vec![];
                        for p in paths {
                            if let Ok(p) = p {
                                if p.path().is_file() {
                                    stories.push(p.path())
                                }
                            }
                        }
                        
                    }

                    Response::html("<h3>interstratis</h3>interactive adventures")
                },
                _ => Response::html("<a href='/'>Nothing here</a>")
        
        )
    });
}
