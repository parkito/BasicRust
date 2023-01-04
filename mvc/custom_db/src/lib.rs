#![feature(map_try_insert)]

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;
use chrono::NaiveDate;


#[derive(Debug)]
pub struct RepoError {
    msg: String,
}

impl Display for RepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for RepoError {}

pub type RepoResult<T> = Result<T, RepoError>;

#[derive(Debug)]
pub enum UserLevel {
    CLIENT,
    ADMIN,
    MODER,
}

#[derive(Debug)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub password: String,
    pub birth_date: NaiveDate,
    pub level: UserLevel,
}

pub trait EntityRepo<E, K> {
    fn save(&mut self, entity: E) -> RepoResult<K>;

    fn find(&self, key: K) -> RepoResult<Option<E>>;

    fn update(&self, key: K, entity: E) -> RepoResult<E>;

    fn remove(&self, key: K) -> RepoResult<K>;

    fn find_all(&self) -> RepoResult<Vec<E>>;
}

pub struct UserCacheRepo {
    map: HashMap<String, UserEntity>,
}

impl EntityRepo<UserEntity, String> for UserCacheRepo {
    fn save(&mut self, entity: UserEntity) -> RepoResult<String> {
        match self.map.try_insert(entity.id.to_string(), entity) {
            Err(err) => Err(RepoError { msg: format!("User with id {} already exists", err.value.id) }),
            Ok(some) => Ok(some.id.to_string())
        }
    }

    fn find(&self, key: String) -> RepoResult<Option<UserEntity>> {
        todo!()
    }

    fn update(&self, key: String, entity: UserEntity) -> RepoResult<UserEntity> {
        todo!()
    }

    fn remove(&self, key: String) -> RepoResult<String> {
        todo!()
    }

    fn find_all(&self) -> RepoResult<Vec<UserEntity>> {
        todo!()
    }
}

