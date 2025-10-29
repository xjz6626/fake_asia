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
    // (省份, 城市, 真实邮编列表)
    pub const CHINESE_PROVINCE_CITY_POSTAL: &'static [(&'static str, &'static str, &'static [&'static str])] = &[
        ("北京市", "北京市", &["100000", "100005", "100020", "100035", "100044", "100050", "100081", "100089"]),
        ("上海市", "上海市", &["200000", "200001", "200021", "200030", "200050", "200080", "200120", "200233"]),
        ("广东省", "广州市", &["510000", "510030", "510060", "510120", "510180", "510220", "510280", "510360"]),
        ("广东省", "深圳市", &["518000", "518001", "518026", "518034", "518053", "518083", "518101", "518129"]),
        ("天津市", "天津市", &["300000", "300010", "300051", "300100", "300130", "300193", "300221", "300301"]),
        ("重庆市", "重庆市", &["400000", "400010", "400020", "400030", "400042", "400060", "400084", "400700"]),
        ("四川省", "成都市", &["610000", "610011", "610031", "610041", "610051", "610064", "610081", "610213"]),
        ("浙江省", "杭州市", &["310000", "310002", "310007", "310012", "310020", "310051", "311100", "311200"]),
        ("湖北省", "武汉市", &["430000", "430014", "430022", "430030", "430050", "430063", "430070", "430223"]),
        ("陕西省", "西安市", &["710000", "710003", "710016", "710032", "710043", "710054", "710068", "710100"]),
        ("江苏省", "南京市", &["210000", "210008", "210012", "210018", "210029", "210037", "211100", "211189"]),
        ("湖南省", "长沙市", &["410000", "410005", "410011", "410013", "410021", "410083", "410100", "410205"]),
        ("江苏省", "苏州市", &["215000", "215004", "215008", "215021", "215104", "215123", "215131", "215600"]),
        ("福建省", "厦门市", &["361000", "361001", "361004", "361006", "361009", "361012", "361100", "361023"]),
        ("山东省", "青岛市", &["266000", "266001", "266003", "266011", "266021", "266033", "266071", "266100"]),
        ("辽宁省", "大连市", &["116000", "116001", "116011", "116021", "116023", "116033", "116100", "116200"]),
        ("辽宁省", "沈阳市", &["110000", "110001", "110011", "110013", "110021", "110031", "110101", "110122"]),
        ("黑龙江省", "哈尔滨市", &["150000", "150001", "150010", "150020", "150030", "150040", "150060", "150080"]),
        ("山东省", "济南市", &["250000", "250001", "250011", "250012", "250014", "250022", "250100", "250200"]),
        ("河南省", "郑州市", &["450000", "450001", "450003", "450006", "450008", "450015", "450052", "450100"]),
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

    // 中国固定电话区号（城市，区号）
    pub const CHINESE_LANDLINE_AREA_CODES: &'static [(&'static str, &'static str)] = &[
        ("北京", "010"),
        ("上海", "021"),
        ("广州", "020"),
        ("深圳", "0755"),
        ("天津", "022"),
        ("重庆", "023"),
        ("成都", "028"),
        ("杭州", "0571"),
        ("武汉", "027"),
        ("西安", "029"),
        ("南京", "025"),
        ("长沙", "0731"),
        ("苏州", "0512"),
        ("厦门", "0592"),
        ("青岛", "0532"),
        ("大连", "0411"),
        ("沈阳", "024"),
        ("哈尔滨", "0451"),
        ("济南", "0531"),
        ("郑州", "0371"),
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

    // --- 身份证地区码 (前6位) ---
    pub const ID_CARD_AREA_CODES: &'static [&'static str] = &[
        "110000", // 北京
        "310000", // 上海
        "440000", // 广东
        "500000", // 重庆
        "510000", // 四川
        "330000", // 浙江
        "420000", // 湖北
        "610000", // 陕西
        "320000", // 江苏
        "430000", // 湖南
        "350000", // 福建
        "370000", // 山东
        "210000", // 辽宁
        "230000", // 黑龙江
        "410000", // 河南
    ];

    // --- 中国车牌省份代码 ---
    pub const LICENSE_PLATE_PROVINCES: &'static [(&'static str, &'static str)] = &[
        ("京", "北京"),
        ("沪", "上海"),
        ("粤", "广东"),
        ("津", "天津"),
        ("渝", "重庆"),
        ("川", "四川"),
        ("浙", "浙江"),
        ("鄂", "湖北"),
        ("陕", "陕西"),
        ("苏", "江苏"),
        ("湘", "湖南"),
        ("闽", "福建"),
        ("鲁", "山东"),
        ("辽", "辽宁"),
        ("黑", "黑龙江"),
        ("豫", "河南"),
    ];

    // --- 邮箱域名 ---
    pub const EMAIL_DOMAINS: &'static [&'static str] = &[
        "qq.com", "163.com", "126.com", "sina.com", "sohu.com",
        "gmail.com", "outlook.com", "foxmail.com", "yeah.net", "aliyun.com",
    ];

    // --- 日本姓氏和名字 ---
    pub const JAPANESE_LAST_NAMES: &'static [&'static str] = &[
        "佐藤", "鈴木", "高橋", "田中", "渡辺", "伊藤", "山本", "中村",
        "小林", "加藤", "吉田", "山田", "佐々木", "山口", "松本", "井上",
    ];
    
    pub const JAPANESE_MALE_FIRST_NAMES: &'static [&'static str] = &[
        "太郎", "健太", "大輔", "誠", "拓也", "翔太", "健", "翔",
        "優", "陽翔", "蓮", "大翔", "悠真", "湊", "樹",
    ];

    pub const JAPANESE_FEMALE_FIRST_NAMES: &'static [&'static str] = &[
        "さくら", "美咲", "結衣", "愛", "陽菜", "美羽", "葵", "さくら",
        "凛", "莉子", "心春", "結菜", "杏", "柚", "花",
    ];

    // --- 韩国名字 ---
    pub const KOREAN_MALE_FIRST_NAMES: &'static [&'static str] = &[
        "민준 (Min-jun)", "서준 (Seo-jun)", "예준 (Ye-jun)", "도윤 (Do-yoon)",
        "시우 (Si-woo)", "주원 (Ju-won)", "하준 (Ha-jun)", "지호 (Ji-ho)",
    ];

    pub const KOREAN_FEMALE_FIRST_NAMES: &'static [&'static str] = &[
        "서연 (Seo-yeon)", "지우 (Ji-woo)", "서윤 (Seo-yoon)", "지민 (Ji-min)",
        "하은 (Ha-eun)", "민서 (Min-seo)", "수아 (Su-ah)", "은서 (Eun-seo)",
    ];

    // --- 韩国城市 ---
    pub const KOREAN_CITIES: &'static [&'static str] = &[
        "서울 (Seoul)", "부산 (Busan)", "인천 (Incheon)", "대구 (Daegu)",
        "대전 (Daejeon)", "광주 (Gwangju)", "수원 (Suwon)", "제주 (Jeju)",
    ];

    // --- 日本详细地址数据 ---
    // (都道府县, 城市, 真实邮编范围)
    pub const JAPANESE_PREFECTURE_CITY_POSTAL: &'static [(&'static str, &'static str, &'static [&'static str])] = &[
        ("東京都", "新宿区", &["160-0001", "160-0022", "160-0023", "169-0072", "169-0075"]),
        ("東京都", "渋谷区", &["150-0001", "150-0002", "150-0011", "150-0043", "151-0051"]),
        ("東京都", "港区", &["105-0001", "105-0011", "106-0032", "107-0052", "108-0014"]),
        ("大阪府", "大阪市", &["530-0001", "530-0047", "531-0071", "542-0081", "556-0011"]),
        ("京都府", "京都市", &["600-8216", "602-0841", "604-0931", "606-8335", "612-8082"]),
        ("神奈川県", "横浜市", &["220-0011", "220-0051", "231-0023", "232-0066", "244-0003"]),
        ("愛知県", "名古屋市", &["450-0002", "453-0014", "456-0006", "460-0008", "464-0075"]),
        ("北海道", "札幌市", &["060-0001", "060-0042", "064-0804", "065-0024", "003-0803"]),
        ("兵庫県", "神戸市", &["650-0001", "650-0044", "651-0096", "652-0802", "657-0029"]),
        ("福岡県", "福岡市", &["810-0001", "810-0041", "812-0011", "814-0001", "819-0025"]),
        ("広島県", "広島市", &["730-0011", "730-0051", "732-0052", "733-0003", "734-0007"]),
        ("宮城県", "仙台市", &["980-0011", "980-0803", "981-0933", "982-0011", "984-0015"]),
        ("沖縄県", "那覇市", &["900-0001", "900-0015", "900-0032", "902-0067", "903-0804"]),
    ];

    pub const JAPANESE_AREA_NAMES: &'static [&'static str] = &[
        "中央", "西", "東", "南", "北", "本町", "新町", "大通",
    ];

    // 日本手机号前缀（070, 080, 090系列）
    pub const JAPANESE_MOBILE_PREFIXES: &'static [&'static str] = &[
        "070", "080", "090",
    ];

    // 日本固定电话区号（城市，区号）
    pub const JAPANESE_LANDLINE_AREA_CODES: &'static [(&'static str, &'static str)] = &[
        ("東京", "03"),
        ("大阪", "06"),
        ("横浜", "045"),
        ("名古屋", "052"),
        ("札幌", "011"),
        ("神戸", "078"),
        ("京都", "075"),
        ("福岡", "092"),
        ("川崎", "044"),
        ("広島", "082"),
        ("仙台", "022"),
        ("千葉", "043"),
        ("那覇", "098"),
    ];

    // --- 韩国详细地址数据 ---
    // (道/市, 区/市, 真实邮编列表)
    pub const KOREAN_PROVINCE_CITY_POSTAL: &'static [(&'static str, &'static str, &'static [&'static str])] = &[
        ("서울특별시", "강남구", &["06001", "06132", "06234", "06349", "06524"]),
        ("서울특별시", "종로구", &["03001", "03142", "03188", "03196", "03089"]),
        ("서울특별시", "강서구", &["07501", "07626", "07774", "07803", "07957"]),
        ("부산광역시", "해운대구", &["48001", "48093", "48123", "48206", "48400"]),
        ("인천광역시", "남동구", &["21501", "21638", "21695", "21774", "21999"]),
        ("대구광역시", "수성구", &["42001", "42103", "42234", "42369", "42543"]),
        ("대전광역시", "유성구", &["34101", "34126", "34186", "34324", "34569"]),
        ("광주광역시", "동구", &["61401", "61452", "61499", "61623", "61752"]),
        ("경기도", "수원시", &["16001", "16234", "16309", "16459", "16677"]),
        ("경기도", "성남시", &["13101", "13209", "13315", "13488", "13591"]),
        ("제주특별자치도", "제주시", &["63001", "63122", "63241", "63309", "63644"]),
    ];

    pub const KOREAN_STREET_TYPES: &'static [&'static str] = &[
        "로 (ro)", "길 (gil)", "대로 (daero)",
    ];

    pub const KOREAN_AREA_NAMES: &'static [&'static str] = &[
        "중앙", "신", "강변", "역삼", "논현", "삼성", "청담",
    ];

    // 韩国手机号前缀（010系列）
    pub const KOREAN_MOBILE_PREFIXES: &'static [&'static str] = &[
        "010",
    ];

    // 韩国固定电话区号（城市，区号）
    pub const KOREAN_LANDLINE_AREA_CODES: &'static [(&'static str, &'static str)] = &[
        ("서울", "02"),      // 首尔
        ("부산", "051"),     // 釜山
        ("인천", "032"),     // 仁川
        ("대구", "053"),     // 大邱
        ("대전", "042"),     // 大田
        ("광주", "062"),     // 光州
        ("울산", "052"),     // 蔚山
        ("세종", "044"),     // 世宗
        ("수원", "031"),     // 水原
        ("제주", "064"),     // 济州
    ];

    // --- 印度数据 ---
    
    // 印度常见姓氏（多种文化背景）
    pub const INDIAN_LAST_NAMES: &'static [&'static str] = &[
        "Kumar", "Singh", "Sharma", "Patel", "Khan",
        "Gupta", "Verma", "Joshi", "Yadav", "Reddy",
        "Nair", "Menon", "Iyer", "Pillai", "Das",
        "Roy", "Chatterjee", "Mukherjee", "Banerjee", "Shah",
        "Desai", "Mehta", "Pandey", "Mishra", "Agarwal",
    ];

    // 印度常见男性名字
    pub const INDIAN_MALE_FIRST_NAMES: &'static [&'static str] = &[
        "Raj", "Arjun", "Rohan", "Amit", "Rahul",
        "Vikram", "Anil", "Suresh", "Rajesh", "Sanjay",
        "Ravi", "Krishna", "Anand", "Ajay", "Karan",
        "Aditya", "Aryan", "Dhruv", "Harsh", "Ishaan",
    ];

    // 印度常见女性名字
    pub const INDIAN_FEMALE_FIRST_NAMES: &'static [&'static str] = &[
        "Priya", "Anjali", "Pooja", "Neha", "Kavya",
        "Divya", "Shreya", "Aishwarya", "Riya", "Ananya",
        "Isha", "Meera", "Sanya", "Tanvi", "Diya",
        "Nisha", "Lakshmi", "Deepika", "Sneha", "Swati",
    ];

    // 印度主要城市和邮编
    pub const INDIAN_CITIES: &'static [&'static str] = &[
        "Mumbai", "Delhi", "Bangalore", "Hyderabad", "Chennai",
        "Kolkata", "Pune", "Ahmedabad", "Jaipur", "Lucknow",
    ];

    // 印度详细地址数据（城市，州，真实邮编列表）
    pub const INDIAN_CITY_STATE_POSTAL: &'static [(&'static str, &'static str, &'static [&'static str])] = &[
        ("Mumbai", "Maharashtra", &["400001", "400012", "400051", "400058", "400067", "400092"]),
        ("Delhi", "Delhi", &["110001", "110011", "110021", "110034", "110055", "110092"]),
        ("Bangalore", "Karnataka", &["560001", "560017", "560034", "560068", "560076", "560100"]),
        ("Hyderabad", "Telangana", &["500001", "500016", "500032", "500045", "500072", "500089"]),
        ("Chennai", "Tamil Nadu", &["600001", "600014", "600028", "600042", "600079", "600095"]),
        ("Kolkata", "West Bengal", &["700001", "700019", "700027", "700053", "700071", "700091"]),
        ("Pune", "Maharashtra", &["411001", "411014", "411028", "411037", "411046", "411057"]),
        ("Ahmedabad", "Gujarat", &["380001", "380015", "380022", "380051", "380061", "380081"]),
        ("Jaipur", "Rajasthan", &["302001", "302012", "302021", "302033", "302039", "302042"]),
        ("Lucknow", "Uttar Pradesh", &["226001", "226010", "226016", "226024", "226028", "226031"]),
    ];

    // 印度街道类型
    pub const INDIAN_STREET_TYPES: &'static [&'static str] = &[
        "Road", "Street", "Lane", "Avenue", "Marg", "Colony", "Nagar",
    ];

    pub const INDIAN_AREA_NAMES: &'static [&'static str] = &[
        "MG", "Gandhi", "Nehru", "Station", "Park", "Main", "Church", "Market",
    ];

    // 印度手机号前缀（10位数字，常见运营商）
    pub const INDIAN_MOBILE_PREFIXES: &'static [&'static str] = &[
        "98", "99", "97", "96", "95", "94", "93", "92", "91", "90",
        "89", "88", "87", "86", "85", "84", "83", "82", "81", "80",
        "70", "75", "76", "77", "78", "79",
    ];

    // 印度固定电话区号（城市，区号）
    pub const INDIAN_LANDLINE_AREA_CODES: &'static [(&'static str, &'static str)] = &[
        ("Mumbai", "022"),
        ("Delhi", "011"),
        ("Bangalore", "080"),
        ("Hyderabad", "040"),
        ("Chennai", "044"),
        ("Kolkata", "033"),
        ("Pune", "020"),
        ("Ahmedabad", "079"),
        ("Jaipur", "0141"),
        ("Lucknow", "0522"),
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

