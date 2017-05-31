use lichen::parse::{Parser, Env};
use lichen::eval::{Evaluator};
use lichen::var::Var;

use std::fs;
use std::io::BufReader;
use std::io::prelude::*;


pub struct View {
    env: Env
}

impl View {
    pub fn new (view: &str) -> Option<View> {
        if let Ok(h) = fs::File::open(view) {
            let mut r = BufReader::new(h);
            let mut src = String::new();
            if let Ok(rb) = r.read_to_string(&mut src) {
                if rb > 0 {
                    let p = Parser::parse_blocks(&src);
                    if let Ok(p) = p {
                        return Some(View { env: p.into_env() })
                    }
                }
            }
        }

        None
    }
    
    pub fn render (&mut self) -> String {
        let mut r = String::new();
        let mut data = ::Empty;
        let eval = Evaluator::new(&mut self.env, &mut data);

        for (mut vars,_) in eval {
            for var in vars.drain(..) {
                match var {
                    Var::String(s) => r.push_str(s.as_str()),
                    _ => {},
                }
            }
        }

        r
    }
}
