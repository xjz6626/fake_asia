# fake_asia 中文文档

## 项目简介

`fake_asia` 是一个专注于亚洲国家的假数据生成器 Rust 库，支持中国、日本、韩国和印度。它提供真实格式的姓名、地址、电话号码等数据生成，适用于单元测试、开发环境数据填充、API 测试等场景。

## 支持国家

- 🇨🇳 **中国** - 包含真实邮编的完整中文数据
- 🇯🇵 **日本** - 包含真实邮编的日本数据
- 🇰🇷 **韩国** - 包含真实邮编的韩国数据
- 🇮🇳 **印度** - 包含真实 PIN 码的印度数据

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
fake_asia = "0.1.0"
```

## 快速上手

### 基础用法

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // 生成中文姓名
    let name = format!(
        "{}{}",
        chinese_last_name(&mut rng),
        chinese_first_name(&mut rng)
    );
    println!("姓名: {}", name);
    
    // 生成电话号码
    let mobile = chinese_phone_number(&mut rng);
    let landline = chinese_landline(&mut rng);
    println!("手机: {}", mobile);
    println!("座机: {}", landline);
    
    // 生成身份证号
    let id_card = chinese_id_card(&mut rng);
    println!("身份证: {}", id_card);
    
    // 生成地址（含真实邮编）
    let address = chinese_address(&mut rng);
    println!("地址: {}", address);
    
    // 生成邮箱
    let email = email(&mut rng);
    println!("邮箱: {}", email);
    
    // 生成车牌号
    let plate = chinese_license_plate(&mut rng);
    println!("车牌: {}", plate);
}
```

### 使用 FakeAsia Trait

```rust
use fake_asia::*;

fn main() {
    // 通过 FakeAsia trait 的 fake() 方法生成
    let name: String = FakeAsia::fake();
    let address: ChineseAddress = FakeAsia::fake();
    let company: ChineseCompany = FakeAsia::fake();
    
    println!("姓名: {}", name);
    println!("地址: {}", address);
    println!("公司: {}", company);
}
```

### 生成完整人员信息

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // 生成完整的中国人员信息
    let person = chinese_person(&mut rng);
    println!("{}", person);
    
    // 生成日本人员信息
    let jp_person = japanese_person(&mut rng);
    println!("{}", jp_person);
    
    // 生成韩国人员信息
    let kr_person = korean_person(&mut rng);
    println!("{}", kr_person);
    
    // 生成印度人员信息
    let in_person = indian_person(&mut rng);
    println!("{}", in_person);
}
```

### 批量生成

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // 批量生成 10 个中国人员
    let persons = chinese_persons(10, &mut rng);
    
    for (i, person) in persons.iter().enumerate() {
        println!("=== 人员 {} ===", i + 1);
        println!("{}\n", person);
    }
    
    // 使用通用批量生成函数
    let phones = generate_multiple(5, &mut rng, chinese_phone_number);
    println!("5 个手机号: {:?}", phones);
}
```

## API 文档

### 中国数据

**姓名：**
- `chinese_last_name(&mut rng)` - 随机姓氏
- `chinese_male_first_name(&mut rng)` - 随机男性名字
- `chinese_female_first_name(&mut rng)` - 随机女性名字
- `chinese_first_name(&mut rng)` - 随机名字（男女混合）

**联系方式：**
- `chinese_phone_number(&mut rng)` - 11 位手机号
- `chinese_landline(&mut rng)` - 固定电话（区号 + 号码）
- `email(&mut rng)` - 电子邮件地址

**身份信息：**
- `chinese_id_card(&mut rng)` - 18 位身份证号（含校验位）
- `chinese_license_plate(&mut rng)` - 车牌号

**地址：**
- `chinese_address(&mut rng)` - 完整地址（返回 `ChineseAddress` 结构体）
- `chinese_city(&mut rng)` - 城市名
- `chinese_company(&mut rng)` - 公司名（返回 `ChineseCompany` 结构体）

**批量：**
- `chinese_person(&mut rng)` - 完整人员信息
- `chinese_persons(count, &mut rng)` - 批量人员信息

### 日本数据