/// 代表一个伪造的日本地址
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JapaneseAddress {
    pub prefecture: &'static str,  // 都道府县
    pub city: &'static str,        // 市区
    pub street: String,            // 街道
    pub postal_code: String,       // 邮编（7位，格式：XXX-XXXX）
}

impl fmt::Display for JapaneseAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "〒{} {} {} {}",
            self.postal_code, self.prefecture, self.city, self.street
        )
    }
}

/// 代表一个伪造的韩国地址
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KoreanAddress {
    pub province: &'static str,    // 道/特别市
    pub city: &'static str,        // 区/市
    pub street: String,            // 街道
    pub postal_code: String,       // 邮编（5位）
}

impl fmt::Display for KoreanAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} (우편번호: {})",
            self.province, self.city, self.street, self.postal_code
        )
    }
}

/// 代表一个伪造的印度地址
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndianAddress {
    pub city: &'static str,        // 城市
    pub state: &'static str,       // 州
    pub street: String,            // 街道
    pub postal_code: String,       // 邮编（PIN码，6位）
}

impl fmt::Display for IndianAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {} - {}",
            self.street, self.city, self.state, self.postal_code
        )
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
    data::CHINESE_PROVINCE_CITY_POSTAL.choose(rng).unwrap_or(&("", "北京市", &["100000"])).1
}
pub fn japanese_city<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::JAPANESE_CITIES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的韩国姓氏
pub fn korean_last_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::KOREAN_LAST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的韩国男性名字
pub fn korean_male_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::KOREAN_MALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的韩国女性名字
pub fn korean_female_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::KOREAN_FEMALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的韩国城市
pub fn korean_city<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::KOREAN_CITIES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的日本姓氏
pub fn japanese_last_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::JAPANESE_LAST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的日本男性名字
pub fn japanese_male_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::JAPANESE_MALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的日本女性名字
pub fn japanese_female_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::JAPANESE_FEMALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}

