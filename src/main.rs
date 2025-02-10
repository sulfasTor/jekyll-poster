use std::path::PathBuf;

use clap::Parser;
use post::post::launch_editor;

pub mod post;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<PathBuf>,

    #[arg(short, long, default_value = "default")]
    layout: String,

    #[arg(short, long)]
    publish: bool,
}

fn main() {
    let args = Args::parse();
    match launch_editor(args.file, &args.layout) {
        Ok(filename) => println!("All fine. Wrote {filename}"),
        Err(err) => println!("{err}"),
    }
}
