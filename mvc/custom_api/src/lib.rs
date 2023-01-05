use chrono::{Date, DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum UserLevel {
    CLIENT,
    ADMIN,
    MODER,
}

impl UserLevel {
    fn code(level: &UserLevel) -> u8 {
        return *level as u8;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPersonalDataDto {
    pub id: String,
    pub username: String,
    pub birth_date: NaiveDate,
    pub level: UserLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegistrationDto {
    pub id: String,
    pub username: String,
    pub password: String,
    pub birth_date: NaiveDate,
    pub level: UserLevel,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub content: String,
}