// TODO: Remove this Ugly file and interpret views files in views.rs at compile time
use regex::Regex;
use std::fs::{self, DirEntry, ReadDir};
use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error> {
    println!("cargo:rerun-if-changed=views/*.html");

    let dir = fs::read_dir("./views")?;
    dir_recurse(dir)?;
    Ok(())
}

fn dir_recurse(dir: ReadDir) -> Result<(), std::io::Error> {
    for entry in dir {
        let entry = entry?;

        if let Ok(file_type) = entry.file_type() {
            if file_type.is_file() {
                compile(entry)?;
            } else if file_type.is_dir() {
                dir_recurse(fs::read_dir(entry.path())?)?;
            }
        }
    }
    Ok(())
}

fn compile(entry: DirEntry) -> Result<(), std::io::Error> {
    if entry.path().extension() == Some(std::ffi::OsStr::new("html")) {
        let mut file = fs::File::open(entry.path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents = contents.replace("\"", "\\\"");

        let re = Regex::new(r"(?ms)\{(?P<v>[a-zA-Z._]+) \|(?P<e>[a-zA-Z]+)\| \[").unwrap();
        while re.is_match(&contents) {
            contents = re
                .replace_all(&contents, "\"), Value(Array(\"$v\", \"$e\", &[Litteral(\"")
                .to_string();
        }

        let re = Regex::new(r"(?ms)\]\}").unwrap();
        while re.is_match(&contents) {
            contents = re.replace_all(&contents, "\")])), Litteral(\"").to_string();
        }

        let re = Regex::new(r"\{(?P<v>[a-zA-Z._]+)\}").unwrap();
        while re.is_match(&contents) {
            contents = re
                .replace_all(&contents, "\"), Value(Content(\"$v\")), Litteral(\"")
                .to_string();
        }

        let mut f = fs::File::create(&format!("{}.in", entry.path().as_path().to_str().unwrap()))?;
        f.write_all(format!("&[Litteral(\"{}\")]", contents).as_bytes())?;
    }
    Ok(())
}
