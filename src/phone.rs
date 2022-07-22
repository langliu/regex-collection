use regex::Regex;

/// 手机号(mobile phone)中国(宽松), 只要是13,14,15,16,17,18,19开头即可
///
/// # 例子
///
/// ```
/// use regex_collection::phone;
///
/// let text = String::from("18328073000");
/// phone::is_phone(&text);
/// ```
pub fn is_phone(text: &str) -> bool {
    let re = Regex::new(r"^(?:(?:\+|00)86)?1[3-9]\d{9}$").unwrap();
    re.is_match(text)
}

/// 手机号(mobile phone)中国(最宽松), 只要是1开头即可
///
/// # 例子
///
/// ```
/// use regex_collection::phone;
///
/// let text = String::from("18328073000");
/// phone::is_phone_easy(&text);
/// ```
pub fn is_phone_easy(text: &str) -> bool {
    let re = Regex::new(r"^(?:(?:\+|00)86)?1\d{10}$").unwrap();
    re.is_match(text)
}

/// 座机(tel phone)电话
///
/// # 例子
///
/// ```
/// use regex_collection::phone;
///
/// let text = String::from("0817-12341234");
/// phone::is_tel_phone(&text);
/// ```
pub fn is_tel_phone(text: &str) -> bool {
    let re = Regex::new(r"^(?:(?:\d{3}-)?\d{8}|^(?:\d{4}-)?\d{7,8})(?:-\d+)?$").unwrap();
    re.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_phone() {
        let phone_number = String::from("18328073000");
        let phone_number_2 = String::from("28317890987");
        let phone_number_3 = String::from("+8618317890987");
        let phone_number_4 = String::from("008618317890987");
        assert_eq!(is_phone(&phone_number), true);
        assert_eq!(is_phone(&phone_number_3), true, "+86");
        assert_eq!(is_phone(&phone_number_2), false);
        assert_eq!(is_phone(&phone_number_4), true, "0086");
    }

    #[test]
    fn test_is_phone_easy() {
        let phone_number = String::from("18328073000");
        let phone_number_2 = String::from("28317890987");
        let phone_number_3 = String::from("+8618317890987");
        let phone_number_4 = String::from("008618317890987");
        let phone_number_5 = String::from("008612317890987");
        assert_eq!(is_phone_easy(&phone_number), true);
        assert_eq!(is_phone_easy(&phone_number_3), true, "+86");
        assert_eq!(is_phone_easy(&phone_number_2), false);
        assert_eq!(is_phone_easy(&phone_number_4), true, "0086");
        assert_eq!(is_phone_easy(&phone_number_5), true, "12开头的手机号");
    }

    #[test]
    fn test_is_tel_phone() {
        let phone_number = String::from("0817-12341234");
        let phone_number_2 = String::from("0817-12341234-1233");
        let phone_number_3 = String::from("17890981");
        let phone_number_4 = String::from("18317890987");
        assert_eq!(is_tel_phone(&phone_number), true, "座机号码");
        assert_eq!(is_tel_phone(&phone_number_2), true, "座机号码+内部短号");
        assert_eq!(is_tel_phone(&phone_number_3), true, "不带区号的座机号码");
        assert_eq!(is_tel_phone(&phone_number_4), false, "手机号码");
    }
}
