extern crate clap;

use clap::{App, Arg};

use std::path::PathBuf;

#[derive(Debug)]
struct WatchList {
    files: Vec<PathBuf>
}

impl WatchList {
    pub fn new<'a, I>(files_and_dirs: I) -> std::io::Result<WatchList>
        where I: Iterator<Item = &'a str>
    {
        let mut result = WatchList {
            files: Vec::new()
        };

        for path_str in files_and_dirs {
            let path = PathBuf::from(path_str);
            result.add_entry(path)?;
        }

        result.files.sort();

        return Ok(result);
    }

    fn add_entry(&mut self, path: PathBuf) -> std::io::Result<()> {
        if path.is_file() {
            self.files.push(path);
        } else {
            let nested_entries = path.read_dir()?;
            for nested_entry in nested_entries {
                let nested_path = nested_entry?.path();
                self.add_entry(nested_path)?;
            }
        }

        return Ok(());
    } 
}

fn main() -> std::io::Result<()> {
    let args = App::new("CLAR: Comment Lines Async Replacer")
        .author("Roman Kolesnev, rvkolesnev@gmail.com")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Allows you to edit your documentation code comments as a separate markdown files.")
        .arg(
            Arg::with_name("files_and_dirs")
                .help("files and directories to watch")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let files_and_dirs = args.values_of("files_and_dirs").unwrap();
    let watch_list = WatchList::new(files_and_dirs)?;

    println!("Files: ");
    for file in watch_list.files {
        println!("  {:?}", file);
    }

    return Ok(());
}
