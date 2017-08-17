/// This test suite prevents bitrot on examples/docs lichen source files
extern crate lichen;

use lichen::parse::Parser;
use lichen::eval::Evaluator;

use std::fs;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;



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

    add_paths(&mut paths,"./stories");

    assert!(paths.len() > 0);

    for p in paths {
        if let Ok(h) = fs::File::open(&p) {
            let mut r = BufReader::new(h);
            let mut src = String::new();
            if let Ok(_) = r.read_to_string(&mut src) {
                match Parser::parse_blocks(&src) {
                    Ok(b) => {
                        let mut env = b.into_env();
                        assert!(env.src.len() > 0);
                        let mut ev = Evaluator::new(&mut env);
                        println!("Evaluating {:?}", p.to_str());
                        let (vars,next) = ev.next().expect("No values returned on eval");
                        assert!(vars.len() > 0 ||
                                next.is_some());
                    },
                    Err(e) => { panic!("ERROR: Unable to parse source, {:?} -- {:}", p, e) }
                }
            }
            else { panic!("ERROR: Unable to parse source, {:?}", p) }
        }
        else { panic!("ERROR: Unable to parse source, {:?}", p) }
    }
}
