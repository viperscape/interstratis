#[macro_use] extern crate nickel;
extern crate lichen;
extern crate rand;
extern crate cookie;

use rand::random;

use lichen::eval::{Evaluator,EvaluatorState};
use lichen::env::Env;
use lichen::source::Next;


use cookie::Cookie;
use nickel::{Nickel, HttpRouter, FormBody};
use nickel::extensions::Redirect;
use nickel::Request;

mod stories;
use stories::Stories;

//mod view;
//use view::View;

use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use std::process::Command;
use std::env;
use std::thread;
use std::time::{Duration,Instant};


const SERVER_ADDR: &'static str = "localhost:6060";

#[cfg(any(not(unix)))]
const EXEC: &'static str = "lifecycle.exe";

#[cfg(any(unix))]
const EXEC: &'static str = "lifecycle";

pub struct Client {
    session: Instant,
    state: EvaluatorState,
    env: Env,
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
    fn get_client<'c> (&'c self, req: &Request) -> Option<&'c Client> {
        if let Some(cookies) =  req.origin.headers.get_raw("Cookie") {
            let cookies = parse_cookies(cookies);
            if let Some(sid) = get_cookie("sid", &cookies) {
                return self.clients.get(sid)
                //if let Some(c) = app.clients.get(sid) {
                //if c.session.elapsed() < Duration::from_secs(max_age) {
                //    return c
                //}
                //}
            }
        }

        None
    }

    fn add_client (&mut self) -> Vec<u8> {
        let sid = random::<u64>() .to_string();
        let sid = Cookie::new("sid", sid.clone()).to_string();

        let mut env = Env::empty();
        let state = Evaluator::new(&mut env).save();
        
        let client = Client { session: Instant::now(), env: env, state: state };
        self.clients.insert(sid.clone(),client);
        sid.as_bytes().to_vec()
    }
}

fn main() {
    let reboot_id = env::var("STRATIS_REBOOT").expect("STRATIS_REBOOT id missing");
    let app = Arc::new(Mutex::new(App::default()));
    let mut server = Nickel::new();
    apply_routes(&mut server, &app);
    
    server.listen("0.0.0.0:6063");
}

fn apply_routes(server: &mut Nickel, app: &Arc<Mutex<App>>) {
    let app = app.clone();
    server.get("/", middleware! {
        |req, mut res|
        if let Ok(mut app) = app.lock() {
            let sid = app.add_client();
            res.headers_mut().set_raw("Set-Cookie",
                                      vec![sid]);
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

fn get_cookie<'a>(name: &str, cookies: &'a[Cookie]) -> Option<&'a str> {
    for c in cookies {
        let kv = c.name_value();
        if kv.0 == name {
            return Some(kv.1)
        }
    }

    None
}
