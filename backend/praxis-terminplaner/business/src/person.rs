use datalayer::person::PersonRepository;
use models::person::PersonModel;

use crate::base::Business;



pub struct PersonBusiness {
    pub repository: PersonRepository
}

impl Business<PersonRepository, PersonModel, String> for PersonBusiness {
    fn get_repository(&self) ->  &PersonRepository {
        &self.repository
    }
}
