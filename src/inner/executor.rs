use anyhow::{Context, Result};
use command_group::{CommandGroup, GroupChild};

use crate::configs::Configs;
use std::{
    cell::RefCell,
    process::Command,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

pub enum WatchEvents {
    FileModified,
    Quit,
}

pub struct Executor {
    cfg: Configs,
    child: RefCell<Option<GroupChild>>,
    receiver: Receiver<WatchEvents>,
}

impl Executor {
    pub fn init(cfg: Configs, r: Receiver<WatchEvents>) -> Self {
        Self {
            cfg,
            child: RefCell::new(None),
            receiver: r,
        }
    }
    pub fn execute(self) -> JoinHandle<()> {
        thread::spawn(move || {
            for e in &self.receiver {
                match e {
                    WatchEvents::Quit => {
                        self.stop_child();
                        break;
                    }
                    WatchEvents::FileModified => {
                        if let Err(e) = self.run() {
                            eprintln!("Error while running command: {}", e);
                        }
                    }
                }
            }
            self.stop_child();
        })
    }
    fn run(&self) -> Result<()> {
        self.stop_child();
        if self.cfg.verbose {
            println!(
                "Running: {} {}",
                self.cfg.cmd.cmd,
                self.cfg.cmd.args.join(" ")
            )
        }
        let mut cmd = Command::new(&self.cfg.cmd.cmd);
        cmd.args(&self.cfg.cmd.args);
        let c = cmd
            .group_spawn()
            .with_context(|| "Something went wrong while starting the command")?;
        *self.child.borrow_mut() = Some(c);
        Ok(())
    }
    fn stop_child(&self) {
        if let Some(mut c) = self.child.borrow_mut().take() {
            if self.cfg.verbose {
                println!("Stopping previous process group");
            }
            if let Err(e) = c.kill() {
                eprintln!("Error while stopping process group: {}", e);
            }
            if let Err(e) = c.wait() {
                eprintln!("Error while waiting for process group to exit: {}", e);
            }
        }
    }
}