/// 生成日本手机号（11位，格式：090-XXXX-XXXX）
pub fn japanese_phone_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefix = data::JAPANESE_MOBILE_PREFIXES.choose(rng).unwrap_or(&"090");
    let middle: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    let suffix: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    format!("{}-{}-{}", prefix, middle, suffix)
}

/// 生成日本固定电话号码
/// 格式：区号-XXXX-XXXX，例如：03-1234-5678（东京）
pub fn japanese_landline<R: Rng + ?Sized>(rng: &mut R) -> String {
    let (_city, area_code) = data::JAPANESE_LANDLINE_AREA_CODES.choose(rng)
        .unwrap_or(&("東京", "03"));
    
    // 日本固定电话：区号 + 8位号码（分为4-4格式）
    let part1: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    let part2: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    
    format!("{}-{}-{}", area_code, part1, part2)
}

/// 生成日本地址
pub fn japanese_address<R: Rng + ?Sized>(rng: &mut R) -> JapaneseAddress {
    // 1. 选择都道府县、城市、真实邮编列表
    let (prefecture, city, postal_codes) = data::JAPANESE_PREFECTURE_CITY_POSTAL.choose(rng)
        .unwrap_or(&("東京都", "新宿区", &["160-0001"]));
    
    // 2. 从真实邮编列表中选择一个
    let postal_code = postal_codes.choose(rng).unwrap_or(&"160-0001");
    
    // 3. 生成街道（如：中央2丁目3番地4号）
    let area = data::JAPANESE_AREA_NAMES.choose(rng).unwrap_or(&"中央");
    let chome = rng.gen_range(1..=5);  // 丁目
    let banchi = rng.gen_range(1..=20); // 番地
    let go = rng.gen_range(1..=30);     // 号
    let street = format!("{}{}丁目{}番地{}号", area, chome, banchi, go);
    
    JapaneseAddress {
        prefecture,
        city,
        street,
        postal_code: postal_code.to_string(),
    }
}

