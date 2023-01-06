use custom_common::CustomAppProperty;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbContext {
    pool: DbPool,
}

impl DbContext {
    fn close(&self) {}
}

pub fn init_pg_context(props: &CustomAppProperty) -> DbContext {
    let url = format!("{}", props.db_uri);
    let manager = ConnectionManager::<PgConnection>::new(url);
    return DbContext {
        pool: r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    };
}