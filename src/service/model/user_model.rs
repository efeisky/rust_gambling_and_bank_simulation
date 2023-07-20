use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct UserModel {
    pub username: String,
    pub password: String,
    pub is_active: bool,
    pub money: i16,
}

pub fn create_user_model(username: &String, password: String) -> UserModel {
    UserModel {
        username: username.to_string(),
        password: password,
        is_active: true,
        money: 0,
    }
}

pub trait UserTrait {
    fn get_username(&self) -> &String;
    fn get_password(&self) -> &String;
    fn get_account_status(&self) -> &bool;
    fn get_money(&self) -> &i16;
    fn set_money(&mut self, money: i16);
    fn default() -> UserModel;
}

impl UserTrait for UserModel {
    fn default() -> UserModel {
        UserModel {
            username: "".to_string(),
            password: "".to_string(),
            is_active: true,
            money: 0,
        }
    }
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_account_status(&self) -> &bool {
        &self.is_active
    }

    fn get_money(&self) -> &i16 {
        &self.money
    }
    
    fn set_money(&mut self, money: i16) {
        self.money = money;
    }
}
