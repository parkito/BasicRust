use std::{fmt, fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path};

struct FileAppender {
    writter: BufWriter<File>,
}

impl FileAppender {
    fn append(&mut self, line: &str) {
        self.writter.write_all(line.as_bytes()).expect("Cannot write to log file");
        self.writter.flush().expect("Cannot flush file");
    }
}

struct FileManager;

impl FileManager {
    fn read_file(file_uri: &str) -> Vec<String> {
        let reader = FileIoFactory::create_buf_reader(file_uri);
        let v = match reader.lines().collect::<io::Result<Vec<String>>>() {
            Ok(l) => l,
            Err(err) => panic!("Cannot read file due to {}", err.to_string()),
        };
        return v;
    }

    fn remove(file_uri: &str) {
        fs::remove_file(Path::new(file_uri)).expect("File cannot be removed");
    }
}

struct FileIoFactory;

impl FileIoFactory {
    fn create_buf_writer(file_uri: &str) -> BufWriter<File> {
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


#[test]
fn write_and_read_file() {
    let expected_content = "something";
    let file = "/tmp/1.txt";

    let mut appender = FileAppender { writter: FileIoFactory::create_buf_writer(file) };

    appender.append(expected_content);

    let found_content = FileManager::read_file(file);
    assert_eq!(found_content.len(), 1);
    assert_eq!(found_content[0], expected_content);

    FileManager::remove(file)
}



