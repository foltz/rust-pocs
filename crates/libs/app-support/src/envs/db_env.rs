use std::env;

#[derive(Debug)]
pub struct DbEnv {
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_ssl: String,
}

impl DbEnv {
    pub fn load() -> DbEnv {

        let _ = dotenvy::dotenv();

        let db_host = env::var("DB_HOST");
        let db_name = env::var("DB_NAME");
        let db_user = env::var("DB_USER");
        let db_pass = env::var("DB_PASS");
        let db_ssl = env::var("DB_SSL");

        if db_host.is_err() {
            panic!("missing env-value: DB_HOST");
        }
        if db_name.is_err() {
            panic!("missing env-value: DB_NAME");
        }
        if db_user.is_err() {
            panic!("missing env-value: DB_USER");
        }
        if db_pass.is_err() {
            panic!("missing env-value: DB_PASS");
        }
        if db_ssl.is_err() {
            panic!("missing env-value: DB_SSL");
        }

        DbEnv {
            db_host: db_host.unwrap(),
            db_name: db_name.unwrap(),
            db_user: db_user.unwrap(),
            db_pass: db_pass.unwrap(),
            db_ssl: db_ssl.unwrap(),
        }
    }
}