/// 生成韩国手机号（11位，格式：010-XXXX-XXXX）
pub fn korean_phone_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefix = data::KOREAN_MOBILE_PREFIXES.choose(rng).unwrap_or(&"010");
    let middle: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    let suffix: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    format!("{}-{}-{}", prefix, middle, suffix)
}

/// 生成韩国固定电话号码
/// 格式：区号-XXXX-XXXX，例如：02-1234-5678（首尔）
pub fn korean_landline<R: Rng + ?Sized>(rng: &mut R) -> String {
    let (_city, area_code) = data::KOREAN_LANDLINE_AREA_CODES.choose(rng)
        .unwrap_or(&("서울", "02"));
    
    // 韩国固定电话：区号 + 8位号码（分为4-4格式）
    let part1: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    let part2: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
    
    format!("{}-{}-{}", area_code, part1, part2)
}

/// 获取一个随机的印度姓氏
pub fn indian_last_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::INDIAN_LAST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的印度男性名字
pub fn indian_male_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::INDIAN_MALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的印度女性名字
pub fn indian_female_first_name<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::INDIAN_FEMALE_FIRST_NAMES.choose(rng).unwrap_or(&"")
}

/// 获取一个随机的印度城市
pub fn indian_city<R: Rng + ?Sized>(rng: &mut R) -> &'static str {
    data::INDIAN_CITIES.choose(rng).unwrap_or(&"")
}

/// 生成印度手机号（10位数字）
pub fn indian_phone_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefix = data::INDIAN_MOBILE_PREFIXES.choose(rng).unwrap_or(&"98");
    let suffix: String = (0..8).map(|_| rng.gen_range(0..=9).to_string()).collect();
    format!("{}{}", prefix, suffix)
}

/// 生成印度固定电话号码
/// 格式：区号-XXXXXXXX，例如：022-12345678（孟买）
pub fn indian_landline<R: Rng + ?Sized>(rng: &mut R) -> String {
    let (_city, area_code) = data::INDIAN_LANDLINE_AREA_CODES.choose(rng)
        .unwrap_or(&("Mumbai", "022"));
    
    // 印度固定电话：区号 + 8位号码
    let number: String = (0..8).map(|_| rng.gen_range(0..=9).to_string()).collect();
    
    format!("{}-{}", area_code, number)
}

/// 生成印度地址
pub fn indian_address<R: Rng + ?Sized>(rng: &mut R) -> IndianAddress {
    // 1. 选择城市、州、真实邮编列表
    let (city, state, postal_codes) = data::INDIAN_CITY_STATE_POSTAL.choose(rng)
        .unwrap_or(&("Mumbai", "Maharashtra", &["400001"]));
    
    // 2. 从真实邮编列表中选择一个
    let postal_code = postal_codes.choose(rng).unwrap_or(&"400001");
    
    // 3. 生成街道（如：MG Road 123）
    let area = data::INDIAN_AREA_NAMES.choose(rng).unwrap_or(&"Main");
    let street_type = data::INDIAN_STREET_TYPES.choose(rng).unwrap_or(&"Road");
    let number = rng.gen_range(1..=999);
    let street = format!("{} {} {}", area, street_type, number);
    
    IndianAddress {
        city,
        state,
        street,
        postal_code: postal_code.to_string(),
    }
}

/// 生成韩国地址
pub fn korean_address<R: Rng + ?Sized>(rng: &mut R) -> KoreanAddress {
    // 1. 选择道/市、区、真实邮编列表
    let (province, city, postal_codes) = data::KOREAN_PROVINCE_CITY_POSTAL.choose(rng)
        .unwrap_or(&("서울특별시", "강남구", &["06001"]));
    
    // 2. 从真实邮编列表中选择一个
    let postal_code = postal_codes.choose(rng).unwrap_or(&"06001");
    
    // 3. 生成街道
    let area = data::KOREAN_AREA_NAMES.choose(rng).unwrap_or(&"중앙");
    let street_type = data::KOREAN_STREET_TYPES.choose(rng).unwrap_or(&"로 (ro)");
    let number = rng.gen_range(1..=500);
    let street = format!("{}{} {}", area, street_type, number);
    
    KoreanAddress {
        province,
        city,
        street,
        postal_code: postal_code.to_string(),
    }
}


// --- 电话、地址、公司 ---

