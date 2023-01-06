pub struct CustomAppProperty {
    pub port: i32,
    pub db_user: String,
    pub db_password: String,
    pub db_uri: String,
}

pub struct CustomAppContext {
    pub props: CustomAppProperty
}