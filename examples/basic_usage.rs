// 基础使用示例

use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    println!("=== fake_asia 库使用示例 ===\n");

    // 1. 生成中文姓名
    println!("【1. 中文姓名】");
    for _ in 0..3 {
        let name = format!(
            "{}{}",
            chinese_last_name(&mut rng),
            chinese_first_name(&mut rng)
        );
        println!("  {}", name);
    }

    // 2. 使用 Trait 方式生成姓名
    println!("\n【2. 使用 FakeAsia Trait】");
    for _ in 0..3 {
        let name: String = FakeAsia::fake();
        println!("  {}", name);
    }

    // 3. 生成联系方式
    println!("\n【3. 联系方式】");
    println!("  手机号: {}", chinese_phone_number(&mut rng));
    println!("  座机号: {}", chinese_landline(&mut rng));
    println!("  邮箱: {}", email(&mut rng));

    // 4. 生成身份信息
    println!("\n【4. 身份信息】");
    println!("  身份证号: {}", chinese_id_card(&mut rng));
    println!("  车牌号: {}", chinese_license_plate(&mut rng));

    // 5. 生成地址
    println!("\n【5. 地址信息】");
    let addr = chinese_address(&mut rng);
    println!("  {}", addr);

    // 6. 生成公司
    println!("\n【6. 公司信息】");
    let company = chinese_company(&mut rng);
    println!("  {}", company);

    // 7. 生成完整人员信息
    println!("\n【7. 完整人员信息】");
    let person = chinese_person(&mut rng);
    println!("{}", person);

    // 8. 批量生成
    println!("\n【8. 批量生成3个人员】");
    let persons = chinese_persons(3, &mut rng);
    for (i, person) in persons.iter().enumerate() {
        println!("\n--- 人员 {} ---", i + 1);
        println!("{}", person);
    }

    // 9. 日本数据
    println!("\n【9. 日本数据】");
    let jp_name = format!(
        "{}{}",
        japanese_last_name(&mut rng),
        japanese_male_first_name(&mut rng)
    );
    println!("  姓名: {}", jp_name);
    println!("  携帯: {}", japanese_phone_number(&mut rng));
    println!("  固定: {}", japanese_landline(&mut rng));
    println!("  地址: {}", japanese_address(&mut rng));

    // 10. 韩国数据
    println!("\n【10. 韩国数据】");
    let kr_name = format!(
        "{} {}",
        korean_last_name(&mut rng),
        korean_male_first_name(&mut rng)
    );
    println!("  姓名: {}", kr_name);
    println!("  휴대폰: {}", korean_phone_number(&mut rng));
    println!("  전화: {}", korean_landline(&mut rng));
    println!("  地址: {}", korean_address(&mut rng));

    // 11. 批量生成手机号
    println!("\n【11. 批量生成5个手机号】");
    let phones = generate_multiple(5, &mut rng, chinese_phone_number);
    for (i, phone) in phones.iter().enumerate() {
        println!("  {}. {}", i + 1, phone);
    }

    // 12. 日本完整人员信息
    println!("\n【12. 日本完整人员信息】");
    let jp_person = japanese_person(&mut rng);
    println!("{}", jp_person);

    // 13. 韩国完整人员信息
    println!("\n【13. 韩国完整人员信息】");
    let kr_person = korean_person(&mut rng);
    println!("{}", kr_person);

    // 14. 印度数据
    println!("\n【14. 印度数据 (India)】");
    let in_name = format!(
        "{} {}",
        indian_male_first_name(&mut rng),
        indian_last_name(&mut rng)
    );
    println!("  Name: {}", in_name);
    println!("  Mobile: {}", indian_phone_number(&mut rng));
    println!("  Landline: {}", indian_landline(&mut rng));
    println!("  Address: {}", indian_address(&mut rng));

    // 15. 印度完整人员信息
    println!("\n【15. 印度完整人员信息】");
    let in_person = indian_person(&mut rng);
    println!("{}", in_person);

    // 16. 四国对比
    println!("\n【16. 中日韩印对比】");
    println!("\n中国:");
    println!("{}", chinese_person(&mut rng));
    println!("\n日本:");
    println!("{}", japanese_person(&mut rng));
    println!("\n韩国:");
    println!("{}", korean_person(&mut rng));
    println!("\nIndia:");
    println!("{}", indian_person(&mut rng));

    println!("\n=== 示例结束 ===");
}

