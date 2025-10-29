# fake_asia

🎭 专注于亚洲国家（中国、日本、韩国、印度）的真实测试数据生成库

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

中文文档 | [English](README.md)

## 简介

`fake_asia` 是一个用 Rust 编写的伪造数据生成库，专门用于生成亚洲地区的真实格式测试数据。提供真实的姓名、地址、电话号码等数据格式。适合用于：

- 🧪 单元测试和集成测试
- 📊 开发环境数据填充
- 🎯 API 接口测试
- 🔒 隐私保护（用假数据替代真实用户信息）
- 📝 文档和演示

## 支持国家

- 🇨🇳 **中国** - 完整的中文数据，真实邮编
- 🇯🇵 **日本** - 日本数据，真实邮编
- 🇰🇷 **韩国** - 韩国数据，真实邮编
- 🇮🇳 **印度** - 印度数据，真实PIN码

## 功能特性

### 🇨🇳 中国数据生成

- **姓名**：100个常见姓氏 + 男性/女性名字
- **手机号**：真实的号段前缀（130-189系列）
- **座机号**：区号 + 7-8位号码
- **身份证号**：18位标准格式，带校验码
- **地址**：省份、城市、街道、6位**真实邮编**
- **公司名**：城市 + 关键词 + 公司类型
- **车牌号**：省份简称 + 字母 + 数字组合
- **电子邮件**：常见域名（QQ、163、Gmail等）

### 🇯🇵 日本数据生成

- **姓名**：常见日本姓氏和名字（男/女）
- **手机号**：090/080/070系列
- **座机号**：区号（03、06等）+ 号码
- **地址**：都道府县、城市、街道、7位**真实邮编**（XXX-XXXX格式）
- **电子邮件**：国际域名

### 🇰🇷 韩国数据生成

- **姓名**：常见韩国姓氏和名字（男/女）
- **手机号**：010系列
- **座机号**：区号（02、051等）+ 号码
- **地址**：道/市、区、街道、5位**真实邮编**
- **电子邮件**：国际域名

### 🇮🇳 印度数据生成

- **姓名**：常见印度名字（多种文化背景）
- **手机号**：10位数字，真实前缀（98、99、97等）
- **座机号**：城市区号（022、011等）+ 8位号码
- **地址**：城市、州、街道、6位**真实PIN码**
- **电子邮件**：国际域名

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
fake_asia = "0.1.0"
```

## 快速开始

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
    
    // 生成地址（带真实邮编）
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

### 使用 Trait 方式

```rust
use fake_asia::*;

fn main() {
    // 使用 FakeAsia trait 的 fake() 方法
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

### 批量生成数据

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // 批量生成10个中国人员信息
    let persons = chinese_persons(10, &mut rng);
    
    for (i, person) in persons.iter().enumerate() {
        println!("=== 人员 {} ===", i + 1);
        println!("{}\n", person);
    }
    
    // 使用通用的批量生成函数
    let phones = generate_multiple(5, &mut rng, chinese_phone_number);
    println!("5个手机号: {:?}", phones);
}
```

### 多国数据示例

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // 日本数据
    let jp_name = format!(
        "{}{}",
        japanese_last_name(&mut rng),
        japanese_male_first_name(&mut rng)
    );
    println!("日本姓名: {}", jp_name);
    println!("手机: {}", japanese_phone_number(&mut rng));
    println!("地址: {}", japanese_address(&mut rng));
    
    // 韩国数据
    let kr_name = format!(
        "{} {}",
        korean_last_name(&mut rng),
        korean_male_first_name(&mut rng)
    );
    println!("韩国姓名: {}", kr_name);
    println!("手机: {}", korean_phone_number(&mut rng));
    println!("地址: {}", korean_address(&mut rng));
    
    // 印度数据
    let in_name = format!(
        "{} {}",
        indian_male_first_name(&mut rng),
        indian_last_name(&mut rng)
    );
    println!("印度姓名: {}", in_name);
    println!("手机: {}", indian_phone_number(&mut rng));
    println!("地址: {}", indian_address(&mut rng));
}
```

## API 文档

### 中国数据

**姓名相关：**
- `chinese_last_name(&mut rng)` - 随机中文姓氏
- `chinese_male_first_name(&mut rng)` - 随机男性名字
- `chinese_female_first_name(&mut rng)` - 随机女性名字
- `chinese_first_name(&mut rng)` - 随机名字（男/女）

**联系方式：**
- `chinese_phone_number(&mut rng)` - 11位手机号
- `chinese_landline(&mut rng)` - 带区号的座机号
- `email(&mut rng)` - 电子邮件地址

**身份信息：**
- `chinese_id_card(&mut rng)` - 18位身份证号（带校验码）
- `chinese_license_plate(&mut rng)` - 车牌号

