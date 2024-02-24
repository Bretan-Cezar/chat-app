use std::fs::{File, read};
use std::io::BufReader;

pub struct Config {

    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry: u64,
    pub ws_tick: u64,
    pub reg_tick: u64
}

impl Config {

    pub fn init() -> Config {

        let db_config_path = std::env::var("DB_CONFIG").expect("a config file path for the database must be provided with the DB_CONFIG key");
        let jwt_secret = std::env::var("JWT_SECRET").expect("a JWT secret must be provided with the JWT_SECRET key");
        let jwt_expiry = std::env::var("JWT_EXPIRY")
            .expect("a JWT duration (minutes) must be provided with the JWT_EXPIRY key")
            .parse::<u64>().expect("Invalid JWT duration");

        let ws_tick = std::env::var("WS_TICK")
            .expect("a websocket tick value (milliseconds) must be provided with the WS_TICK key")
            .parse::<u64>().expect("Invalid WS tick value");

        let reg_tick = std::env::var("REG_TICK")
            .expect("a tick value (milliseconds) for the public registration endpoint must be provided with the REG_TICK key")
            .parse::<u64>().expect("Invalid registration tick value");

        let f = File::open(db_config_path).expect("database config file not found");
        let props = read(BufReader::new(f)).expect("Error on reading database config file");

        let database_url = format!(
            "postgresql://{}:{}@{}:{}/{}?currentSchema={}",
            props.get("user").expect("user not found in database config file"),
            props.get("password").expect("password not found in database config file"),
            props.get("host").expect("host not found in database config file"),
            props.get("port").expect("port not found in database config file"),
            props.get("dbname").expect("dbname not found in database config file"),
            props.get("schema").expect("schema not found in database config file")
        );

        Config {
            database_url,
            jwt_secret,
            jwt_expiry,
            ws_tick,
            reg_tick
        }
    }
}