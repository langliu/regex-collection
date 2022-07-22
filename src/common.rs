use regex::Regex;

/// 火车车次
///
/// # Examples
///
/// ```
/// let train_number = String::from("G14");
/// let train_number_2 = String::from("G11234");
///
/// assert_eq!(regex_collection::common::is_train_number(&train_number), true);
/// assert_eq!(regex_collection::common::is_train_number(&train_number_2), false);
/// ```
pub fn is_train_number(train_number: &str) -> bool {
    let re = Regex::new(r"^[GCDZTSPKXLY1-9]\d{1,4}$").unwrap();
    re.is_match(train_number)
}

/// 手机机身码(IMEI)
///
/// # Examples
///
/// ```
/// let imei = String::from("122457589068754");
/// let imei_2 = String::from("G11234");
///
/// assert_eq!(regex_collection::common::is_imei(&imei), true);
/// assert_eq!(regex_collection::common::is_imei(&imei_2), false);
/// ```
pub fn is_imei(imei: &str) -> bool {
    let re = Regex::new(r"^\d{15,17}$").unwrap();
    re.is_match(imei)
}

/// 网址
///
/// # Examples
///
/// ```
/// let url = String::from("http://baidu.com:8081");
/// let url_2 = String::from("http://baidu.com/asdf/#/234-123");
///
/// assert_eq!(regex_collection::common::is_url(&url), true);
/// assert_eq!(regex_collection::common::is_url(&url_2), true);
/// ```
pub fn is_url(url: &str) -> bool {
    let re = Regex::new(r"^(((ht|f)tps?)://)?([^!@#$%^&*?.\s-]([^!@#$%^&*?.\s]{0,63}[^!@#$%^&*?.\s])?\.)+[a-z]{2,6}/?").unwrap();
    re.is_match(url)
}

/// 必须带端口号的网址(或ip)
///
/// # Examples
///
/// ```
/// let url = String::from("http://baidu.com:8081");
/// let url_2 = String::from("http://baidu.com");
///
/// assert_eq!(regex_collection::common::is_url_with_port(&url), true);
/// assert_eq!(regex_collection::common::is_url_with_port(&url_2), false);
/// ```
pub fn is_url_with_port(url: &str) -> bool {
    let re = Regex::new(r"^((ht|f)tps?://)?[\w-]+(\.[\w-]+)+:\d{1,5}/?$").unwrap();
    re.is_match(url)
}

/// 统一社会信用代码
///
/// # Examples
///
/// ```
/// let url = String::from("9A1234568378908767");
/// let url_2 = String::from("http://baidu.com");
///
/// assert_eq!(regex_collection::common::is_unified_social_credit_code(&url), true);
/// assert_eq!(regex_collection::common::is_unified_social_credit_code(&url_2), false);
/// ```
pub fn is_unified_social_credit_code(str: &str) -> bool {
    let re = Regex::new(r"^[0-9A-HJ-NPQRTUWXY]{2}\d{6}[0-9A-HJ-NPQRTUWXY]{10}$").unwrap();
    re.is_match(str)
}

/// 视频(video)链接地址(支持swf|avi|flv|mpg|rm|mov|wav|asf|3gp|mkv|rmvb|mp4格式)
///
/// # Examples
///
/// ```
/// let url = String::from("http://ASD.COM/asd/asd");
/// let url_2 = String::from("http://baidu.com/ewe.mov");
///
/// assert_eq!(regex_collection::common::is_video_url(&url), false);
/// assert_eq!(regex_collection::common::is_video_url(&url_2), true);
/// ```
pub fn is_video_url(url: &str) -> bool {
    let re =
        Regex::new(r"^https?://(.+/)+.+(\.(swf|avi|flv|mpg|rm|mov|wav|asf|3gp|mkv|rmvb|mp4))$")
            .unwrap();
    re.is_match(url)
}

/// 图片(video)链接地址(支持gif|png|jpg|jpeg|webp|svg|psd|bmp|tif格式)
///
/// # Examples
///
/// ```
/// let url = String::from("http://ASD.COM/asd/asd.psd");
/// let url_2 = String::from("https://baidu.com/ewe.jpg");
///
/// assert_eq!(regex_collection::common::is_image_url(&url), false);
/// assert_eq!(regex_collection::common::is_image_url(&url_2), true);
/// ```
pub fn is_image_url(url: &str) -> bool {
    let re = Regex::new(r"^https?://(.+/)+.+(gif|png|jpg|jpeg|webp|svg|psd|bmp|tif)$").unwrap();
    re.is_match(url)
}

/// base64格式
///
/// # Examples
///
/// ```
/// let text = String::from("http://ASD.COM/asd/asd.psd");
/// let text_2 = String::from("data:image/gif;base64,dsafasdfasddGhpcyBpcyBhIGV4YW1wbGU=");
///
/// assert_eq!(regex_collection::common::is_base64(&text), false);
/// assert_eq!(regex_collection::common::is_base64(&text_2), true);
/// ```
pub fn is_base64(url: &str) -> bool {
    let re = Regex::new(r"^\s*data:(?:[a-z]+/[a-z0-9-+.]+(?:;[a-z-]+=[a-z0-9-]+)?)?(?:;base64)?,([a-z0-9!$&',()*+;=\-._~:@/?%\s]*?)\s*$").unwrap();
    re.is_match(url)
}

/// 银行卡号
///
/// # Examples
///
/// ```
/// let text = String::from("http://ASD.COM/asd/asd.psd");
/// let text_2 = String::from("data:image/gif;base64,dsafasdfasddGhpcyBpcyBhIGV4YW1wbGU=");
///
/// assert_eq!(regex_collection::common::is_credit_card_numbers(&text), false);
/// assert_eq!(regex_collection::common::is_credit_card_numbers(&text_2), true);
/// ```
pub fn is_credit_card_numbers(url: &str) -> bool {
    let re = Regex::new(r"^[1-9]\d{9,29}$").unwrap();
    re.is_match(url)
}