- `japanese_last_name(&mut rng)` - 姓氏
- `japanese_male_first_name(&mut rng)` - 男性名字
- `japanese_female_first_name(&mut rng)` - 女性名字
- `japanese_phone_number(&mut rng)` - 手机号（090-XXXX-XXXX）
- `japanese_landline(&mut rng)` - 固定电话（03-XXXX-XXXX）
- `japanese_address(&mut rng)` - 完整地址（含真实邮编）
- `japanese_city(&mut rng)` - 城市名
- `japanese_person(&mut rng)` - 完整人员信息
- `japanese_persons(count, &mut rng)` - 批量人员信息

### 韩国数据

- `korean_last_name(&mut rng)` - 姓氏
- `korean_male_first_name(&mut rng)` - 男性名字
- `korean_female_first_name(&mut rng)` - 女性名字
- `korean_phone_number(&mut rng)` - 手机号（010-XXXX-XXXX）
- `korean_landline(&mut rng)` - 固定电话（02-XXXX-XXXX）
- `korean_address(&mut rng)` - 完整地址（含真实邮编）
- `korean_city(&mut rng)` - 城市名
- `korean_person(&mut rng)` - 完整人员信息
- `korean_persons(count, &mut rng)` - 批量人员信息

### 印度数据

- `indian_last_name(&mut rng)` - 姓氏
- `indian_male_first_name(&mut rng)` - 男性名字
- `indian_female_first_name(&mut rng)` - 女性名字
- `indian_phone_number(&mut rng)` - 手机号（10 位）
- `indian_landline(&mut rng)` - 固定电话（022-XXXXXXXX）
- `indian_address(&mut rng)` - 完整地址（含真实 PIN 码）
- `indian_city(&mut rng)` - 城市名
- `indian_person(&mut rng)` - 完整人员信息
- `indian_persons(count, &mut rng)` - 批量人员信息

### 工具函数

- `generate_multiple(count, &mut rng, generator)` - 通用批量生成函数

## 数据结构

```rust
// 中国地址
pub struct ChineseAddress {
    pub province: &'static str,    // 省份
    pub city: &'static str,        // 城市
    pub street: String,            // 街道
    pub postal_code: String,       // 真实 6 位邮编
}

// 日本地址
pub struct JapaneseAddress {
    pub prefecture: &'static str,  // 都道府县
    pub city: &'static str,        // 市区
    pub street: String,            // 街道
    pub postal_code: String,       // 真实 7 位邮编（XXX-XXXX）
}

// 韩国地址
pub struct KoreanAddress {
    pub province: &'static str,    // 道/特别市
    pub city: &'static str,        // 区/市
    pub street: String,            // 街道
    pub postal_code: String,       // 真实 5 位邮编
}

// 印度地址
pub struct IndianAddress {
    pub city: &'static str,        // 城市
    pub state: &'static str,       // 州
    pub street: String,            // 街道
    pub postal_code: String,       // 真实 6 位 PIN 码
}

// 人员信息结构体
pub struct PersonInfo { /* 中国 */ }
pub struct JapanesePersonInfo { /* 日本 */ }
pub struct KoreanPersonInfo { /* 韩国 */ }
pub struct IndianPersonInfo { /* 印度 */ }
```

## FakeAsia Trait

```rust
pub trait FakeAsia {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self;
    fn fake() -> Self where Self: Sized;
}
```

已实现的类型：
- `String` - 生成中文全名
- `ChineseAddress` - 中国地址
- `JapaneseAddress` - 日本地址
- `KoreanAddress` - 韩国地址
- `IndianAddress` - 印度地址
- `ChineseCompany` - 中国公司

## 运行测试

```bash
cargo test
```

查看测试输出：

```bash
cargo test -- --nocapture
```

运行示例：

```bash
cargo run --example basic_usage
```

## 数据来源

本库使用的所有数据均来自公开信息：
- 姓名来自各国常见姓氏和名字
- 地址使用真实城市名和**真实邮编**
- 电话号码使用真实运营商/区号前缀
- 中国身份证遵循 GB 11643-1999 标准
- 所有生成的数据均为随机组合，不对应真实个人

⚠️ **注意**：生成的数据仅供测试使用，请勿用于欺诈或非法活动。

## 未来计划

- [ ] 支持更多亚洲国家（新加坡、泰国、越南等）
- [ ] 添加银行卡号生成
- [ ] 添加护照号生成
- [ ] 支持导出为 JSON/CSV 格式
- [ ] 添加更多真实地址数据
- [ ] 集成拼音库以改进邮箱生成
- [ ] 添加企业信用代码生成

## 许可证

本项目基于 MIT 许可证开源，详见 [LICENSE](LICENSE) 文件。
