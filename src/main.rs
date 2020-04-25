use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "clar",
    about = "Comment Lines Async Replacer: tool to editing your code comments as a separate Markdown files."
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    files_to_watch: Vec<PathBuf>,
}

fn main() {
    let opt: Opt = Opt::from_args();

    if opt.files_to_watch.is_empty() {
        println!("No input files given, exiting now. Run `clar --help` for details.");
        return;
    }

    println!("Files to watch:");
    for file in opt.files_to_watch {
        println!("  {}", file.to_str().unwrap());
    }
}
