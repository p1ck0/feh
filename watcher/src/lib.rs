use notify::{Watcher, RecursiveMode, watcher};

use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};
use std::process::{Command, Child, ChildStdout};
use std::option::Option;
use std::thread;
use std::error::Error;


pub struct Notifier {
    watcher: notify::INotifyWatcher,
    channelrx: Receiver<notify::DebouncedEvent>,
    command: Command,
    child: Child,
}

impl Notifier {
    pub fn new(paths: Vec<String>, mut command: Command) -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(0))?;
        for path in paths {
            watcher.watch(path, RecursiveMode::NonRecursive)?;
        }

        let mut child = command.spawn()?;
        child.kill();
        
        Ok(Self{ 
            watcher: watcher,
            channelrx: rx,
            command: command,
            child: child,
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.child = self.command.spawn()?;
        let out = self.child.stdout.take();
        Self::output(out);
        Self::handler(self);
        Ok(())
    }

    fn handler(&mut self) {
        loop {
            let event = self.channelrx.recv();
            match event {
                event => Self::reload(self),
            };
        };
    }

    fn output(stdout: Option<ChildStdout>) {
        thread::spawn(move || {
            match stdout {
                Some(_) => println!("{:?}", stdout),
                None => print!(""),
            };
        }).join();
    }

    fn reload(&mut self) {
        self.child.kill();
        self.child = self.command.spawn().expect("");
        let out = self.child.stdout.take();
        Self::output(out);
    }
}