use rustyline::{self, Context};
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;

use shell::Toolbox;
use std::borrow::Cow::{self, Borrowed};
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
    type Candidate = String;

    #[inline]
    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<String>)> {
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

impl Hinter for CmdCompleter {
    #[inline]
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for CmdCompleter {
    #[inline]
    fn highlight_prompt<'p>(&self, prompt: &'p str) -> Cow<'p, str> {
        Borrowed(prompt)
    }

    #[inline]
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Borrowed(hint)
    }
}

impl rustyline::Helper for CmdCompleter {}
