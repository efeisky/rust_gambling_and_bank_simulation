use crate::reader::create_account;

use super::model::user_model::UserModel;

pub fn register_user(user : UserModel) -> bool {
    return create_account(user);
}