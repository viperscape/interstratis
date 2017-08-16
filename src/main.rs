#[macro_use] extern crate nickel;
extern crate lichen;
extern crate rand;
extern crate cookie;
extern crate rustc_serialize;

use rand::random;

use lichen::eval::{Evaluator,EvaluatorState};
use lichen::env::Env;
use lichen::var::Var;
use lichen::source::Next;


use cookie::Cookie;
use nickel::{Nickel, HttpRouter};//, FormBody};
use nickel::extensions::Redirect;
use nickel::Request;

mod stories;
use stories::Stories;

//mod view;
//use view::View;

use std::sync::{Arc,Mutex};
use std::collections::HashMap;
//use std::process::Command;
use std::env;
//use std::thread;
use std::time::{Duration,Instant};


const SERVER_ADDR: &'static str = "localhost:6060";

#[cfg(any(not(unix)))]
const EXEC: &'static str = "lifecycle.exe";

#[cfg(any(unix))]
const EXEC: &'static str = "lifecycle";

pub struct Client {
    session: Instant,
    story: Option<String>,
    state: EvaluatorState,
    env: Env,
}

impl Default for Client {
    fn default() -> Client {
        let mut env = Env::empty();
        let state = Evaluator::new(&mut env).save();
        
        Client { session: Instant::now(),
                 env: env,
                 state: state,
                 story: None }
    }
}


pub type Clients = HashMap<String,Client>;
struct App {
    stories: Stories,
    clients: Clients,
    last_reboot: Instant,
}

impl Default for App {
    fn default () -> App {
        let stories = Stories::default();
        App {
            stories: stories,
            clients: HashMap::new(),
            last_reboot: Instant::now(),
        }
    }
}

impl App {
    fn parse_sid<'c> (&self, req: &'c Request) -> Option<String> {
        if let Some(cookies) = req.origin.headers.get_raw("Cookie") {
            let cookies = parse_cookies(cookies);
            return get_cookie("sid", &cookies)
        }

        None
    }
    
    
    fn get_client<'c> (&'c self, req: &Request) -> Option<&'c Client> {
        if let Some(ref sid) = self.parse_sid(req) {
            return self.clients.get(sid)
        }

        None
    }

    fn get_client_mut<'c> (&'c mut self, req: &Request) -> Option<&'c mut Client> {
        if let Some(ref sid) = self.parse_sid(req) {
            return self.clients.get_mut(sid)
        }

        None
    }

    fn add_client (&mut self) -> Vec<u8> {
        let sid = random::<u64>() .to_string();
        let sid_cookie = Cookie::new("sid", sid.clone()).to_string();

        let client = Client::default();
        
        self.clients.insert(sid,client);
        sid_cookie.as_bytes().to_vec()
    }
}

fn main() {
    //let reboot_id = env::var("STRATIS_REBOOT").expect("STRATIS_REBOOT id missing");
    let app = Arc::new(Mutex::new(App::default()));
    let mut server = Nickel::new();
    apply_routes(&mut server, &app);
    
    server.listen("0.0.0.0:6063");
}

fn lock_err() -> &'static str {
    "error on posioned mutex"
}