/// 获取一个随机的中国大陆手机号 (String)
pub fn chinese_phone_number<R: Rng + ?Sized>(rng: &mut R) -> String {
    let prefix = data::CHINESE_MOBILE_PREFIXES.choose(rng).unwrap_or(&"138");
    let suffix: String = (0..8).map(|_| rng.gen_range(0..=9).to_string()).collect();
    format!("{}{}", prefix, suffix)
}

/// 生成中国固定电话号码
/// 格式：区号-号码，例如：010-12345678（北京）、0755-12345678（深圳）
pub fn chinese_landline<R: Rng + ?Sized>(rng: &mut R) -> String {
    let (_city, area_code) = data::CHINESE_LANDLINE_AREA_CODES.choose(rng)
        .unwrap_or(&("北京", "010"));
    
    // 根据区号长度生成对应长度的号码（3位区号配8位号码，4位区号配7位号码）
    let number_length = if area_code.len() == 3 { 8 } else { 7 };
    let number: String = (0..number_length)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect();
    
    format!("{}-{}", area_code, number)
}

/// 获取一个随机的中国地址 (ChineseAddress 结构体)
pub fn chinese_address<R: Rng + ?Sized>(rng: &mut R) -> ChineseAddress {
    // 1. 选择省份、城市、真实邮编列表
    let (province, city, postal_codes) = data::CHINESE_PROVINCE_CITY_POSTAL.choose(rng)
        .unwrap_or(&("北京市", "北京市", &["100000"]));
    
    // 2. 从真实邮编列表中选择一个
    let postal_code = postal_codes.choose(rng).unwrap_or(&"100000");
    
    // 3. 生成街道
    let road_name = data::CHINESE_ROAD_NAMES.choose(rng).unwrap_or(&"人民");
    let suffix = data::CHINESE_STREET_SUFFIXES.choose(rng).unwrap_or(&"路");
    let number = rng.gen_range(1..=999);
    let street = format!("{}{}{}号", road_name, suffix, number);
    
    ChineseAddress {
        province,
        city,
        street,
        postal_code: postal_code.to_string(),
    }
}

/// 获取一个随机的中国公司名 (ChineseCompany 结构体)
pub fn chinese_company<R: Rng + ?Sized>(rng: &mut R) -> ChineseCompany {
    // 随机选择一个城市名，并去掉"市"
    let city = data::CHINESE_PROVINCE_CITY_POSTAL.choose(rng)
        .unwrap_or(&("", "北京", &["100000"])).1.replace("市", "");
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

/// 生成中国身份证号码（18位）
/// 格式：地区码(6位) + 出生日期(8位) + 顺序码(3位) + 校验码(1位)
pub fn chinese_id_card<R: Rng + ?Sized>(rng: &mut R) -> String {
    // 1. 地区码（前6位）
    let area_code = data::ID_CARD_AREA_CODES.choose(rng).unwrap_or(&"110000");
    
    // 2. 出生日期（8位：YYYYMMDD）
    let year = rng.gen_range(1950..=2005);
    let month = rng.gen_range(1..=12);
    let day = match month {
        2 => rng.gen_range(1..=28),
        4 | 6 | 9 | 11 => rng.gen_range(1..=30),
        _ => rng.gen_range(1..=31),
    };
    let birthday = format!("{:04}{:02}{:02}", year, month, day);
    
    // 3. 顺序码（3位，奇数为男，偶数为女）
    let sequence = rng.gen_range(100..=999);
    
    // 4. 计算校验码
    let id_without_check = format!("{}{}{:03}", area_code, birthday, sequence);
    let check_code = calculate_id_check_code(&id_without_check);
    
    format!("{}{}", id_without_check, check_code)
}

/// 计算身份证校验码
fn calculate_id_check_code(id_17: &str) -> char {
    let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let check_codes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    
    let sum: u32 = id_17.chars()
        .enumerate()
        .map(|(i, c)| c.to_digit(10).unwrap_or(0) * weights[i])
        .sum();
    
    check_codes[(sum % 11) as usize]
}

/// 生成中国车牌号
/// 格式：省份简称 + 字母 + 5位数字/字母组合
pub fn chinese_license_plate<R: Rng + ?Sized>(rng: &mut R) -> String {
    let province = data::LICENSE_PLATE_PROVINCES.choose(rng).unwrap_or(&("京", "北京")).0;
    let letter = (b'A' + rng.gen_range(0..26)) as char;
    
    // 生成5位数字/字母组合（通常前几位是字母，后面是数字）
    let mut plate_num = String::new();
    for i in 0..5 {
        if i < 2 && rng.gen_bool(0.3) {
            // 前面可能是字母
            plate_num.push((b'A' + rng.gen_range(0..26)) as char);
        } else {
            // 主要是数字
            plate_num.push_str(&rng.gen_range(0..=9).to_string());
        }
    }
    
    format!("{}{}{}", province, letter, plate_num)
}

/// 生成电子邮件地址
pub fn email<R: Rng + ?Sized>(rng: &mut R) -> String {
    let domain = data::EMAIL_DOMAINS.choose(rng).unwrap_or(&"qq.com");
    
    // 生成用户名（可能是拼音、英文或数字组合）
    let username_type = rng.gen_range(0..3);
    let username = match username_type {
        0 => {
            // 使用姓名拼音（简化版）
            let last = chinese_last_name(rng);
            let first = chinese_first_name(rng);
            format!("{}{}{}", 
                romanize_simple(last), 
                romanize_simple(first),
                rng.gen_range(100..9999)
            )
        },
        1 => {
            // 纯数字
            format!("{}", rng.gen_range(100000000..999999999))
        },
        _ => {
            // 字母+数字
            let prefix: String = (0..rng.gen_range(5..10))
                .map(|_| (b'a' + rng.gen_range(0..26)) as char)
                .collect();
            format!("{}{}", prefix, rng.gen_range(100..9999))
        }
    };
    
    format!("{}@{}", username.to_lowercase(), domain)
}

/// 简单的汉字转拼音（仅用于演示，实际应使用专业库）
fn romanize_simple(chinese: &str) -> String {
    // 这是一个非常简化的实现，实际应该使用 pinyin 库
    match chinese {
        "李" => "li", "王" => "wang", "张" => "zhang", "刘" => "liu",
        "陈" => "chen", "杨" => "yang", "赵" => "zhao", "黄" => "huang",
        "周" => "zhou", "吴" => "wu", "徐" => "xu", "孙" => "sun",
        _ => "user"
    }.to_string()
}


// --- 4. 批量生成和工具函数 ---

/// 生成多个指定类型的伪数据
pub fn generate_multiple<T, R, F>(count: usize, rng: &mut R, generator: F) -> Vec<T>
where
    R: Rng + ?Sized,
    F: Fn(&mut R) -> T,
{
    (0..count).map(|_| generator(rng)).collect()
}

/// 生成一个完整的人员信息结构
#[derive(Debug, Clone)]
pub struct PersonInfo {
    pub name: String,
    pub phone: String,
    pub id_card: String,
    pub email: String,
    pub address: ChineseAddress,
    pub company: ChineseCompany,
}

impl fmt::Display for PersonInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "姓名: {}\n手机: {}\n身份证: {}\n邮箱: {}\n地址: {}\n公司: {}",
            self.name, self.phone, self.id_card, self.email, self.address, self.company
        )
    }
}

