use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use super::service::model::login_model::LoginModel;
use super::service::model::user_model::{UserModel,UserTrait};

pub fn change_money(user_data: &mut UserModel) -> bool {
    let file_path = "src/data/users.json";

    let mut data: serde_json::Value = if let Ok(mut file) = File::open(&file_path) {
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("Error reading file: {}", e);
            return false;
        }
        match serde_json::from_str(&contents) {
            Ok(parsed_data) => parsed_data,
            Err(e) => {
                eprintln!("Error parsing JSON data: {}", e);
                return false;
            }
        }
    } else {
        serde_json::Value::Array(Vec::new())
    };

    if !data.is_array() {
        eprintln!("JSON data is not an array");
        return false;
    }

    match serde_json::to_value(&user_data) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing data to JSON: {}", e);
            return false;
        }
    };

    if let Some(index) = data.as_array_mut().unwrap().iter_mut().position(|data| data["username"] == user_data.username) {
        data[index]["is_active"] = json!(user_data.is_active);
        data[index]["money"] = json!(user_data.get_money());
    } else {
        let new_user_data = json!({
            "username": user_data.username,
            "password": user_data.password,
            "is_active": user_data.is_active,
            "money": user_data.get_money()
        });
        data.as_array_mut().unwrap().push(new_user_data);
    }

    let updated_json = match serde_json::to_string_pretty(&data) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing data to JSON: {}", e);
            return false;
        }
    };

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return false;
        }
    };
    if let Err(e) = file.write_all(updated_json.as_bytes()) {
        eprintln!("Error writing to the file: {}", e);
        return false;
    }

    true
}
pub fn create_account(user_data: UserModel) -> bool {
    let file_path = "src/data/users.json";

    let mut data: serde_json::Value = if let Ok(mut file) = File::open(&file_path) {
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("Error reading file: {}", e);
            return false;
        }
        match serde_json::from_str(&contents) {
            Ok(parsed_data) => parsed_data,
            Err(e) => {
                eprintln!("Error parsing JSON data: {}", e);
                return false;
            }
        }
    } else {
        serde_json::Value::Array(Vec::new())
    };

    if !data.is_array() {
        eprintln!("JSON data is not an array");
        return false;
    }

    let user_data_json = match serde_json::to_value(&user_data) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing data to JSON: {}", e);
            return false;
        }
    };
    data.as_array_mut().unwrap().push(user_data_json);

    let updated_json = match serde_json::to_string_pretty(&data) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error serializing data to JSON: {}", e);
            return false;
        }
    };

    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return false;
        }
    };
    if let Err(e) = file.write_all(updated_json.as_bytes()) {
        eprintln!("Error writing to the file: {}", e);
        return false;
    }

    true
}
pub fn get_user_infos(username: String) -> Option<UserModel> {
    let file_path = "src/data/users.json";

    let mut contents = String::new();
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return None;
        }
    };

    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", e);
        return None;
    }

    let data: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
            eprintln!("Error parsing JSON data: {}", e);
            return None;
        }
    };

    if !data.is_array() {
        eprintln!("JSON data is not an array");
        return None;
    }

    for account in data.as_array().unwrap() {
        if let Some(account_username) = account.get("username").and_then(|u| u.as_str()) {
            if account_username == username {
                // Deserialize the matched account to UserModel and return it
                match serde_json::from_value::<UserModel>(account.clone()) {
                    Ok(user_model) => return Some(user_model),
                    Err(e) => {
                        eprintln!("Error parsing UserModel: {}", e);
                        return None;
                    }
                }
            }
        }
    }
    None
}
pub fn login_account(user_data: LoginModel) -> bool {
    let file_path = "src/data/users.json";

    let mut contents = String::new();
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return false;
        }
    };

    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", e);
        return false;
    }

    let data: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(parsed_data) => parsed_data,
        Err(e) => {
            eprintln!("Error parsing JSON data: {}", e);
            return false;
        }
    };

    if !data.is_array() {
        eprintln!("JSON data is not an array");
        return false;
    }

    for account in data.as_array().unwrap() {
        if let Some(username) = account.get("username").and_then(|u| u.as_str()) {
            if let Some(password) = account.get("password").and_then(|p| p.as_str()) {
                if username == user_data.username && password == user_data.password {
                    return true;
                }
            }
        }
    }

    false
}