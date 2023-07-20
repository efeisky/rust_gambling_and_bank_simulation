use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
    pub is_active: bool,
}

pub fn create_login_model(username: &String, password: String) -> LoginModel {
    LoginModel {
        username: username.to_string(),
        password: password,
        is_active: true,
    }
}

pub trait LoginTrait {
    fn get_username(&self) -> &String;
    fn get_password(&self) -> &String;
    fn get_account_status(&self) -> &bool;
}

impl LoginTrait for LoginModel {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_account_status(&self) -> &bool {
        &self.is_active
    }
}
