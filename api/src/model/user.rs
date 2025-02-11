use derive_new::new;
use garde::Validate;
use kernel::model::id::UserId;
use serde::{Deserialize, Serialize};
use strum::VariantNames;

#[derive(Serialize, Deserialize, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum RoleName {
    Admin,
    User,
}

impl From<kernel::model::role::Role> for RoleName {
    fn from(role: kernel::model::role::Role) -> Self {
        match role {
            kernel::model::role::Role::Admin => Self::Admin,
            kernel::model::role::Role::User => Self::User,
        }
    }
}

impl From<RoleName> for kernel::model::role::Role {
    fn from(role_name: RoleName) -> Self {
        match role_name {
            RoleName::Admin => Self::Admin,
            RoleName::User => Self::User,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersResponse {
    pub items: Vec<UserResponse>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: RoleName,
}

impl From<kernel::model::user::User> for UserResponse {
    fn from(value: kernel::model::user::User) -> Self {
        let kernel::model::user::User {
            id,
            name,
            email,
            role,
        } = value;
        Self {
            id,
            name,
            email,
            role: role.into(),
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPasswordRequest {
    #[garde(length(min = 1))]
    current_password: String,
    #[garde(length(min = 1))]
    new_password: String,
}

#[derive(new)]
pub struct UpdateUserPasswordRequestWithUserId(UserId, UpdateUserPasswordRequest);

impl From<UpdateUserPasswordRequestWithUserId> for kernel::model::user::event::UpdateUserPassword {
    fn from(value: UpdateUserPasswordRequestWithUserId) -> Self {
        let UpdateUserPasswordRequestWithUserId(
            user_id,
            UpdateUserPasswordRequest {
                current_password,
                new_password,
            },
        ) = value;
        Self {
            user_id,
            current_password,
            new_password,
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[garde(length(min = 1))]
    name: String,
    #[garde(email)]
    email: String,
    #[garde(length(min = 1))]
    password: String,
}

impl From<CreateUserRequest> for kernel::model::user::event::CreateUser {
    fn from(value: CreateUserRequest) -> Self {
        let CreateUserRequest {
            name,
            email,
            password,
        } = value;
        Self {
            name,
            email,
            password,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRoleRequest {
    role: RoleName,
}

#[derive(new)]
pub struct UpdateUserRoleRequestWithUserId(UserId, UpdateUserRoleRequest);

impl From<UpdateUserRoleRequestWithUserId> for kernel::model::user::event::UpdateUserRole {
    fn from(value: UpdateUserRoleRequestWithUserId) -> Self {
        let UpdateUserRoleRequestWithUserId(user_id, UpdateUserRoleRequest { role }) = value;
        Self {
            user_id,
            role: role.into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookOwner {
    pub id: UserId,
    pub name: String,
}

impl From<kernel::model::user::BookOwner> for BookOwner {
    fn from(value: kernel::model::user::BookOwner) -> Self {
        let kernel::model::user::BookOwner { id, name } = value;
        Self { id, name }
    }
}
