// 我们需要 std::fmt 来实现 Display trait
use std::fmt;
use rand::Rng;
use rand::seq::SliceRandom;

// --- 1. 定义数据集 ---

mod data {
    // --- 中文姓名数据 ---
    pub const CHINESE_LAST_NAMES: &'static [&'static str] = &[
        "李", "王", "张", "刘", "陈", "杨", "赵", "黄", "周", "吴",
        "徐", "孙", "胡", "朱", "高", "林", "何", "郭", "马", "罗",
        "梁", "宋", "郑", "谢", "韩", "唐", "冯", "于", "董", "萧",
        "程", "曹", "袁", "邓", "许", "傅", "沈", "曾", "彭", "吕",
        "苏", "卢", "蒋", "蔡", "贾", "丁", "魏", "薛", "叶", "阎",
        "余", "潘", "杜", "戴", "夏", "钟", "汪", "田", "任", "姜",
        "范", "方", "石", "姚", "谭", "廖", "邹", "熊", "金", "陆",
        "郝", "孔", "白", "崔", "康", "毛", "邱", "秦", "江", "史",
        "顾", "侯", "邵", "孟", "龙", "万", "段", "漕", "钱", "汤",
        "尹", "黎", "易", "常", "武", "乔", "贺", "赖", "龚", "文",
    ];
    pub const CHINESE_MALE_FIRST_NAMES: &'static [&'static str] = &[
        "伟", "强", "磊", "军", "洋", "勇", "杰", "涛", "明", "刚",
        "平", "辉", "斌", "超", "鹏", "亮", "俊", "峰", "波", "凯",
        "浩", "鑫", "波", "宇", "博", "瑞", "志强", "建华", "文杰", "志明",
        "国栋", "卫国", "建军", "振华", "家豪", "俊杰", "浩宇", "子轩", "博文",
        "明轩", "天宇", "致远", "文昊", "国强", "永强", "德华", "文龙", "世杰",
    ];
    pub const CHINESE_FEMALE_FIRST_NAMES: &'static [&'static str] = &[
        "芳", "娜", "敏", "静", "丽", "艳", "秀英", "玉兰", "桂英", "丹",
        "萍", "燕", "娟", "红", "霞", "慧", "婷", "洁", "玲", "琳",
        "菲", "雪", "倩", "珍", "怡", "欣", "雅", "琪", "梦", "晨",
        "秀兰", "桂兰", "雅婷", "欣怡", "子涵", "雨涵", "诗涵", "梦洁", "晓燕",
        "美玲", "晓慧", "文静", "思雨", "若曦", "语嫣", "静怡", "雪梅", "丽华",
    ];

    // --- 中文地址数据 ---
    // (省份, 城市, 邮编前缀) - 前缀现在是6位
    pub const CHINESE_PROVINCE_CITY_POSTAL: &'static [(&'static str, &'static str, &'static str)] = &[
        ("北京市", "北京市", "100000"),
        ("上海市", "上海市", "200000"),
        ("广东省", "广州市", "510000"),
        ("广东省", "深圳市", "518000"),
        ("天津市", "天津市", "300000"),
        ("重庆市", "重庆市", "400000"),
        ("四川省", "成都市", "610000"),
        ("浙江省", "杭州市", "310000"),
        ("湖北省", "武汉市", "430000"),
        ("陕西省", "西安市", "710000"),
        ("江苏省", "南京市", "210000"),
        ("湖南省", "长沙市", "410000"),
        ("江苏省", "苏州市", "215000"),
        ("福建省", "厦门市", "361000"),
        ("山东省", "青岛市", "266000"),
        ("辽宁省", "大连市", "116000"),
        ("辽宁省", "沈阳市", "110000"),
        ("黑龙江省", "哈尔滨市", "150000"),
        ("山东省", "济南市", "250000"),
        ("河南省", "郑州市", "450000"),
    ];
    pub const CHINESE_STREET_SUFFIXES: &'static [&'static str] = &[
        "路", "街", "大道", "巷", "胡同", "小区", "广场",
    ];
    pub const CHINESE_ROAD_NAMES: &'static [&'static str] = &[
        "人民", "解放", "建设", "和平", "中山", "胜利", "平安", "光明",
        "花园", "中心", "长江", "黄河", "新华", "民主", "团结",
    ];

    // --- 中文电话数据 ---
    pub const CHINESE_MOBILE_PREFIXES: &'static [&'static str] = &[
        "130", "131", "132", "133", "134", "135", "136", "137", "138", "139",
        "150", "151", "152", "153", "155", "156", "157", "158", "159",
        "180", "181", "182", "183", "184", "185", "186", "187", "188", "189",
        "170", "176", "177", "178", "198", "199", "166",
    ];

    // --- 中文公司数据 ---
    pub const CHINESE_COMPANY_KEYWORDS: &'static [&'static str] = &[
        "华", "信", "通", "海", "天", "神", "龙", "安", "泰", "盛",
        "创", "新", "远", "东", "光", "明", "博", "达", "瑞", "丰",
        "科技", "信息", "能源", "贸易", "实业", "文化", "发展", "建设",
    ];
    pub const CHINESE_COMPANY_SUFFIXES: &'static [&'static str] = &[
        "有限公司", "股份有限公司", "集团", "（中国）有限公司", "信息技术有限公司",
    ];

    // --- 其他亚洲数据 ---
    pub const JAPANESE_CITIES: &'static [&'static str] = &[
        "东京", "大阪", "京都", "横滨", "名古屋", "札幌", "神户",
        "福冈", "广岛", "仙台", "那霸",
    ];
    
    // (韩国姓氏，现在将被使用)
    pub const KOREAN_LAST_NAMES: &'static [&'static str] = &[
        "김 (Kim)", "이 (Lee)", "박 (Park)", "최 (Choi)", "정 (Jeong)",
        "강 (Kang)", "조 (Cho)", "윤 (Yoon)", "장 (Jang)", "임 (Lim)",
    ];
}

