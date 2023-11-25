use datalayer::user::UserRepository;
use entities::user::User;
use uuid::Uuid;
use datalayer::base::Repository;

use crate::base::Business;

pub struct UserBusiness {

    pub repository: UserRepository

}


impl Business<User, String> for UserBusiness {

    fn insert(&self, user: &User) -> Result<usize, diesel::result::Error> {

        let password = user.password.to_owned();

        let user_with_hash = User {
            password: Some(authentication::hash(&password.unwrap())),
            user_id: Uuid::new_v4().to_string(),
            username: user.username.to_owned() 
        };

        self.repository.insert(&user_with_hash)

    }

    fn update(&self,entity: &User)-> Result<usize, diesel::result::Error> {

        self.repository.update(entity)

    }

    fn delete(&self, id: &String)-> Result<usize, diesel::result::Error> {

        self.repository.delete(id)

    }

    fn get(&self, id: &String) -> Result<User, diesel::result::Error> {

        self.repository.get(id)
    }

}
