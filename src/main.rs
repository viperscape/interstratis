#[macro_use]
extern crate rouille;
extern crate lichen;
extern crate rand;

use rand::random;

use lichen::eval::{Eval,Evaluator,EvaluatorState};
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
    let app = Arc::new(Mutex::new(App::default()));
    
    rouille::start_server("localhost:6060", move |rqs| {
        let mut empty = Empty;
        router!(rqs,
                (GET) (/) => {
                    if let Ok(app) = app.lock() {
                        let mut view = View::new("./views/main.ls").expect("Main view missing");
                        let mut rsp = view.render();
                        
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
                            let mut rsp = format!("Story {}<br>", story);
                            let mut ev = state.as_eval(env,&mut empty);
                            
                            if let Some((mut vars,_node)) = ev.next() {
                                let link = format!("<a href='/stories/{}/{}'>continue</a></br>",story,id);
                                rsp.push_str(&link);
                                
                                for var in vars.drain(..) {
                                    rsp.push_str(&var.to_string());
                                    rsp.push_str("<br>");
                                }
                            }
                            else {
                                rsp.push_str("<a href='/'>Finished</a>");
                                let link = format!(" | <a href='/stories/{}/restart'>Restart</a>",id);
                                rsp.push_str(&link);
                            }

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
                (GET) (/stories/{id: u32}/restart) => {
                    if let Ok(mut app) = app.lock() {
                        let _ = app.cache.remove(&id);
                    }
                    
                    Response::redirect_301("/")
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
