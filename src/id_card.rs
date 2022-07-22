use regex::Regex;

/// 身份证号(2代,18位数字),最后一位是校验位,可能为数字或字符X
///
/// # Examples
///
/// ```
/// let id_card = String::from("511324199612163557");
///
/// assert_eq!(regex_collection::id_card::is_id_card(&id_card), true);
/// ```
pub fn is_id_card(card_number: &str) -> bool {
    let re = Regex::new(
        r"^[1-9]\d{5}(?:18|19|20)\d{2}(?:0[1-9]|10|11|12)(?:0[1-9]|[1-2]\d|30|31)\d{3}[\dXx]$",
    )
    .unwrap();
    re.is_match(card_number)
}

/// 香港身份证
///
/// # Examples
///
/// ```
/// let id_card = String::from("K034178(2)");
///
/// assert_eq!(regex_collection::id_card::is_hongkong_id_card(&id_card), true);
/// ```
pub fn is_hongkong_id_card(card_number: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z]\d{6}\([\dA]\)$").unwrap();
    re.is_match(card_number)
}

/// 澳门身份证
///
/// # Examples
///
/// ```
/// let id_card = String::from("5534178(2)");
///
/// assert_eq!(regex_collection::id_card::is_macau_id_card(&id_card), true);
/// ```
pub fn is_macau_id_card(card_number: &str) -> bool {
    let re = Regex::new(r"^[1|5|7]\d{6}\(\d\)$").unwrap();
    re.is_match(card_number)
}

/// 台湾身份证
///
/// # Examples
///
/// ```
/// let id_card = String::from("K034178212");
///
/// assert_eq!(regex_collection::id_card::is_taiwan_id_card(&id_card), true);
/// ```
pub fn is_taiwan_id_card(card_number: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z][0-9]{9}$").unwrap();
    re.is_match(card_number)
}
