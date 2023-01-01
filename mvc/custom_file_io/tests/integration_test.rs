mod custom_file_io_tests {
    use std::sync::Mutex;
    use custom_file_io::{FileAppender, FileIoFactory, FileManager};

    #[test]
    fn write_and_read_file() {
        let expected_content = "something";
        let file = "/tmp/1.txt";

        let appender = FileAppender { writer: Mutex::new(FileIoFactory::create_buf_writer(file)) };

        appender.append(expected_content);

        let found_content = FileManager::read_file(file);
        assert_eq!(found_content.len(), 1);
        assert_eq!(found_content[0], expected_content);

        FileManager::remove(file)
    }
}