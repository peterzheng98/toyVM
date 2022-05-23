mod toyVM;

use clap::Parser;
use log::{Level, warn, error, debug, info, trace};
use pretty_env_logger;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Java class files
    #[clap(short, long)]
    classfile: String,

    /// Run tests
    #[clap(short, takes_value = false)]
    tests: bool,
}

fn ParseCommandLines() -> Args{
    let args = Args::parse();
    trace!("classfile: {}", args.classfile);
    trace!("enable tests: {}", args.tests);
    args
}


fn main() {
    // Init logger
    pretty_env_logger::init();
    
    // Parse command lines
    let args = ParseCommandLines();

    // Enter VM executions
    toyVM::entry(args.classfile);
}
