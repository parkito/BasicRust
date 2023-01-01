mod custom_logger_test {
    use custom_file_io::FileManager;
    use custom_logger::{Level, LogFactory, LogSetting, LogType};
    use custom_logger::Level::TRACE;
    use LogType::FILE;

    #[test]
    fn levels_have_correct_names() {
        for level in Level::VALUES {
            assert_eq!(level.0.to_str(), level.1);
        };
    }

    #[test]
    fn print_logs_in_std() {
        let logger = LogFactory::build("first_logger");
        logger.log(Level::INFO, "first message");
    }

    #[test]
    fn print_logs_to_file() {
        let file = "/tmp/log.log";
        LogFactory::set_log_setting(
            LogSetting { log_type: FILE, file_path: Some(file.to_string()), root_level: Some(TRACE) }
        );
        let logger = LogFactory::build("first_logger");

        logger.log(Level::INFO, "first message");
        logger.log(Level::INFO, "second message");
        logger.log(Level::INFO, "third message");

        let lines = FileManager::read_file(file);
        assert_eq!(lines.len(), 3);

        FileManager::remove(file);
    }
}