fn apply_routes(server: &mut Nickel, app_: &Arc<Mutex<App>>) {
    let app = app_.clone();
    server.get("/", middleware! {
        |req, mut res|        
        if let Ok(mut app) = app.lock() {
            if let Some(c) = app.get_client(req) {
                if let Some(ref story) = c.story {
                    return res.redirect(format!("/story/{}",story))
                }

                return res.redirect("/stories")
            }
            
            let sid = app.add_client();
            res.headers_mut().set_raw("Set-Cookie",
                                      vec![sid]);
            return res.redirect("/stories")
        }
        
        lock_err()
    });

    let app = app_.clone();
    server.get("/story/:story", middleware! {
        |req, res|
        if let Ok(mut app) = app.lock() {
            if let Some(story) = req.param("story") {
                if let Some(mut env) = app.stories.parse(story) {
                    if let Some(ref mut c) = app.get_client_mut(req) {
                        // update the client
                        c.state = { Evaluator::new(&mut env).save() };
                        c.env = env;
                        c.story = Some(story.to_owned());
                        c.session = Instant::now();
                        return res.redirect(format!("/story/{}/",story))
                    }
                }
            }
        }

        ""
    });

    let app = app_.clone();
    server.get("/story/:story/:advance/", middleware! {
        |req, res|
        if let Ok(mut app) = app.lock() {
            if let Some(ref mut c) = app.get_client_mut(req) {
                let mut ev = c.state.as_eval(&mut c.env);
                if let Some(advance) = req.param("advance") {
                    ev.advance(advance.to_owned());
                    c.state = ev.save();
                    if let Some(story) = req.param("story") {
                        return res.redirect(format!("/story/{}/",story))
                    }
                }
            }
        }
    });
    
    let app = app_.clone();
    server.get("/story/:story/", middleware! {
        |req, res|
        if let Ok(mut app) = app.lock() {
            if let Some(ref mut c) = app.get_client_mut(req) {
                let mut ev = c.state.as_eval(&mut c.env);

                let mut map = HashMap::new();
                let mut nexts: Vec<NextResult> = vec![];
                let mut vars: Vec<String> = vec![];
                
                while let Some((mut v,next)) = ev.next() {
                    // add in vars for rendering
                    for var in v.drain(..) {
                        vars.push(var.to_string());
                    }

                    // add in nodes for rendering
                    if let Some(next) = next {
                        match next {
                            Next::Await(node) => {
                                let nr = NextResult {
                                        name: node.to_owned(),
                                        block: node.to_string(),
                                };
                                nexts.push(nr);
                            },
                            Next::Select(selects) => {
                                for (name,node) in selects.iter() {
                                    let nr = NextResult {
                                        name: name.to_owned(),
                                        block: node[0].to_string(),
                                    };
                                    nexts.push(nr);
                                }
                            },
                            _ => { }
                        }
                    }

                    if vars.len() > 0 ||
                        nexts.len() > 0 { break }
                }

                c.state = ev.save();

                
                if vars.len() < 1 &&
                    nexts.len() < 1 {
                    c.story = None;
                    return res.redirect("/");
                }

                if let Some(story) = req.param("story") {
                    let er = EmitResult {
                        story: story.to_owned(),
                        vars: vars,
                        nexts: nexts
                    };
                    map.insert("result".to_owned(),er);
                    
                    return res.render("views/story.html", &map);
                }
                
            }
        }
    });

    let app = app_.clone();
    server.get("/stories", middleware! {
        |req, res|        
        if let Ok(mut app) = app.lock() {
            let mut map = HashMap::new();
            map.insert("story".to_owned(), app.stories.get_paths());
            
            return res.render("views/stories.html", &map);
        }

        ""
    });
}

fn parse_cookies(raw: &[Vec<u8>]) -> Vec<Cookie> {
    let mut cookies = vec!();
    for cookies_raw in raw.iter() {
        let cookies_str = String::from_utf8(cookies_raw.clone())
            .expect("Non-utf8 encoding encountered");
        for cookie_str in cookies_str.split(';') {
            let s = cookie_str.trim().to_owned();
            if let Ok(cookie) = Cookie::parse(s) {
                cookies.push(cookie);
            }
            
        }
    }

    cookies
}

fn get_cookie(name: &str, cookies: &[Cookie]) -> Option<String> {
    for c in cookies {
        let kv = c.name_value();
        if kv.0 == name {
            return Some(kv.1.to_owned())
        }
    }

    None
}


#[derive(RustcEncodable)]
struct EmitResult {
    vars: Vec<String>,
    nexts: Vec<NextResult>,
    story: String,
}

#[derive(RustcEncodable)]
struct NextResult {
    name: String, // formal name presented
    block: String, //destination block
}