/// 生成一个完整的中国人员信息
pub fn chinese_person<R: Rng + ?Sized>(rng: &mut R) -> PersonInfo {
    let last_name = chinese_last_name(rng);
    let first_name = chinese_first_name(rng);
    let name = format!("{}{}", last_name, first_name);
    
    PersonInfo {
        name,
        phone: chinese_phone_number(rng),
        id_card: chinese_id_card(rng),
        email: email(rng),
        address: chinese_address(rng),
        company: chinese_company(rng),
    }
}

/// 生成多个人员信息
pub fn chinese_persons<R: Rng + ?Sized>(count: usize, rng: &mut R) -> Vec<PersonInfo> {
    generate_multiple(count, rng, chinese_person)
}

/// 日本人员信息结构
#[derive(Debug, Clone)]
pub struct JapanesePersonInfo {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: JapaneseAddress,
}

impl fmt::Display for JapanesePersonInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "氏名: {}\n電話: {}\nメール: {}\n住所: {}",
            self.name, self.phone, self.email, self.address
        )
    }
}

/// 生成一个完整的日本人员信息
pub fn japanese_person<R: Rng + ?Sized>(rng: &mut R) -> JapanesePersonInfo {
    let last_name = japanese_last_name(rng);
    let first_name = japanese_male_first_name(rng); // 或随机选择男女名字
    let name = format!("{}{}", last_name, first_name);
    
    JapanesePersonInfo {
        name,
        phone: japanese_phone_number(rng),
        email: email(rng),
        address: japanese_address(rng),
    }
}

/// 生成多个日本人员信息
pub fn japanese_persons<R: Rng + ?Sized>(count: usize, rng: &mut R) -> Vec<JapanesePersonInfo> {
    generate_multiple(count, rng, japanese_person)
}

/// 韩国人员信息结构
#[derive(Debug, Clone)]
pub struct KoreanPersonInfo {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: KoreanAddress,
}

impl fmt::Display for KoreanPersonInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "이름: {}\n전화: {}\n이메일: {}\n주소: {}",
            self.name, self.phone, self.email, self.address
        )
    }
}

/// 生成一个完整的韩国人员信息
pub fn korean_person<R: Rng + ?Sized>(rng: &mut R) -> KoreanPersonInfo {
    let last_name = korean_last_name(rng);
    let first_name = korean_male_first_name(rng); // 或随机选择男女名字
    let name = format!("{} {}", last_name, first_name);
    
    KoreanPersonInfo {
        name,
        phone: korean_phone_number(rng),
        email: email(rng),
        address: korean_address(rng),
    }
}

/// 生成多个韩国人员信息
pub fn korean_persons<R: Rng + ?Sized>(count: usize, rng: &mut R) -> Vec<KoreanPersonInfo> {
    generate_multiple(count, rng, korean_person)
}

/// 印度人员信息结构
#[derive(Debug, Clone)]
pub struct IndianPersonInfo {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub address: IndianAddress,
}

impl fmt::Display for IndianPersonInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nPhone: {}\nEmail: {}\nAddress: {}",
            self.name, self.phone, self.email, self.address
        )
    }
}

/// 生成一个完整的印度人员信息
pub fn indian_person<R: Rng + ?Sized>(rng: &mut R) -> IndianPersonInfo {
    let first_name = indian_male_first_name(rng); // 或随机选择男女名字
    let last_name = indian_last_name(rng);
    let name = format!("{} {}", first_name, last_name);
    
    IndianPersonInfo {
        name,
        phone: indian_phone_number(rng),
        email: email(rng),
        address: indian_address(rng),
    }
}

/// 生成多个印度人员信息
pub fn indian_persons<R: Rng + ?Sized>(count: usize, rng: &mut R) -> Vec<IndianPersonInfo> {
    generate_multiple(count, rng, indian_person)
}


// --- 5. 定义 FakeAsia Trait ---

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

impl FakeAsia for JapaneseAddress {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self {
        japanese_address(rng)
    }
}

impl FakeAsia for KoreanAddress {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self {
        korean_address(rng)
    }
}

