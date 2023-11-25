
extern crate diesel;


use entities::user::User;
use entities::schema::user::{self};
use diesel::prelude::*;

use crate::{insert_into, delete, update, get_by_id, fetch_entity};

use crate::base::Repository;

pub struct UserRepository {


}

impl Repository<User, String> for UserRepository {

    fn insert(&self, entity: &User) -> Result<usize, diesel::result::Error> {

        let connection = &mut entities::establish_connection();

        insert_into!(connection, user::table, entity)

    }

    fn update(&self, entity: &User) -> Result<usize, diesel::result::Error>{

        let connection = &mut entities::establish_connection();

        update!(connection, user::table, entity, &entity.user_id)

    }

    fn delete(&self, id: &String) -> Result<usize, diesel::result::Error> {
        let connection = &mut entities::establish_connection();

        delete!(connection, user::table, id)

    }

    fn get(&self, id: &String) -> Result<User, diesel::result::Error>{
        let connection = &mut entities::establish_connection();

        get_by_id!(connection, user::table, User::as_select, id)

    }

}

impl UserRepository {

    pub fn get_user_by_username(&self, username: &String) -> Result<User, diesel::result::Error>{

        let connection = &mut entities::establish_connection();

        fetch_entity!(connection, user::table, User::as_select, user::username.eq(username))

    }

}
