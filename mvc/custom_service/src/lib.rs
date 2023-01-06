use uuid::Uuid;
use custom_api::{UserLevel, UserPersonalDataDto, UserRegistrationDto};
use custom_api::UserLevel::{ADMIN, CLIENT, MODER};
use custom_db::{CustomResult, EntityRepo, UserEntity, UserAccessLevel};
use custom_db::UserAccessLevel::CanCreateUsers;
use UserAccessLevel::{CanAdminister, CanMessage};

pub trait Converter<E, D> {
    fn to_entity(dto: &D) -> E;

    fn to_dto(entity: &E) -> D;
}

struct UserPermissionConverter;

struct UserRegistrationDtoConverter;

struct UserPersonalDataDtoConverter;

impl Converter<UserAccessLevel, UserLevel> for UserPermissionConverter {
    fn to_entity(dto: &UserLevel) -> UserAccessLevel {
        return match dto {
            CLIENT => CanMessage,
            ADMIN => CanAdminister,
            MODER => CanCreateUsers
        };
    }

    fn to_dto(entity: &UserAccessLevel) -> UserLevel {
        return match entity {
            CanMessage => CLIENT,
            CanAdminister => ADMIN,
            CanCreateUsers => MODER
        };
    }
}

impl Converter<UserEntity, UserRegistrationDto> for UserRegistrationDtoConverter {
    fn to_entity(dto: &UserRegistrationDto) -> UserEntity {
        return UserEntity {
            id: Uuid::new_v7().to_string(),
            username: dto.username.to_string(),
            password: dto.password.to_string(),
            birth_date: dto.birth_date,
            level: UserPermissionConverter::to_entity(&dto.level),
        };
    }

    fn to_dto(entity: &UserEntity) -> UserRegistrationDto {
        //not implemented
        todo!()
    }
}

impl Converter<UserEntity, UserPersonalDataDto> for UserPersonalDataDtoConverter {
    fn to_entity(dto: &UserPersonalDataDto) -> UserEntity {
        return UserEntity {
            id: dto.id.to_string(),
            username: dto.username.to_string(),
            password: "".to_string(),
            birth_date: dto.birth_date,
            level: UserPermissionConverter::to_entity(&dto.level),
        };
    }

    fn to_dto(entity: &UserEntity) -> UserPersonalDataDto {
        return UserPersonalDataDto {
            id: entity.id.to_string(),
            username: entity.username.to_string(),
            birth_date: entity.birth_date,
            level: UserPermissionConverter::to_dto(&entity.level),
        };
    }
}

pub trait UserService {
    fn save_user(&mut self, user: UserRegistrationDto) -> CustomResult<String>;

    fn find_user(&mut self, id: String) -> CustomResult<Option<&UserPersonalDataDto>>;
}

pub struct UserServiceImp {
    repo: dyn EntityRepo<UserEntity, String>,
}

impl UserService for UserServiceImp {
    fn save_user(&mut self, user: UserRegistrationDto) -> CustomResult<String> {
        self.repo.save(UserRegistrationDtoConverter::to_entity(&user))
    }

    fn find_user(&mut self, id: String) -> CustomResult<Option<&UserPersonalDataDto>> {
        return match self.repo.find(id)? {
            None => Ok(None),
            Some(entity) => Ok(Option::from(&UserPersonalDataDtoConverter::to_dto(entity))),
        };
    }
}