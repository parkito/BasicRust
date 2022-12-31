use custom_logger::Level::DEBUG;
use custom_logger::LogFactory;

fn main() {
    let logger = LogFactory::build("first_logger");
    logger.log(DEBUG, "hello");
}