**地址和公司：**
- `chinese_address(&mut rng)` - 完整地址（返回 `ChineseAddress` 结构体）
- `chinese_city(&mut rng)` - 城市名
- `chinese_company(&mut rng)` - 公司名（返回 `ChineseCompany` 结构体）

**批量生成：**
- `chinese_person(&mut rng)` - 生成完整的人员信息
- `chinese_persons(count, &mut rng)` - 批量生成多个人员信息

### 日本数据

- `japanese_last_name(&mut rng)` - 日本姓氏
- `japanese_male_first_name(&mut rng)` - 日本男性名字
- `japanese_female_first_name(&mut rng)` - 日本女性名字
- `japanese_phone_number(&mut rng)` - 手机号（090-XXXX-XXXX）
- `japanese_landline(&mut rng)` - 座机号（03-XXXX-XXXX）
- `japanese_address(&mut rng)` - 完整地址（带真实邮编）
- `japanese_city(&mut rng)` - 城市名
- `japanese_person(&mut rng)` - 完整人员信息
- `japanese_persons(count, &mut rng)` - 批量生成

### 韩国数据

- `korean_last_name(&mut rng)` - 韩国姓氏
- `korean_male_first_name(&mut rng)` - 韩国男性名字
- `korean_female_first_name(&mut rng)` - 韩国女性名字
- `korean_phone_number(&mut rng)` - 手机号（010-XXXX-XXXX）
- `korean_landline(&mut rng)` - 座机号（02-XXXX-XXXX）
- `korean_address(&mut rng)` - 完整地址（带真实邮编）
- `korean_city(&mut rng)` - 城市名
- `korean_person(&mut rng)` - 完整人员信息
- `korean_persons(count, &mut rng)` - 批量生成

### 印度数据

- `indian_last_name(&mut rng)` - 印度姓氏
- `indian_male_first_name(&mut rng)` - 印度男性名字
- `indian_female_first_name(&mut rng)` - 印度女性名字
- `indian_phone_number(&mut rng)` - 手机号（10位数字）
- `indian_landline(&mut rng)` - 座机号（022-XXXXXXXX）
- `indian_address(&mut rng)` - 完整地址（带真实PIN码）
- `indian_city(&mut rng)` - 城市名
- `indian_person(&mut rng)` - 完整人员信息
- `indian_persons(count, &mut rng)` - 批量生成

### 工具函数

- `generate_multiple(count, &mut rng, generator)` - 通用批量生成函数

### 数据结构

```rust
// 中国地址
pub struct ChineseAddress {
    pub province: &'static str,
    pub city: &'static str,
    pub street: String,
    pub postal_code: String,  // 真实的6位邮编
}

// 日本地址
pub struct JapaneseAddress {
    pub prefecture: &'static str,
    pub city: &'static str,
    pub street: String,
    pub postal_code: String,  // 真实的7位邮编（XXX-XXXX格式）
}

// 韩国地址
pub struct KoreanAddress {
    pub province: &'static str,
    pub city: &'static str,
    pub street: String,
    pub postal_code: String,  // 真实的5位邮编
}

// 印度地址
pub struct IndianAddress {
    pub city: &'static str,
    pub state: &'static str,
    pub street: String,
    pub postal_code: String,  // 真实的6位PIN码
}

// 人员信息结构
pub struct PersonInfo { /* 中国人员 */ }
pub struct JapanesePersonInfo { /* 日本人员 */ }
pub struct KoreanPersonInfo { /* 韩国人员 */ }
pub struct IndianPersonInfo { /* 印度人员 */ }
```

### Trait

```rust
pub trait FakeAsia {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self;
    fn fake() -> Self where Self: Sized;
}
```

已为以下类型实现 `FakeAsia`：
- `String` - 生成中文全名
- `ChineseAddress`、`JapaneseAddress`、`KoreanAddress`、`IndianAddress`
- `ChineseCompany`

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

## 数据来源说明

本库使用的所有数据均为公开的常见信息：
- 姓名来自各国的常见姓氏和名字
- 地址使用真实的城市名称和**真实邮编**
- 电话号使用真实的运营商号段/区号前缀
- 中国身份证号按照国标 GB 11643-1999 规则生成
- 所有生成的数据都是随机组合，不对应真实个人

⚠️ **注意**：生成的数据仅用于测试目的，请勿用于欺诈或其他非法用途。

## 未来计划

- [ ] 支持更多亚洲国家（新加坡、泰国、越南等）
- [ ] 添加银行卡号生成
- [ ] 添加护照号生成
- [ ] 支持导出为 JSON/CSV 格式
- [ ] 添加更多真实的地址数据
- [ ] 集成拼音库以改进邮箱生成
- [ ] 添加企业信用代码生成

## 贡献

欢迎提交 Issue 和 Pull Request！

## 开源协议

本项目采用 MIT 协议开源。详见 [LICENSE](LICENSE) 文件。

## 致谢

感谢所有为这个项目做出贡献的开发者！

---

如果这个项目对你有帮助，请给个 ⭐️ 吧！

