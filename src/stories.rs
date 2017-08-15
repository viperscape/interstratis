extern crate lichen;

use self::lichen::parse::Parser;
use self::lichen::env::Env;

use std::fs;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;

pub struct Stories {
    paths: HashMap<String,PathBuf>,
}

impl Default for Stories {
    fn default () -> Stories {
        let mut stories = Stories {
            paths: HashMap::new()
        };

        stories.path("./stories/");
        
        stories
    }
}

impl Stories {
    pub fn path (&mut self, path: &str) {        
        if let Ok(paths) = fs::read_dir(path) {
            for p in paths {
                if let Ok(p) = p {
                    if p.path().is_file() {
                        if let Some(name) = p.path().file_stem() {
                            let name = name.to_str()
                                .expect("Found non-utf8 filename encoding")
                                .to_owned();
                            
                            self.paths.insert(name,
                                              p.path());
                        }
                    }
                }
            }
        }
    }
    
    pub fn get_paths(&self) -> Vec<&String> {
        let mut p = vec![];
        
        for (k,_) in self.paths.iter() {
            p.push(k);
        }

        p
    }

    pub fn parse (&self, story: &str) -> Option<Env> {
        if let Some(ref story) = self.paths.get(story) {
            if let Ok(h) = fs::File::open(story) {
                let mut r = BufReader::new(h);
                let mut src = String::new();
                if let Ok(rb) = r.read_to_string(&mut src) {
                    if rb > 0 {
                        let p = Parser::parse_blocks(&src);
                        if let Ok(p) = p {
                            return Some(p.into_env())
                        }
                    }
                }
            }
        }

        None
    }
}
