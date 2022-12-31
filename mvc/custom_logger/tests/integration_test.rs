mod custom_logger_test {
    use logging::{Level, LogFactory};

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
}