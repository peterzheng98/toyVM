use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Java class files
    #[clap(short, long)]
    classfile: String,
}

fn main() {
    let args = Args::parse();
    println!("Classfiles: {}!", args.classfile);
}