// --- 2. 定义新的结构体 ---

/// 代表一个伪造的中国地址
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChineseAddress {
    pub province: &'static str,
    pub city: &'static str,
    pub street: String,
    pub postal_code: String,
}

/// 实现 Display Trait，以便能被 println! 打印
impl fmt::Display for ChineseAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} (邮编: {})",
            self.province, self.city, self.street, self.postal_code
        )
    }
}

/// 代表一个伪造的中国公司
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChineseCompany {
    pub name: String,
}

impl fmt::Display for ChineseCompany {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


// --- 3. 创建“取值器” (Faker) 函数 ---

// (姓名函数)
pub fn chinese_last_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::CHINESE_LAST_NAMES.choose(rng).unwrap_or(&"")
}
pub fn chinese_male_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::CHINESE_MALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}
pub fn chinese_female_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::CHINESE_FEMALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}
pub fn chinese_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    let all_names = [data::CHINESE_MALE_FIRST_NAMES, data::CHINESE_FEMALE_FIRST_NAMES];
    let chosen_list = all_names.choose(rng).unwrap_or(&data::CHINESE_MALE_FIRST_NAMES);
    chosen_list.choose(rng).unwrap_or(&"")
}

// (城市函数)
pub fn chinese_city<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::CHINESE_PROVINCE_CITY_POSTAL.choose(rng).unwrap_or(&("","","")).1
}
pub fn japanese_city<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::JAPANESE_CITIES.choose(rng).unwrap_or(&"")
}

/// *** 新增 *** 获取一个随机的韩国姓氏
pub fn korean_last_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::KOREAN_LAST_NAMES.choose(rng).unwrap_or(&"")
}


// --- 电话、地址、公司 ---

/// 获取一个随机的中国大陆手机号 (String)
pub fn chinese_phone_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefix = data::CHINESE_MOBILE_PREFIXES.choose(rng).unwrap_or(&"138");
    let suffix: String = (0..8).map(|_| rng.gen_range(0..=9).to_string()).collect();
    format!("{}{}", prefix, suffix)
}

/// 获取一个随机的中国地址 (ChineseAddress 结构体)
pub fn chinese_address<R: Rng + ?Sized>(rng: &mut R) -> ChineseAddress {
    // 1. 选择 省份、城市、邮编前缀
    let (province, city, base_postal_code) = data::CHINESE_PROVINCE_CITY_POSTAL.choose(rng).unwrap_or(&("北京市", "北京市", "100000"));
    
    // 2. 生成街道
    let road_name = data::CHINESE_ROAD_NAMES.choose(rng).unwrap_or(&"人民");
    let suffix = data::CHINESE_STREET_SUFFIXES.choose(rng).unwrap_or(&"路");
    let number = rng.gen_range(1..=999);
    let street = format!("{}{}{}号", road_name, suffix, number);
    
    // 3. *** 修复BUG ***
    //    生成邮编 (取前缀的前3位 + 3位随机数)
    let postal_prefix = &base_postal_code[0..3]; // 例如, "100000" -> "100"
    let postal_suffix: String = (0..3).map(|_| rng.gen_range(0..=9).to_string()).collect();
    let postal_code = format!("{}{}", postal_prefix, postal_suffix); // 结果 "100" + "123" = "100123" (6位)
    
    ChineseAddress {
        province,
        city,
        street,
        postal_code,
    }
}

