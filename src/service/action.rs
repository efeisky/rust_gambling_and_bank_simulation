use super::{super::reader::change_money, model::user_model::UserModel};
pub fn action_money(active_user_data : &mut UserModel) -> bool {
    change_money(active_user_data)
}