use std::fs;
use std::io;
use std::path::Path;

use clap::{App, Arg};
use pajamas::fetch;
use serde_json::to_string_pretty;
use serde_json::Value;

fn main() -> io::Result<()> {
    let matches = App::new("Pajamas")
        .about("A tool for inspecting package.json files")
        .author("Dylan Baker <dylan@dlyan.net>")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .about("The path to the directory that contains the package.json file"),
        )
        .arg(
            Arg::new("path")
                .index(1)
                .about("The path to the value in question, e.g. foo.bar.baz"),
        )
        .get_matches();
    let dir = matches.value_of("arg").unwrap_or(".");
    let file_path = Path::join(Path::new(dir), "package.json").canonicalize()?;

    if !file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory '{:?}' does not exist", file_path),
        ));
    }

    let contents = fs::read_to_string(file_path)?;
    match serde_json::from_str::<Value>(&contents) {
        Ok(document) => {
            let path = matches.value_of("path");
            match fetch(path, &document) {
                Ok(result) => {
                    if result.is_string() {
                        println!("{}", result.as_str().unwrap());
                    } else {
                        println!("{}", to_string_pretty(result).unwrap());
                    };
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
    }

    Ok(())
}
