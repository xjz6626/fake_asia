# fake_asia

🎭 A realistic fake data generator library focused on Asian countries (China, Japan, South Korea, India)

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[中文文档](README_CN.md) | English

## Introduction

`fake_asia` is a Rust library for generating realistic fake data specifically for Asian countries. It provides authentic formats for names, addresses, phone numbers, and more. Perfect for:

- 🧪 Unit and integration testing
- 📊 Development environment data population
- 🎯 API testing
- 🔒 Privacy protection (replacing real user data with fake data)
- 📝 Documentation and demos

## Supported Countries

- 🇨🇳 **China** - Complete Chinese data with real postal codes
- 🇯🇵 **Japan** - Japanese data with authentic postal codes
- 🇰🇷 **South Korea** - Korean data with real postal codes
- 🇮🇳 **India** - Indian data with genuine PIN codes

## Features

### 🇨🇳 China Data Generation

- **Names**: 100 common surnames + male/female given names
- **Mobile**: Real carrier prefixes (130-189 series)
- **Landline**: Area codes + 7-8 digit numbers
- **ID Card**: 18-digit format with check digit
- **Address**: Province, city, street, 6-digit **real postal code**
- **Company**: City + keywords + company type
- **License Plate**: Province code + letter + number combination
- **Email**: Common domains (QQ, 163, Gmail, etc.)

### 🇯🇵 Japan Data Generation

- **Names**: Common Japanese surnames and given names (male/female)
- **Mobile**: 090/080/070 series
- **Landline**: Area codes (03, 06, etc.) + numbers
- **Address**: Prefecture, city, street, 7-digit **real postal code** (XXX-XXXX format)
- **Email**: International domains

### 🇰🇷 South Korea Data Generation

- **Names**: Common Korean surnames and given names (male/female)
- **Mobile**: 010 series
- **Landline**: Area codes (02, 051, etc.) + numbers
- **Address**: Province/city, district, street, 5-digit **real postal code**
- **Email**: International domains

### 🇮🇳 India Data Generation

- **Names**: Common Indian names (multicultural backgrounds)
- **Mobile**: 10-digit numbers with real prefixes (98, 99, 97, etc.)
- **Landline**: City area codes (022, 011, etc.) + 8-digit numbers
- **Address**: City, state, street, 6-digit **real PIN code**
- **Email**: International domains

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fake_asia = "0.1.0"
```

## Quick Start

### Basic Usage

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // Generate Chinese name
    let name = format!(
        "{}{}",
        chinese_last_name(&mut rng),
        chinese_first_name(&mut rng)
    );
    println!("Name: {}", name);
    
    // Generate phone numbers
    let mobile = chinese_phone_number(&mut rng);
    let landline = chinese_landline(&mut rng);
    println!("Mobile: {}", mobile);
    println!("Landline: {}", landline);
    
    // Generate ID card
    let id_card = chinese_id_card(&mut rng);
    println!("ID: {}", id_card);
    
    // Generate address with real postal code
    let address = chinese_address(&mut rng);
    println!("Address: {}", address);
    
    // Generate email
    let email = email(&mut rng);
    println!("Email: {}", email);
    
    // Generate license plate
    let plate = chinese_license_plate(&mut rng);
    println!("Plate: {}", plate);
}
```

### Using the Trait

```rust
use fake_asia::*;

fn main() {
    // Use FakeAsia trait's fake() method
    let name: String = FakeAsia::fake();
    let address: ChineseAddress = FakeAsia::fake();
    let company: ChineseCompany = FakeAsia::fake();
    
    println!("Name: {}", name);
    println!("Address: {}", address);
    println!("Company: {}", company);
}
```

### Complete Person Information

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // Generate complete Chinese person info
    let person = chinese_person(&mut rng);
    println!("{}", person);
    
    // Generate Japanese person info
    let jp_person = japanese_person(&mut rng);
    println!("{}", jp_person);
    
    // Generate Korean person info
    let kr_person = korean_person(&mut rng);
    println!("{}", kr_person);
    
    // Generate Indian person info
    let in_person = indian_person(&mut rng);
    println!("{}", in_person);
}
```

### Batch Generation

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // Generate 10 Chinese persons
    let persons = chinese_persons(10, &mut rng);
    
    for (i, person) in persons.iter().enumerate() {
        println!("=== Person {} ===", i + 1);
        println!("{}\n", person);
    }
    
    // Use generic batch generation function
    let phones = generate_multiple(5, &mut rng, chinese_phone_number);
    println!("5 phone numbers: {:?}", phones);
}
```

### Multi-Country Examples

```rust
use fake_asia::*;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    
    // Japanese data
    let jp_name = format!(
        "{}{}",
        japanese_last_name(&mut rng),
        japanese_male_first_name(&mut rng)
    );
    println!("Japanese name: {}", jp_name);
    println!("Mobile: {}", japanese_phone_number(&mut rng));
    println!("Address: {}", japanese_address(&mut rng));
    
    // Korean data
    let kr_name = format!(
        "{} {}",
        korean_last_name(&mut rng),
        korean_male_first_name(&mut rng)
    );
    println!("Korean name: {}", kr_name);
    println!("Mobile: {}", korean_phone_number(&mut rng));
    println!("Address: {}", korean_address(&mut rng));
    
    // Indian data
    let in_name = format!(
        "{} {}",
        indian_male_first_name(&mut rng),
        indian_last_name(&mut rng)
    );
    println!("Indian name: {}", in_name);
    println!("Mobile: {}", indian_phone_number(&mut rng));
    println!("Address: {}", indian_address(&mut rng));
}
```

