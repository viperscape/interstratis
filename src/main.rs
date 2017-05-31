#[macro_use]
extern crate rouille;
extern crate lichen;
extern crate rand;

use rand::random;

use lichen::eval::{Eval,Evaluator};
use lichen::var::Var;
use lichen::parse::Env;

use rouille::{Response};

mod stories;
use stories::Stories;

mod view;
use view::View;

use std::sync::{Arc,Mutex};
use std::collections::HashMap;


struct App {
    views: Vec<View>,
    stories: Stories,
    cache: HashMap<u32, Env>,
}

impl Default for App {
    fn default () -> App {
        let stories = Stories::default();
        let view = View::new("./views/main.ls").expect("Main view missing");
        App {
            views: vec![view],
            stories: stories,
            cache: HashMap::new(),
        }
    }
}

fn main() {
    let app = Arc::new(Mutex::new(App::default()));
    
    rouille::start_server("localhost:6060", move |rqs| {
        let mut empty = Empty;
        router!(rqs,
                (GET) (/) => {
                    let mut rsp = app.lock().unwrap().views[0].render();
                    
                    for p in app.lock().unwrap().stories.get_paths() {
                        let s = format!("<a href='/stories/{}'>{}</a><br>", p,p);
                        rsp.push_str(&s);
                    }

                    Response::html(rsp)
                },
                (GET) (/stories/{story: String}) => {
                    let id = random::<u32>();
                    if let Some(env) = app.lock().unwrap().stories.parse(&story) {   
                        app.lock().unwrap().cache.insert(id,env);
                    }
                    
                    
                    Response::redirect_301(format!("/stories/{}/{}",story,id))
                },
                (GET) (/stories/{story: String}/{id: u32}) => {
                    if let Some(ref mut env) = app.lock().unwrap().cache.get_mut(&id) {
                        let mut ev = Evaluator::new(env, &mut empty);
                        if let Some((mut vars,_node)) = ev.next() {
                            let mut story = format!("Story {}<br>", story);
                            
                            for var in vars.drain(..) {
                                story.push_str(&var.to_string());
                            }
                            
                            Response::html(story)
                        }
                        else {
                            Response::html("<a href='/'>Finished</a>")
                        }
                    }
                    else {
                        Response::html("<a href='/'>Nothing here</a>")
                    }
                },
                _ => Response::html("<a href='/'>Nothing here</a>")
                
                )
    });
}


struct Empty;
impl Eval for Empty {
    #[allow(unused_variables)]
    fn get (&self, path: Option<Vec<&str>>, lookup: &str) -> Option<Var> {
        None
    }

    #[allow(unused_variables)]
    fn set (&mut self, path: Option<Vec<&str>>, lookup: &str, var: Var) {
    }

    #[allow(unused_variables)]
    fn call (&mut self, var: Var, fun: &str, vars: &Vec<Var>) -> Option<Var> {
        None
    }
}
