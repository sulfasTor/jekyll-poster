use std::{path::PathBuf, process};

use clap::Parser;
use post::{edit::create_post, publish::add_and_commit_post};

pub mod post;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    post_id_name: String,
    project_path: PathBuf,

    #[arg(short, long, default_value = "default")]
    layout: String,

    #[arg(short, long)]
    publish: bool,

    #[arg(short, long)]
    draft: bool,
}

fn main() {
    let args = Args::parse();

    let post_path = match create_post(
        &args.post_id_name,
        &args.project_path,
        &args.layout,
        args.draft,
    ) {
        Ok(filename_path) => filename_path,
        Err(err) => {
            eprint!("{}", err);
            process::exit(1);
        }
    };

    if args.publish {
        match add_and_commit_post(&post_path, &args.project_path, args.draft) {
            Ok(_) => println!("Succesfully published new post."),
            Err(err) => {
                eprintln!("{err}");
                process::exit(1);
            }
        }
    }
}