/// 获取一个随机的中国公司名 (ChineseCompany 结构体)
pub fn chinese_company<R: Rng + ?Sized>(rng: &mut R) -> ChineseCompany {
    // 随机选择一个城市名，并去掉“市”
    let city = data::CHINESE_PROVINCE_CITY_POSTAL.choose(rng).unwrap_or(&("","北京","")).1.replace("市", "");
    let keyword1 = data::CHINESE_COMPANY_KEYWORDS.choose(rng).unwrap_or(&"");
    let keyword2 = data::CHINESE_COMPANY_KEYWORDS.choose(rng).unwrap_or(&"");
    let suffix = data::CHINESE_COMPANY_SUFFIXES.choose(rng).unwrap_or(&"");

    // 随机组合 1-2 个关键词
    let name_part = if rng.gen_bool(0.5) {
        format!("{}{}", keyword1, keyword2)
    } else {
        keyword1.to_string()
    };

    let name = format!("{}{}{}", city, name_part, suffix);
    ChineseCompany { name }
}


// --- 4. 定义 FakeAsia Trait ---

pub trait FakeAsia {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self;

    fn fake() -> Self where Self: Sized {
        let mut rng = rand::thread_rng();
        Self::fake_asia(&mut rng)
    }
}

// 为 String 实现您的 Trait (用于生成全名)
impl FakeAsia for String {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self {
        format!(
            "{}{}",
            chinese_last_name(rng),
            chinese_first_name(rng)
        )
    }
}

// (为地址和公司实现 Trait)
impl FakeAsia for ChineseAddress {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self {
        chinese_address(rng)
    }
}

impl FakeAsia for ChineseCompany {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self {
        chinese_company(rng)
    }
}


// --- 5. 单元测试 ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chinese_names() {
        let mut rng = rand::thread_rng();
        
        let last_name = chinese_last_name(&mut rng);
        assert!(!last_name.is_empty());
        println!("随机姓氏: {}", last_name);

        let male_name = chinese_male_first_name(&mut rng);
        assert!(!male_name.is_empty());
        println!("随机男性名字: {}", male_name);
        
        let female_name = chinese_female_first_name(&mut rng);
        assert!(!female_name.is_empty());
        println!("随机女性名字: {}", female_name);
    }

    #[test]
    fn test_get_chinese_city_name() {
        let mut rng = rand::thread_rng();
        let city = chinese_city(&mut rng);
        println!("随机中国城市名: {}", city);
        assert!(!city.is_empty());
    }

    #[test]
    fn test_fake_trait_for_string_name() {
        let full_name: String = FakeAsia::fake();
        println!("Trait 生成的中文全名: {}", full_name);
        assert!(full_name.len() > 1);
    }

    // --- 测试电话、地址、公司 ---

    #[test]
    fn test_get_chinese_phone_number() {
        let mut rng = rand::thread_rng();
        let phone = chinese_phone_number(&mut rng);
        
        println!("随机手机号: {}", phone);
        assert_eq!(phone.len(), 11); // 手机号都是11位
        assert!(phone.starts_with("1"));
    }

    #[test]
    fn test_get_chinese_address() {
        let mut rng = rand::thread_rng();
        let addr = chinese_address(&mut rng);

        println!("随机地址: {}", addr); // 这里会调用 Display trait
        
        // *** 修复BUG ***：这里是测试失败的地方
        assert_eq!(addr.postal_code.len(), 6); // 现在邮编应该是6位了
        
        assert!(addr.street.contains("号"));
        assert!(!addr.province.is_empty());
    }

    #[test]
    fn test_get_chinese_company() {
        let mut rng = rand::thread_rng();
        let company = chinese_company(&mut rng);

        println!("随机公司: {}", company); // 这里会调用 Display trait
        assert!(company.name.ends_with("有限公司") || company.name.ends_with("集团") || company.name.ends_with("股份有限公司") || company.name.ends_with("（中国）有限公司") || company.name.ends_with("信息技术有限公司"));
    }

    #[test]
    fn test_fake_trait_for_structs() {
        let addr: ChineseAddress = FakeAsia::fake();
        let company: ChineseCompany = FakeAsia::fake();

        println!("Trait 生成的地址: {}", addr);
        println!("Trait 生成的公司: {}", company);
        
        // *** 修复BUG ***：这里是测试失败的地方
        assert_eq!(addr.postal_code.len(), 6);
        
        assert!(!company.name.is_empty());
    }

    // *** 新增测试 (解决 dead_code 警告) ***
    #[test]
    fn test_get_korean_last_name() {
        let mut rng = rand::thread_rng();
        let name = korean_last_name(&mut rng);
        println!("随机韩国姓氏: {}", name);
        assert!(!name.is_empty());
        assert!(data::KOREAN_LAST_NAMES.contains(&name));
    }
}