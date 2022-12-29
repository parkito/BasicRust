use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path};

fn main() {
    println!("Hello, world!");
}

struct FileWrapper {
    path: Box<Path>,
}

impl FileWrapper {
    fn read_file(&self) -> Vec<String> {
        let file = match File::open(self.path.as_ref()) {
            Ok(f) => f,
            Err(err) =>
                {
                    let file_name = self.path.file_name()
                        .expect("Cannot get filename")
                        .to_str()
                        .expect("Os string conversion failed");
                    panic!("Cannot open the file {} because of {}", file_name, err.to_string())
                }
        };
        let b_reader = BufReader::new(file);
        return match b_reader.lines().collect::<io::Result<Vec<String>>>() {
            Ok(l) => l,
            Err(err) => panic!("Cannot read file due to {}", err.to_string()),
        };
    }

    fn append(&self, line: &str) {
        let file = match OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(self.path.as_ref()) {
            Ok(f) => f,
            Err(err) => panic!("Cannot write to {} due to {}", self.file_name(), err.to_string())
        };
        let mut b_writer = BufWriter::new(file);
        b_writer.write_all(line.as_bytes()).expect("Cannot write to file")
    }

    fn file_name(&self) -> &str {
        return self.path.file_name()
            .expect("Cannot get filename")
            .to_str()
            .expect("Os string conversion failed");
    }

    fn remove(&self) {
        fs::remove_file(self.path.as_ref()).expect("File cannot be removed");
    }
}

#[test]
fn write_and_read_file() {
    let content = "something";
    let wrapper = FileWrapper { path: Box::from(Path::new("/tmp/1.txt")) };

    wrapper.append(content);

    let res = wrapper.read_file();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], content);

    wrapper.remove()
}



