use std::env;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Configuration {
    pub directory: PathBuf,
    shell: String,
    editor: String,
    pager: String,
    agent: String,
}

impl Configuration {
    pub(crate) fn read() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            directory: env::current_dir()?,
            shell: String::from("/bin/bash"),
            editor: String::from("/usr/bin/emacs -nw"),
            pager: String::from("~/.cargo/bin/bat"),
            agent: String::from("agent"),
        })
    }
}