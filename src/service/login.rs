use crate::reader::login_account;

use super::model::login_model::LoginModel;

pub fn login_user(user : LoginModel) -> bool {
    return login_account(user);
}