use std::process;
use std::thread::sleep;
use std::time::Duration;
mod reader;
mod gambling;
mod input;
mod service{
    pub mod login;
    pub mod register;
    pub mod action;
    pub mod model{
        pub mod process_model;
        pub mod login_model;
        pub mod user_model;
    }
}
use service::login::login_user;
use service::model::user_model::UserModel;
use service::register::register_user;
use service::model::process_model::ProcessType;
use service::model::process_model::ProcessResult;
use service::model::login_model::create_login_model;
use service::model::user_model::create_user_model;
use gambling::play_gambling;

use crate::reader::get_user_infos;
use crate::service::model::user_model::UserTrait;

fn main() {

    println!("Program'a hoş geldiniz. Yapmak istediğiniz işlemi aşağıda seçebilirsiniz.\n 1 - Giriş Yap\n 2 - Kayıt Ol");
    let stage = input::text_input("Devam etmek için yapmak istediğiniz işlem numarasını giriniz :", false);
    let stage: u8 = stage.trim().parse().unwrap();
    println!("\n");
    let result = match stage {
        1 => login(),
        2 => register(),
        _ => ProcessResult::default(ProcessType::Invalid)
    };
    if ProcessType::Invalid != result.process && !result.status{
        println!("\x1B[5;31mProgramdan çıkılıyor...\x1B[0m");
        sleep(Duration::new(2, 0));
        process::exit(0);
    }
    println!("\x1B[2J\x1B[1;1H\x1B[0m");

    let mut user_info = match get_user_infos(result.username) {
        Some(info) => info,
        None => {
            eprintln!("\x1B[5;31mBir Hata Oluştu...\x1B[0m");
            std::thread::sleep(std::time::Duration::from_secs(2));
            panic!("Hata oluştuğu için program sonlandırılıyor.");
        }
    };
    loop {
        println!("-------------------------");
        println!("Hoş Geldin, {}",&user_info.get_username());

        loop {
            println!("1 - Banka Bilgilerim'e Bak\n2 - Kumar Oyna\n3 - Programı Kapat");
            let stage = input::text_input("Devam etmek için yapmak istediğiniz işlem numarasını giriniz :", false);
            match stage.trim().parse::<u8>() {
                Ok(1) => {
                    bank_info(&user_info);
                },
                Ok(2) => {
                    play_gambling(&mut user_info);
                },
                Ok(3) => {
                    process::exit(0);
                },
                _ => {
                    println!("\x1B[5;31mHatalı Numara Girdiniz!\x1B[0m");
                }
            }
        }
    }

}

fn login() -> ProcessResult {
    println!("\x1B[2J\x1B[1;1H\x1B[0m");
    println!("Bankomo  &&  Giriş İşlemi");
    
    println!("\n");

    let username = input::text_input("Kullanıcı Adı: ",false);

    let password = input::text_input("Şifre: ",true);

    println!("\n");
    
    let user =  create_login_model(&username, password);
    let status = login_user(user);
    if status {
        ProcessResult {
            process: ProcessType::Login,
            status: true,
            username
        }
    }else{
        println!("\x1B[31mHatalı Giriş \x1B[0m");
        return ProcessResult::default(ProcessType::Login);
    }
    
}

fn register() -> ProcessResult {
    println!("\x1B[2J\x1B[1;1H\x1B[0m");
    println!("Bankomo  &&  Kayıt İşlemi");
    
    loop {
        println!("\n");

        let username = input::text_input("Kullanıcı Adı: ",false);

        let password = input::text_input("Şifre: ",true);
        let retry_password = input::text_input("Yeniden Şifre: ",true);
        if password != retry_password {
            println!("\x1B[5;31mŞifreler Aynı Olmalıdır!\x1B[0m");
            continue;
        }
        println!("\n");
        let user =  create_user_model(&username, password);
        let status =  register_user(user);
        if status {
            return ProcessResult {
                process: ProcessType::Register,
                status: true,
                username
            };
        }else{
            println!("\x1B[31mHatalı Giriş \x1B[0m");
            return ProcessResult::default(ProcessType::Register);
        }
    }
    
}

fn bank_info(user : &UserModel){
    println!("\x1B[2J\x1B[1;1H\x1B[0m");
    println!("İsim : {}", user.get_username());
    println!("Şifre : {}", user.get_password());
    println!("Para : {} TL", user.get_money());
    if user.get_account_status() == &true {
        println!("Hesap Durumu : Aktif");
    }else{
        println!("Hesap Durumu : Pasif");
    }
    println!("-----------");
    loop {
        println!("1 - Geri Git");
        let stage = input::text_input("Devam etmek için yapmak istediğiniz işlem numarasını giriniz :", false);
        match stage.trim().parse::<u8>() {
            Ok(1) => {
                break;
            },
            _ => {
                println!("\x1B[5;31mHatalı Numara Girdiniz!\x1B[0m");
            }
        }
    }
}