impl FakeAsia for IndianAddress {
    fn fake_asia<R: Rng + ?Sized>(rng: &mut R) -> Self {
        indian_address(rng)
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

    #[test]
    fn test_get_korean_last_name() {
        let mut rng = rand::thread_rng();
        let name = korean_last_name(&mut rng);
        println!("随机韩国姓氏: {}", name);
        assert!(!name.is_empty());
        assert!(data::KOREAN_LAST_NAMES.contains(&name));
    }

    // --- 新功能测试 ---

    #[test]
    fn test_chinese_id_card() {
        let mut rng = rand::thread_rng();
        let id = chinese_id_card(&mut rng);
        
        println!("随机身份证号: {}", id);
        assert_eq!(id.len(), 18); // 身份证号18位
        
        // 验证前17位都是数字
        assert!(id[..17].chars().all(|c| c.is_ascii_digit()));
        
        // 最后一位是数字或X
        let last_char = id.chars().last().unwrap();
        assert!(last_char.is_ascii_digit() || last_char == 'X');
    }

    #[test]
    fn test_chinese_license_plate() {
        let mut rng = rand::thread_rng();
        let plate = chinese_license_plate(&mut rng);
        
        println!("随机车牌号: {}", plate);
        // 车牌号格式：省份简称(1个汉字) + 字母(1) + 5位数字/字母 = 7个字符
        assert_eq!(plate.chars().count(), 7);
    }

    #[test]
    fn test_email() {
        let mut rng = rand::thread_rng();
        let email_addr = email(&mut rng);
        
        println!("随机邮箱: {}", email_addr);
        assert!(email_addr.contains('@'));
        assert!(email_addr.split('@').count() == 2);
        
        let parts: Vec<&str> = email_addr.split('@').collect();
        assert!(!parts[0].is_empty()); // 用户名不为空
        assert!(!parts[1].is_empty()); // 域名不为空
    }

    #[test]
    fn test_japanese_names() {
        let mut rng = rand::thread_rng();
        
        let last = japanese_last_name(&mut rng);
        let male = japanese_male_first_name(&mut rng);
        let female = japanese_female_first_name(&mut rng);
        
        println!("日本姓氏: {}", last);
        println!("日本男性名字: {}{}", last, male);
        println!("日本女性名字: {}{}", last, female);
        
        assert!(!last.is_empty());
        assert!(!male.is_empty());
        assert!(!female.is_empty());
    }

    #[test]
    fn test_korean_names() {
        let mut rng = rand::thread_rng();
        
        let last = korean_last_name(&mut rng);
        let male = korean_male_first_name(&mut rng);
        let female = korean_female_first_name(&mut rng);
        let city = korean_city(&mut rng);
        
        println!("韩国姓氏: {}", last);
        println!("韩国男性名字: {} {}", last, male);
        println!("韩国女性名字: {} {}", last, female);
        println!("韩国城市: {}", city);
        
        assert!(!last.is_empty());
        assert!(!male.is_empty());
        assert!(!female.is_empty());
        assert!(!city.is_empty());
    }

    #[test]
    fn test_comprehensive_person_data() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 生成完整的中国人信息 ===");
        let name: String = FakeAsia::fake();
        let phone = chinese_phone_number(&mut rng);
        let id_card = chinese_id_card(&mut rng);
        let addr: ChineseAddress = FakeAsia::fake();
        let email_addr = email(&mut rng);
        let company: ChineseCompany = FakeAsia::fake();
        let plate = chinese_license_plate(&mut rng);
        
        println!("姓名: {}", name);
        println!("手机号: {}", phone);
        println!("身份证: {}", id_card);
        println!("地址: {}", addr);
        println!("邮箱: {}", email_addr);
        println!("公司: {}", company);
        println!("车牌: {}", plate);
        
        assert!(!name.is_empty());
        assert_eq!(phone.len(), 11);
        assert_eq!(id_card.len(), 18);
        assert!(email_addr.contains('@'));
    }

    #[test]
    fn test_person_info_struct() {
        let mut rng = rand::thread_rng();
        let person = chinese_person(&mut rng);
        
        println!("\n=== PersonInfo 结构体 ===");
        println!("{}", person);
        
        assert!(!person.name.is_empty());
        assert_eq!(person.phone.len(), 11);
        assert_eq!(person.id_card.len(), 18);
        assert!(person.email.contains('@'));
    }

    #[test]
    fn test_batch_generation() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 批量生成5个人员信息 ===");
        let persons = chinese_persons(5, &mut rng);
        
        assert_eq!(persons.len(), 5);
        
