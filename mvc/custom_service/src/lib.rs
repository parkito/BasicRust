use uuid::Uuid;
use custom_api::{UserPersonalDataDto, UserRegistrationDto};
use custom_db::{CustomError, CustomResult, EntityRepo, UserEntity};
use custom_db::UserLevel::CLIENT;

pub trait Converter<E, D> {
    fn to_entity(dto: D) -> E;

    fn to_dto(entity: E) -> D;
}

struct UserRegistrationDtoConverter;

struct UserPersonalDataDtoConverter;

impl Converter<UserEntity, UserRegistrationDto> for UserRegistrationDtoConverter {
    fn to_entity(dto: UserRegistrationDto) -> UserEntity {
        return UserEntity {
            id: Uuid::new_v8().to_string(),
            username: dto.username,
            password: dto.password,
            birth_date: dto.birth_date,
            level: CLIENT,
        };
    }

    fn to_dto(entity: UserEntity) -> UserRegistrationDto {
        todo!()
    }
}

pub trait UserService {
    fn save_user(&mut self, user: UserRegistrationDto) -> CustomResult<String>;

    fn find_user(&self, id: String) -> CustomResult<Option<UserPersonalDataDto>>;
}

pub struct UserServiceImp {
    repo: dyn EntityRepo<UserEntity, String>,
    // converter: dyn Converter<UserEntity, UserPersonalDataDto>,
}

impl UserService for UserServiceImp {
    fn save_user(&mut self, user: UserRegistrationDto) -> CustomResult<String> {
        self.repo.save(user);
        todo!()
    }

    fn find_user(&self, id: String) -> CustomResult<Option<UserPersonalDataDto>> {
        todo!()
    }
}