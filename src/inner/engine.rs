use std::{
    env,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, RecvTimeoutError, Sender},
    },
    thread::JoinHandle,
    time::Duration,
};

use anyhow::{Context, Result};

use notify_debouncer_mini::{DebounceEventResult, DebouncedEvent, new_debouncer};

use crate::{
    configs::Configs,
    inner::{Executor, WatchEvents},
};

pub struct Engine {
    config: Configs,
    sender: Sender<WatchEvents>,
    executor: Option<JoinHandle<()>>,
}

impl Engine {
    pub fn init(config: Configs) -> Self {
        let (tx, rx): (Sender<WatchEvents>, Receiver<WatchEvents>) = mpsc::channel();
        let executor = Executor::init(config.clone(), rx).execute();
        Self {
            config,
            sender: tx,
            executor: Some(executor),
        }
    }
    pub fn start(&mut self) -> Result<()> {
        let path = &self.config.dir;
        let shutdown = self.install_shutdown_handler()?;
        match env::set_current_dir(path) {
            Ok(_) => {}
            Err(_) => eprintln!("The path '{}' does not exists", path.display()),
        }
        if self.config.verbose {
            println!(
                "Watching all files in directory {} with extension(s) of {}",
                path.to_string_lossy(),
                self.config.watch.join(", ")
            );
        }
        let (tx, rx): (Sender<DebounceEventResult>, Receiver<DebounceEventResult>) =
            mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(200), tx)
            .with_context(|| "Something went wrong while creating a debouncer")?;
        debouncer
            .watcher()
            .watch(path, notify::RecursiveMode::Recursive)
            .with_context(|| format!("Could not watch the directory '{}'", path.display()))?;

        while !shutdown.load(Ordering::SeqCst) {
            match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(Ok(e)) => self.handle_event(e),
                Ok(Err(err)) => eprintln!("Error while watching: {}", err),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            };
        }

        let _ = self.sender.send(WatchEvents::Quit);
        if let Some(h) = self.executor.take() {
            h.join().expect("Executor panicked")
        }
        Ok(())
    }

    fn install_shutdown_handler(&self) -> Result<Arc<AtomicBool>> {
        let shutdown = Arc::new(AtomicBool::new(false));
        let handler_shutdown = Arc::clone(&shutdown);
        let handler_sender = self.sender.clone();

        ctrlc::set_handler(move || {
            if !handler_shutdown.swap(true, Ordering::SeqCst) {
                let _ = handler_sender.send(WatchEvents::Quit);
            }
        })
        .with_context(|| "Could not install Ctrl-C handler")?;

        Ok(shutdown)
    }
    fn handle_event(&self, e: Vec<DebouncedEvent>) {
        let valid = self.is_any_valid(&e);
        if valid {
            match self.sender.send(WatchEvents::FileModified) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("The executing thread has been disconnected: {}", e)
                }
            }
        }
    }
    fn is_this_valid(&self, p: &PathBuf) -> bool {
        if p.is_file()
            && let Some(ext) = p.extension()
        {
            for e in &self.config.watch {
                if e.as_str() == ext {
                    return true;
                }
            }
        }
        false
    }
    fn is_any_valid(&self, e: &Vec<DebouncedEvent>) -> bool {
        for event in e {
            if self.is_this_valid(&event.path) {
                return true;
            }
        }
        false
    }
}
