use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let p = chinese_person(&mut rng);
    println!("姓名: {}", p.name);
    println!("手机: {}", p.phone);
    println!("邮箱: {}", p.email);
    println!("身份证: {}", p.id_card);
    println!("地址: {}", p.address);
    println!("公司: {}", p.company);
    println!("车牌: {}", chinese_license_plate(&mut rng));
    println!("银行卡: {}", generate_chinese_bank_card(&mut rng));
}
