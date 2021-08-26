use structopt::StructOpt;


#[derive(StructOpt, Debug)]
pub struct Cli {
    pub pattern: String,
    pub path: Vec<String>,
}