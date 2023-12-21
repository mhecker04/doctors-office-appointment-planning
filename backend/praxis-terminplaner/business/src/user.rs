use async_trait::async_trait;
use datalayer::{user::UserRepository, error::RepositoryError, search::SearchRepository};
use models::user::UserModel;
use crate::{base::Business, search::SearchBusiness, implement_search_business};

pub struct UserBusiness {

    pub repository: UserRepository

}


impl Business<UserRepository, UserModel, String> for UserBusiness {

    fn get_repository(&self) -> &UserRepository {
        &self.repository
    }

}

implement_search_business!(UserBusiness, UserModel, String);