#[macro_use]
extern crate rouille;

extern crate rand;

use rand::random;

use rouille::{Response};

mod stories;
use stories::Stories;

fn main() {
    let stories = Stories::default();
    
    rouille::start_server("localhost:6060", move |rqs| {
        router!(rqs,
                (GET) (/) => {
                    let mut rsp = String::new();
                    rsp.push_str("<h3>interstratis</h3><h4>interactive adventures</h4>");

                    for p in stories.get_paths() {
                        let s = format!("<a href='/stories/{}'>{}</a><br>", p,p);
                        rsp.push_str(&s);
                    }

                    Response::html(rsp)
                },
                (GET) (/stories/{story: String}) => {
                    let cache_id = random::<u32>();
                    Response::redirect_301(format!("/stories/{}/{}",story,cache_id))
                },
                (GET) (/stories/{story: String}/{cache_id: u32}) => {
                    if let Some(e) = stories.parse(&story) {
                        Response::html(format!("Story {}", story))
                    }
                    else {
                        Response::html("<a href='/'>Nothing here</a>")
                    }
                },
                _ => Response::html("<a href='/'>Nothing here</a>")
                
                )
    });
}
