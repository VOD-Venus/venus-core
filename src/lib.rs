use std::{
    io::{self, BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

use anyhow::{anyhow, Ok as AOk};
use config::Config;
use consts::get_v2ray_exe_path;
use error::{log_err, VenusError, VenusResult};

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
    /// Spawn a thread to execute v2ray core binary
    pub fn spawn_core(&mut self) -> VenusResult<()> {
        let core_exec_path = get_v2ray_exe_path();
        let mut child = Command::new(core_exec_path.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let (tx, rx) = mpsc::channel();
        self.child_rx = Some(rx);

        let stdout = child.stdout.take().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "child stdout is empty",
        ))?;
        let stderr = child.stderr.take().ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "child stderr is empty",
        ))?;
        let child_handler = move || {
            let stdout_tx = tx.clone();

            let mut handlers = Vec::with_capacity(2);
            let stdout_handler = thread::spawn(move || {
                let mut lines = BufReader::new(stdout).lines();
                lines.try_for_each(|line| {
                    stdout_tx.send(line?)?;
                    AOk(())
                })?;
                AOk(())
            });
            let stderr_handler = thread::spawn(move || {
                let mut lines = BufReader::new(stderr).lines();
                lines.try_for_each(|line| {
                    tx.send(line?)?;
                    AOk(())
                })?;
                AOk(())
            });

            handlers.push(stdout_handler);
            handlers.push(stderr_handler);
            handlers
                .into_iter()
                .try_for_each(|handler| {
                    handler
                        .join()
                        .map_err(|err| anyhow!("child join failed {err:?}"))??;
                    AOk(())
                })
                .map_err(log_err)?;
            AOk(())
        };
        thread::spawn(child_handler);

        self.child = Some(child);
        Ok(())
    }

    /// Kill core process if exist
    pub fn kill_core(&mut self) -> VenusResult<()> {
        if let Some(core) = self.child.as_mut() {
            core.kill()?;
            Ok(())
        } else {
            Err(VenusError::Core("core not running".into()))
        }
    }

    /// Kill core and spawn new one
    pub fn restart(&mut self) -> VenusResult<()> {
        self.kill_core()?;
        self.spawn_core()?;
        Ok(())
    }
}
