extern crate clap;

use clap::{App, Arg};

fn main() {
    let args = App::new("CLAR: Comment Lines Async Replacer")
        .author("Roman Kolesnev, rvkolesnev@gmail.com")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Allows you to edit your documentation code comments as a separate markdown files.")
        .arg(
            Arg::with_name("files")
                .help("files to watch")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let files = args.values_of("files").unwrap();

    println!("files to watch:");
    for file in files {
        println!("  {}", file);
    }
}
