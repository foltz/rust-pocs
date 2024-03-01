use std::env;
use std::ops::Deref;

#[derive(Debug)]
pub struct StreamEnv {

    pub heartbeat_timeout_millisecs: i64,
    pub schedule_interval_minutes: i64
}

impl StreamEnv {
    pub fn load() -> StreamEnv {

        let _ = dotenvy::dotenv();

        let env_key = "STREAM_HEARTBEAT_TIMEOUT_MILLISECS";
        let heartbeat_timeout_millisecs = env::var(env_key)
            .expect(format!("env-value: {env_key} is missing").deref())
            .parse::<i64>()
            .expect(format!("env-value: {env_key} should be an integer value").deref())
            ;

        // let env_key = "STREAM_SCHEDULE_INTERVAL_HOURS";
        // let schedule_interval_hours = env::var(env_key)
        //     .expect(format!("env-value: {env_key} is missing").deref())
        //     .parse::<i64>()
        //     .expect(format!("env-value: {env_key} should be an integer value").deref())
        //     ;

        let env_key = "STREAM_SCHEDULE_INTERVAL_MINUTES";
        let schedule_interval_minutes = env::var(env_key)
            .expect(format!("env-value: {env_key} is missing").deref())
            .parse::<i64>()
            .expect(format!("env-value: {env_key} should be an integer value").deref())
            ;

        println!("env::stream::heartbeat_timeout: {heartbeat_timeout_millisecs}");
        println!("env::stream::schedule_interval: {schedule_interval_minutes}");

        StreamEnv {
            heartbeat_timeout_millisecs,
            schedule_interval_minutes,
        }
    }
}