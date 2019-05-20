use regex::Regex;
use std::fs;
use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error>{
  let dir = fs::read_dir("./views")?;
  for entry in dir {
    let entry = entry?;
    if let Ok(file_type) = entry.file_type() {
      if file_type.is_file() && entry.path().extension() == Some(std::ffi::OsStr::new("html")) {
        let mut file = fs::File::open(entry.path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents = contents.replace("\"", "\\\"");
        let re = Regex::new(r"(?ms)\{(?P<v>[a-zA-Z._]+) \|(?P<e>[a-zA-Z]+)\| \[(?P<c>.*)\]\}").unwrap();
        while re.is_match(&contents) {
          contents = re.replace_all(&contents, "\"), Value(Array(\"$v\", \"$e\", &[Litteral(\"$c\")])), Litteral(\"").to_string();
        }
        let re = Regex::new(r"\{(?P<v>[a-zA-Z._]+)\}").unwrap();
        while re.is_match(&contents) {
          contents = re.replace_all(&contents, "\"), Value(Content(\"$v\")), Litteral(\"").to_string();
        }
        println!("&[Litteral(\"{}\")]", contents);
        let mut f = fs::File::create(&format!("{}.in", entry.path().as_path().to_str().unwrap()))?;
        f.write_all(format!("&[Litteral(\"{}\")]", contents).as_bytes());
      }
    }
  }
  Ok(())
}