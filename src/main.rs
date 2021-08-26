use structopt::StructOpt;
use cli::Cli;
use std::process::Command;
use watcher::Notifier;

  
fn main() {

    let args = Cli::from_args();

    let mut proc = Command::new("go");
    proc.arg("run");
    proc.arg("main.go");
    
    let mut nt = Notifier::new(args.path, proc).expect("err");
    nt.start();
}