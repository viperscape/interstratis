#[macro_use]
extern crate rouille;
extern crate lichen;
extern crate rand;

use rand::random;

use lichen::eval::{Evaluator,EvaluatorState,Empty};
use lichen::parse::Env;

use rouille::{Response};

mod stories;
use stories::Stories;

mod view;
use view::View;

use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use std::process::Command;
use std::env;
use std::thread;


const SERVER_ADDR: &'static str = "localhost:6060";

#[cfg(any(not(unix)))]
const EXEC: &'static str = "lifecycle.exe";

#[cfg(any(unix))]
const EXEC: &'static str = "lifecycle";


struct App {
    stories: Stories,
    cache: HashMap<u32, (EvaluatorState,Env)>,
}

impl Default for App {
    fn default () -> App {
        let stories = Stories::default();
        App {
            stories: stories,
            cache: HashMap::new(),
        }
    }
}

fn main() {
    let reboot_id = env::var("STRATIS_REBOOT").expect("STRATIS_REBOOT id missing");
    let app = Arc::new(Mutex::new(App::default()));
    
    rouille::start_server(SERVER_ADDR, move |rqs| {
        let mut empty = Empty;
        router!(rqs,
                (GET) (/) => {
                    if let Ok(app) = app.lock() {
                        let mut view = View::new("./views/main.ls").expect("Main view missing");
                        let mut rsp = view.render();

                        // NOTE: we want this to instead happen inside view-render stage
                        for p in app.stories.get_paths() {
                            let s = format!("<a href='/stories/{}'>{}</a><br>", p,p);
                            rsp.push_str(&s);
                        }

                        return Response::html(rsp)
                    }

                    Response::empty_404()
                },
                (GET) (/stories/{story: String}) => {
                    if let Ok(mut app) = app.lock() {
                        let id = random::<u32>();
                        
                        if let Some(mut env) = app.stories.parse(&story) {
                            let _ = env.insert_var("meta", "name".to_owned(), story.clone().into());
                            let _ = env.insert_var("meta", "id".to_owned(), (id as f32).into());
                            
                            let state = { Evaluator::new(&mut env, &mut empty).save() };
                            app.cache.insert(id, (state, env));
                        }
                        
                        return Response::redirect_301(format!("/stories/{}/{}",story,id))
                    }

                    Response::empty_404()
                },
                (GET) (/stories/{story: String}/{id: u32}) => {
                    if let Ok(mut app) = app.lock() {
                        if let Some(&mut (ref mut state, ref mut env)) = app.cache.get_mut(&id) {
                            let mut rsp = String::new();

                            let mut ev = state.as_eval(env,&mut empty);
                            
                            rsp.push_str("<!DOCTYPE html><html>");
                            let story_ = format!("Story {}<br>", story);
                            rsp.push_str(&story_);
                            
                            if let Some((mut vars,_node)) = ev.next() {
                                let link = format!("<a href='/stories/{}/{}'>continue</a></br>",story,id);
                                rsp.push_str(&link);
                                
                                for var in vars.drain(..) {
                                    rsp.push_str("<div>");
                                    rsp.push_str(&var.to_string());
                                    rsp.push_str("</div>");
                                }
                            }
                            else {
                                rsp.push_str("<a href='/'>Finished</a>");
                                let link = format!(" | <a href='/stories/{}/{}/restart'>Restart</a>",story,id);
                                rsp.push_str(&link);
                            }

                            rsp.push_str("</html>");

                            *state = ev.save();

                            return Response::html(rsp)
                        }
                        else {
                            // cache id is invalid, some browsers cache this!
                            // lets redirect them again to recreate the id
                            return Response::redirect_301(format!("/stories/{}",story))
                        }
                    }

                    Response::empty_404()
                },
                (GET) (/stories/{story: String}/{id: u32}/restart) => {
                    if let Ok(mut app) = app.lock() {
                        let _ = app.cache.remove(&id);
                    }
                    
                    Response::redirect_301(format!("/stories/{}",story))
                },
                (POST) (/reboot/{id: String}) => {
                    let valid = id == reboot_id;
                    println!("request to shutdown: {:?}",valid);
                    if valid {
                        let targ = format!("./target/debug/{}",EXEC);
                        let _ = Command::new(&targ).spawn();
                        thread::spawn(|| { thread::sleep(std::time::Duration::new(0,500)); std::process::exit(1); });
                    }
                    
                    Response::html("")
                },
                _ => Response::empty_404()
                
                )
    });
}