## API Documentation

### Chinese Data

**Names:**
- `chinese_last_name(&mut rng)` - Random surname
- `chinese_male_first_name(&mut rng)` - Random male given name
- `chinese_female_first_name(&mut rng)` - Random female given name
- `chinese_first_name(&mut rng)` - Random given name (male/female)

**Contact:**
- `chinese_phone_number(&mut rng)` - 11-digit mobile number
- `chinese_landline(&mut rng)` - Landline with area code
- `email(&mut rng)` - Email address

**Identity:**
- `chinese_id_card(&mut rng)` - 18-digit ID card with check digit
- `chinese_license_plate(&mut rng)` - License plate number

**Location:**
- `chinese_address(&mut rng)` - Complete address (returns `ChineseAddress`)
- `chinese_city(&mut rng)` - City name
- `chinese_company(&mut rng)` - Company name (returns `ChineseCompany`)

**Batch:**
- `chinese_person(&mut rng)` - Complete person information
- `chinese_persons(count, &mut rng)` - Multiple persons

### Japanese Data

- `japanese_last_name(&mut rng)` - Surname
- `japanese_male_first_name(&mut rng)` - Male given name
- `japanese_female_first_name(&mut rng)` - Female given name
- `japanese_phone_number(&mut rng)` - Mobile (090-XXXX-XXXX)
- `japanese_landline(&mut rng)` - Landline (03-XXXX-XXXX)
- `japanese_address(&mut rng)` - Complete address with real postal code
- `japanese_city(&mut rng)` - City name
- `japanese_person(&mut rng)` - Complete person information
- `japanese_persons(count, &mut rng)` - Multiple persons

### Korean Data

- `korean_last_name(&mut rng)` - Surname
- `korean_male_first_name(&mut rng)` - Male given name
- `korean_female_first_name(&mut rng)` - Female given name
- `korean_phone_number(&mut rng)` - Mobile (010-XXXX-XXXX)
- `korean_landline(&mut rng)` - Landline (02-XXXX-XXXX)
- `korean_address(&mut rng)` - Complete address with real postal code
- `korean_city(&mut rng)` - City name
- `korean_person(&mut rng)` - Complete person information
- `korean_persons(count, &mut rng)` - Multiple persons

### Indian Data

- `indian_last_name(&mut rng)` - Surname
- `indian_male_first_name(&mut rng)` - Male given name
- `indian_female_first_name(&mut rng)` - Female given name
- `indian_phone_number(&mut rng)` - Mobile (10-digit)
- `indian_landline(&mut rng)` - Landline (022-XXXXXXXX)
- `indian_address(&mut rng)` - Complete address with real PIN code
- `indian_city(&mut rng)` - City name
- `indian_person(&mut rng)` - Complete person information
- `indian_persons(count, &mut rng)` - Multiple persons

### Utility Functions

- `generate_multiple(count, &mut rng, generator)` - Generic batch generation function

### Data Structures

```rust
// Chinese address
pub struct ChineseAddress {
    pub province: &'static str,
    pub city: &'static str,
    pub street: String,
    pub postal_code: String,  // Real 6-digit postal code
}

// Japanese address
pub struct JapaneseAddress {
    pub prefecture: &'static str,
    pub city: &'static str,
    pub street: String,
    pub postal_code: String,  // Real 7-digit postal code (XXX-XXXX)
}

// Korean address
pub struct KoreanAddress {
    pub province: &'static str,
    pub city: &'static str,
    pub street: String,
    pub postal_code: String,  // Real 5-digit postal code
}

// Indian address
pub struct IndianAddress {
    pub city: &'static str,
    pub state: &'static str,
    pub street: String,
    pub postal_code: String,  // Real 6-digit PIN code
}

// Person information structures
pub struct PersonInfo { /* Chinese */ }
pub struct JapanesePersonInfo { /* Japanese */ }
pub struct KoreanPersonInfo { /* Korean */ }
pub struct IndianPersonInfo { /* Indian */ }
```

### Trait

```rust
pub trait FakeAsia {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self;
    fn fake() -> Self where Self: Sized;
}
```

Implemented for:
- `String` - Generates Chinese full name
- `ChineseAddress`, `JapaneseAddress`, `KoreanAddress`, `IndianAddress`
- `ChineseCompany`

## Running Tests

```bash
cargo test
```

View test output:

```bash
cargo test -- --nocapture
```

Run examples:

```bash
cargo run --example basic_usage
```

## Data Sources

All data used in this library comes from public information:
- Names are common surnames and given names from respective countries
- Addresses use real city names and **authentic postal codes**
- Phone numbers use real carrier/area code prefixes
- Chinese ID cards follow GB 11643-1999 standard
- All generated data is randomly combined and does not correspond to real individuals

⚠️ **Note**: Generated data is for testing purposes only. Do not use for fraud or illegal activities.

## Roadmap

- [ ] Support more Asian countries (Singapore, Thailand, Vietnam, etc.)
- [ ] Add bank card number generation
- [ ] Add passport number generation
- [ ] Support export to JSON/CSV formats
- [ ] Add more realistic address data
- [ ] Integrate pinyin library for improved email generation
- [ ] Add business credit code generation

## Contributing

Issues and Pull Requests are welcome!

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

## Acknowledgments

Thanks to all contributors to this project!

---

If this project helps you, please give it a ⭐️!
