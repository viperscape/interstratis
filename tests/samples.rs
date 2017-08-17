/// This test suite prevents bitrot on examples/docs lichen source files
extern crate lichen;

use lichen::parse::Parser;

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
                    Ok(b) => { b.into_env(); },
                    Err(e) => { panic!("ERROR: Unable to parse source, {:?} -- {:}", p, e) }
                }
            }
            else { panic!("ERROR: Unable to parse source, {:?}", p) }
        }
        else { panic!("ERROR: Unable to parse source, {:?}", p) }
    }
}
