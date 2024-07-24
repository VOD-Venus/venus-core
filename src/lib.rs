use std::{
    io::{BufRead, BufReader},
    process::{Child, Command},
    sync::mpsc::{self, Receiver},
    thread,
};

use anyhow::anyhow;
use config::Config;
use consts::get_v2ray_exe_path;
use error::VenusResult;
use log::error;

pub mod config;
pub mod consts;
pub mod error;

#[derive(Debug)]
pub struct Venus {
    /// v2ray and venus's self config
    pub config: Config,

    /// v2ray process
    pub child: Option<Child>,
    pub child_rx: Option<Receiver<String>>,
}

impl Venus {
    pub fn new() -> VenusResult<Self> {
        let config = Config::new()?;
        Ok(Self {
            config,

            child: None,
            child_rx: None,
        })
    }
}

impl Venus {
    pub fn spawn_core(&mut self) -> VenusResult<()> {
        let core_exec_path = get_v2ray_exe_path();
        let mut child = Command::new(core_exec_path.to_string()).spawn()?;

        let (tx, rx) = mpsc::channel();
        self.child_rx = Some(rx);

        let output = child
            .stdout
            .take()
            .ok_or(anyhow!("todo: child output is missing"))?;
        thread::spawn(move || {
            let mut io = BufReader::new(output);

            let mut buf = String::new();
            loop {
                match io.read_line(&mut buf) {
                    Ok(_) => {
                        if let Err(err) = tx.send(buf.clone()) {
                            error!("todo: child error {err}");
                            break;
                        }
                    }
                    Err(err) => {
                        error!("todo: child error {err}");
                        break;
                    }
                }
            }
        });
        self.child = Some(child);

        Ok(())
    }
}
