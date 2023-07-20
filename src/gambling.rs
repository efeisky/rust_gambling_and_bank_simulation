use crate::{input, service::{model::user_model::{UserModel, UserTrait}, action::action_money}};
use rand::Rng;
fn guess_number(user : &mut UserModel) {
    println!("\x1B[2J\x1B[1;1H\x1B[0m");
    println!("Sayı Tahmin Oyunu");
    loop {
        println!("-----------");
        print!("\n");
        println!("1 ile 50 arasında bir sayı giriniz ( Her bilmede 50 TL / Her kaybetmede -10 TL )");
        let guess_user = input::text_input("Sayı Giriniz ( çıkış : q ) :", false);
        let guess_number = match guess_user.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                if guess_user == String::from("q") {
                    break;
                }
                println!("\x1B[5;31mGeçerli bir sayı girmediniz!\x1B[0m\n");
                return;
            }
        };
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(1..=50);
        
        if guess_number == random_number {
            let current_money = user.get_money() + 50_i16;
            user.set_money(current_money);
            action_money(user);
            println!("\x1B[32mSayıyı Bildin! 50 TL Kazandın\x1B[0m");
        } else {
            let current_money = user.get_money() - 10_i16;
            user.set_money(current_money);
            action_money(user);
            println!("\x1B[5;31mSayıyı Bilemedin! 10 TL Kaybettin. Rastgele sayı: {}\x1B[0m", random_number);
        }
    }
}

fn horse_race(user : &mut UserModel) {
    println!("\x1B[2J\x1B[1;1H\x1B[0m");
    println!("At Yarışı Oyunu");
    loop {
        println!("-----------");
        print!("\n");
        println!("4 adet at bulunmaktadır. Birini Seç ( Her bilmede 25 TL / Her kaybetmede -5 TL )");
        println!("1. At - Sarı Kız - Ateş Ediyor\n2. At - Gökyüzü Mavisi - Atlet gibi hızlı\n3. At - Gece Yıldızı - Aniden çıkan alıcılık\n4. At - Fırtına - Etrafı kavuran serinlik ");
        let guess_user = input::text_input("Atı Seç ( çıkış : q ) :", false);

        if guess_user.trim().eq_ignore_ascii_case("q") {
            break;
        }

        let guess_number = match guess_user.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1B[5;31mGeçerli bir sayı girmediniz!\x1B[0m\n");
                continue;
            }
        };

        if guess_number > 4 || guess_number < 1 {
            println!("\x1B[5;31mGeçerli bir sayı girmediniz!\x1B[0m\n");
            continue;
        }

        let mut rng = rand::thread_rng();
        let winner_horse: u8 = rng.gen_range(1..=4);

        if guess_number == winner_horse {
            let current_money = user.get_money() + 25_i16;
            user.set_money(current_money);
            action_money(user);
            println!("\x1B[32mAtı Bildin! 25 TL Kazandın\x1B[0m");
        } else {
            let current_money = user.get_money() - 10_i16;
            user.set_money(current_money);
            action_money(user);
            println!("\x1B[5;31mAtı Bilemedin! 10 TL Kaybettin. Kazanan At: {} Numara\x1B[0m", winner_horse);
        }
    }
}

pub fn play_gambling(user : &mut UserModel){
    println!("-----------");
    loop {
        println!("1 - Sayı Tahmin Oyunu\n2 - At Yarışı Oyna\n3 - Geri Git");
        let stage = input::text_input("Devam etmek için yapmak istediğiniz işlem numarasını giriniz :", false);
        match stage.trim().parse::<u8>() {
            Ok(1) => {
                guess_number(user);
            },
            Ok(2) => {
                horse_race(user);
            },
            Ok(3) => break,
            _ => {
                println!("\x1B[5;31mHatalı Numara Girdiniz!\x1B[0m");
            }
        }
    }
}