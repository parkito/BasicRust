#![feature(map_try_insert)]

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter, write};
use std::error::Error;
use chrono::NaiveDate;
use crate::RepoErrorType::LOGIC;

#[derive(Debug)]
pub struct CustomError {
    msg: String,
    error_type: RepoErrorType,
}

#[derive(Debug)]
pub enum RepoErrorType {
    LOGIC,
    DB,
    UNKDNOWN,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for CustomError {}

pub type CustomResult<T> = Result<T, CustomError>;

#[derive(Debug)]
pub enum UserAccessLevel {
    CanMessage,
    CanCreateUsers,
    CanAdminister,
}

#[derive(Debug)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub password: String,
    pub birth_date: NaiveDate,
    pub level: UserAccessLevel,
}

pub trait EntityRepo<E, K> {
    fn save(&mut self, entity: E) -> CustomResult<K>;

    fn find(&self, key: K) -> CustomResult<Option<&E>>;

    fn update(&self, key: K, entity: E) -> CustomResult<E>;

    fn remove(&self, key: K) -> CustomResult<K>;

    fn find_all(&self) -> CustomResult<Vec<E>>;
}

pub struct UserCacheRepo {
    map: HashMap<String, UserEntity>,
}

impl EntityRepo<UserEntity, String> for UserCacheRepo {
    fn save(&mut self, entity: UserEntity) -> CustomResult<String> {
        match self.map.try_insert(entity.id.to_string(), entity) {
            Err(err) => Err(CustomError { msg: format!("User with id {} already exists", err.value.id), error_type: LOGIC }),
            Ok(some) => Ok(some.id.to_string())
        }
    }

    fn find(&self, key: String) -> CustomResult<Option<&UserEntity>> {
        return match self.map.get(key.as_str()) {
            None => Ok(None),
            Some(some) => Ok(Option::Some(some))
        };
    }

    fn update(&self, key: String, entity: UserEntity) -> CustomResult<UserEntity> {
        todo!()
    }

    fn remove(&self, key: String) -> CustomResult<String> {
        todo!()
    }

    fn find_all(&self) -> CustomResult<Vec<UserEntity>> {
        todo!()
    }
}

