use std::env;
use custom_rest::run_server;
use log::{info};
use custom_common::{CustomAppContext, CustomAppProperty};

fn main() {
    let props = construct_properties();
    let context = CustomAppContext {
        props
    };
    logging_init();
    info!("Application is initialized");
    let _ = run_server(context);
}

fn logging_init() {
    env::set_var("RUST_LOG",
                 "custom_main=debug,custom_rest=debug,actix_web=debug,actix_server=info",
    );
    env_logger::init();
}

fn construct_properties() -> CustomAppProperty {
    return CustomAppProperty {
        port: 9090,
        db_user: "postgres".to_string(),
        db_password: "postgres".to_string(),
        db_uri: "custom".to_string(),
    };
}
