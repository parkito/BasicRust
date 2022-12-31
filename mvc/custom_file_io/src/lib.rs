use std::{fs, io};
use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path};

//todo use internal state for mantaining buffer
pub struct FileAppender {
    pub writer: BufWriter<File>,
}

impl FileAppender {
    pub fn append(&mut self, line: &str) {
        let ln = format!("{}\n", line);
        self.writer.write_all(ln.as_bytes()).expect("Cannot write to log file");
        self.writer.flush().expect("Cannot flush file");
    }
}

pub struct FileManager;

impl FileManager {
    pub fn read_file(file_uri: &str) -> Vec<String> {
        let reader = FileIoFactory::create_buf_reader(file_uri);
        let v = match reader.lines().collect::<io::Result<Vec<String>>>() {
            Ok(l) => l,
            Err(err) => panic!("Cannot read file due to {}", err.to_string()),
        };
        return v;
    }

    pub fn remove(file_uri: &str) {
        fs::remove_file(Path::new(file_uri)).expect("File cannot be removed");
    }
}

pub struct FileIoFactory;

impl FileIoFactory {
    pub fn create_buf_writer(file_uri: &str) -> BufWriter<File> {
        let path = Path::new(file_uri);
        let file = match OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path) {
            Ok(f) => f,
            Err(err) => panic!("Cannot write to {} due to {}", file_uri, err.to_string())
        };
        return BufWriter::new(file);
    }

    fn create_buf_reader(file_uri: &str) -> BufReader<File> {
        let path = Path::new(file_uri);
        let file = match File::open(path) {
            Ok(f) => f,
            Err(err) =>
                {
                    panic!("Cannot open the file {} because of {}", file_uri, err.to_string())
                }
        };
        return BufReader::new(file);
    }
}