        for (i, person) in persons.iter().enumerate() {
            println!("\n--- 人员 {} ---", i + 1);
            println!("{}", person);
            assert!(!person.name.is_empty());
        }
    }

    #[test]
    fn test_generate_multiple() {
        let mut rng = rand::thread_rng();
        
        // 测试生成多个手机号
        let phones = generate_multiple(10, &mut rng, chinese_phone_number);
        assert_eq!(phones.len(), 10);
        
        for phone in &phones {
            assert_eq!(phone.len(), 11);
        }
        
        println!("生成的10个手机号: {:?}", phones);
    }

    // --- 日本数据测试 ---

    #[test]
    fn test_japanese_phone_number() {
        let mut rng = rand::thread_rng();
        let phone = japanese_phone_number(&mut rng);
        
        println!("日本手机号: {}", phone);
        assert!(phone.contains('-'));
        assert!(phone.starts_with("070") || phone.starts_with("080") || phone.starts_with("090"));
    }

    #[test]
    fn test_japanese_address() {
        let mut rng = rand::thread_rng();
        let addr = japanese_address(&mut rng);
        
        println!("日本地址: {}", addr);
        assert!(addr.postal_code.contains('-'));
        assert!(addr.street.contains("丁目"));
        assert!(!addr.prefecture.is_empty());
    }

    #[test]
    fn test_japanese_person_info() {
        let mut rng = rand::thread_rng();
        let person = japanese_person(&mut rng);
        
        println!("\n=== 日本人员信息 ===");
        println!("{}", person);
        
        assert!(!person.name.is_empty());
        assert!(person.phone.contains('-'));
        assert!(person.email.contains('@'));
    }

    #[test]
    fn test_japanese_persons_batch() {
        let mut rng = rand::thread_rng();
        let persons = japanese_persons(3, &mut rng);
        
        println!("\n=== 批量生成3个日本人员 ===");
        assert_eq!(persons.len(), 3);
        
        for (i, person) in persons.iter().enumerate() {
            println!("\n--- 人員 {} ---", i + 1);
            println!("{}", person);
        }
    }

    #[test]
    fn test_japanese_trait() {
        let addr: JapaneseAddress = FakeAsia::fake();
        println!("通过 Trait 生成的日本地址: {}", addr);
        assert!(!addr.prefecture.is_empty());
    }

    // --- 韩国数据测试 ---

    #[test]
    fn test_korean_phone_number() {
        let mut rng = rand::thread_rng();
        let phone = korean_phone_number(&mut rng);
        
        println!("韩国手机号: {}", phone);
        assert!(phone.contains('-'));
        assert!(phone.starts_with("010"));
    }

    #[test]
    fn test_korean_address() {
        let mut rng = rand::thread_rng();
        let addr = korean_address(&mut rng);
        
        println!("韩国地址: {}", addr);
        assert_eq!(addr.postal_code.len(), 5);
        assert!(!addr.province.is_empty());
    }

    #[test]
    fn test_korean_person_info() {
        let mut rng = rand::thread_rng();
        let person = korean_person(&mut rng);
        
        println!("\n=== 韩国人员信息 ===");
        println!("{}", person);
        
        assert!(!person.name.is_empty());
        assert!(person.phone.contains('-'));
        assert!(person.email.contains('@'));
    }

    #[test]
    fn test_korean_persons_batch() {
        let mut rng = rand::thread_rng();
        let persons = korean_persons(3, &mut rng);
        
        println!("\n=== 批量生成3个韩国人员 ===");
        assert_eq!(persons.len(), 3);
        
        for (i, person) in persons.iter().enumerate() {
            println!("\n--- 사람 {} ---", i + 1);
            println!("{}", person);
        }
    }

    #[test]
    fn test_korean_trait() {
        let addr: KoreanAddress = FakeAsia::fake();
        println!("通过 Trait 生成的韩国地址: {}", addr);
        assert!(!addr.province.is_empty());
    }

    // --- 综合测试：三国对比 ---

    #[test]
    fn test_three_countries_comparison() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 中日韩人员信息对比 ===\n");
        
        println!("【中国人员】");
        let cn_person = chinese_person(&mut rng);
        println!("{}\n", cn_person);
        
        println!("【日本人員】");
        let jp_person = japanese_person(&mut rng);
        println!("{}\n", jp_person);
        
        println!("【韩国사람】");
        let kr_person = korean_person(&mut rng);
        println!("{}", kr_person);
    }

    // --- 固定电话测试 ---

    #[test]
    fn test_chinese_landline() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 中国固定电话测试 ===");
        for _ in 0..5 {
            let landline = chinese_landline(&mut rng);
            println!("  {}", landline);
            assert!(landline.contains('-'));
        }
    }

    #[test]
    fn test_japanese_landline() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 日本固定电话测试 ===");
        for _ in 0..5 {
            let landline = japanese_landline(&mut rng);
            println!("  {}", landline);
            assert!(landline.contains('-'));
            assert_eq!(landline.split('-').count(), 3);
        }
    }

    #[test]
    fn test_korean_landline() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 韩国固定电话测试 ===");
        for _ in 0..5 {
            let landline = korean_landline(&mut rng);
            println!("  {}", landline);
            assert!(landline.contains('-'));
            assert_eq!(landline.split('-').count(), 3);
        }
    }

    #[test]
    fn test_all_phone_types() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 三国电话号码对比 ===\n");
        
        println!("【中国】");
        println!("  手机: {}", chinese_phone_number(&mut rng));
        println!("  座机: {}", chinese_landline(&mut rng));
        
        println!("\n【日本】");
        println!("  携帯: {}", japanese_phone_number(&mut rng));
        println!("  固定: {}", japanese_landline(&mut rng));
        
        println!("\n【韩国】");
        println!("  휴대폰: {}", korean_phone_number(&mut rng));
        println!("  전화: {}", korean_landline(&mut rng));
    }

    // --- 印度数据测试 ---

    #[test]
    fn test_indian_names() {
        let mut rng = rand::thread_rng();
        
        let last = indian_last_name(&mut rng);
        let male = indian_male_first_name(&mut rng);
        let female = indian_female_first_name(&mut rng);
        
        println!("印度姓氏: {}", last);
        println!("印度男性名字: {} {}", male, last);
        println!("印度女性名字: {} {}", female, last);
        
        assert!(!last.is_empty());
        assert!(!male.is_empty());
        assert!(!female.is_empty());
    }

    #[test]
    fn test_indian_phone_number() {
        let mut rng = rand::thread_rng();
        let phone = indian_phone_number(&mut rng);
        
        println!("印度手机号: {}", phone);
        assert_eq!(phone.len(), 10);
    }

    #[test]
    fn test_indian_landline() {
        let mut rng = rand::thread_rng();
        let landline = indian_landline(&mut rng);
        
        println!("印度固定电话: {}", landline);
        assert!(landline.contains('-'));
    }

    #[test]
    fn test_indian_address() {
        let mut rng = rand::thread_rng();
        let addr = indian_address(&mut rng);
        
        println!("印度地址: {}", addr);
        assert_eq!(addr.postal_code.len(), 6);
        assert!(!addr.city.is_empty());
        assert!(!addr.state.is_empty());
    }

    #[test]
    fn test_indian_person_info() {
        let mut rng = rand::thread_rng();
        let person = indian_person(&mut rng);
        
        println!("\n=== 印度人员信息 ===");
        println!("{}", person);
        
        assert!(!person.name.is_empty());
        assert_eq!(person.phone.len(), 10);
        assert!(person.email.contains('@'));
    }

    #[test]
    fn test_indian_persons_batch() {
        let mut rng = rand::thread_rng();
        let persons = indian_persons(3, &mut rng);
        
        println!("\n=== 批量生成3个印度人员 ===");
        assert_eq!(persons.len(), 3);
        
        for (i, person) in persons.iter().enumerate() {
            println!("\n--- Person {} ---", i + 1);
            println!("{}", person);
        }
    }

    #[test]
    fn test_indian_trait() {
        let addr: IndianAddress = FakeAsia::fake();
        println!("通过 Trait 生成的印度地址: {}", addr);
        assert!(!addr.city.is_empty());
    }

    #[test]
    fn test_four_countries_comparison() {
        let mut rng = rand::thread_rng();
        
        println!("\n=== 中日韩印人员信息对比 ===\n");
        
        println!("【中国人员】");
        let cn_person = chinese_person(&mut rng);
        println!("{}\n", cn_person);
        
        println!("【日本人員】");
        let jp_person = japanese_person(&mut rng);
        println!("{}\n", jp_person);
        
        println!("【韩国사람】");
        let kr_person = korean_person(&mut rng);
        println!("{}\n", kr_person);
        
        println!("【Indian Person】");
        let in_person = indian_person(&mut rng);
        println!("{}", in_person);
    }
}