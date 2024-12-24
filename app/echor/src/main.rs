use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about = None)]
struct Args {
    #[arg(required = true)]
    input: Vec<String>,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.input.join(" "));
}
