use rustyline;
use rustyline::completion::Completer;

use shell::Toolbox;
use std::sync::{Arc, Mutex};


pub struct CmdCompleter(Arc<Mutex<Toolbox>>);

impl CmdCompleter {
    #[inline]
    pub fn new(toolbox: Arc<Mutex<Toolbox>>) -> CmdCompleter {
        CmdCompleter(toolbox)
    }

    #[inline]
    fn commands(&self) -> Vec<String> {
        self.0.lock().unwrap().keys()
    }
}

impl Completer for CmdCompleter {
    #[inline]
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.contains(' ') || line.len() != pos {
            return Ok((0, vec![]));
        }

        let results: Vec<String> = self.commands().iter()
            .filter(|x| x.starts_with(line))
            .map(|x| x.clone() + " ")
            .collect();

        Ok((0, results))
    }
}
