/// This test suite prevents bitrot on examples/docs lichen source files
extern crate lichen;

use lichen::parse::Parser;
use lichen::eval::Eval;
use lichen::var::Var;

use std::fs;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;


#[allow(dead_code)]
struct Data;

impl Eval for Data {
    #[allow(unused_variables)]
    fn get (&self, path: Option<Vec<&str>>, lookup: &str) -> Option<Var> {
        None
    }

    #[allow(unused_variables)]
    fn set (&mut self, path: Option<Vec<&str>>, lookup: &str, var: Var) {
    }
    
    #[allow(unused_variables)]
    fn call (&mut self, var: Var, fun: &str, vars: &Vec<Var>) -> Option<Var> {
        match fun {
            "inc" => {
                if let Ok(v) = Var::get_num(&var, self) {
                    let mut r = v;
                    for n in vars.iter() {
                        if let Ok(v) = Var::get_num(&n, self) {
                            r += v;
                        }
                    }

                    return Some(Var::Num(r))
                }
            },
            _ => { }
        }

        None
    }
}

#[test]
fn bitrot() {
    let mut paths = vec![];

    let add_paths = |paths: &mut Vec<PathBuf>,dir| {
        if let Ok(ps) = fs::read_dir(dir) {
            for p in ps {
                if let Ok(p) = p {
                    if p.path().is_file() {
                        paths.push(p.path());
                    }
                }
            }
        }
    };

    add_paths(&mut paths,"./views");
    add_paths(&mut paths,"./stories");

    assert!(paths.len() > 0);

    for p in paths {
        if let Ok(h) = fs::File::open(&p) {
            let mut r = BufReader::new(h);
            let mut src = String::new();
            if let Ok(_) = r.read_to_string(&mut src) {
                match Parser::parse_blocks(&src) {
                    Ok(b) => { b.into_env(); },
                    Err(e) => { panic!("ERROR: Unable to parse source, {:?} -- {:}", p, e) }
                }
            }
            else { panic!("ERROR: Unable to parse source, {:?}", p) }
        }
        else { panic!("ERROR: Unable to parse source, {:?}", p) }
    }
}
