use datalayer::user::UserRepository;
use models::user::UserModel;
use crate::base::Business;

pub struct UserBusiness {

    pub repository: UserRepository

}


impl Business<UserRepository, UserModel, String> for UserBusiness {

    fn get_repository(&self) -> &UserRepository {
        &self.repository
    }

}
