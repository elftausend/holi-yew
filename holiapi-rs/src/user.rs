use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub htl_access_token: String,
    pub username: String,
    pub usid: String,
    pub htl_class: String,
    pub htl_division: String,
    pub favs: Vec<u32>,
    pub uploaded: Vec<u32>,
}

impl UserInfo {
    pub fn new(
        username: String,
        usid: String,
        htl_class: String,
        htl_division: String,
    ) -> UserInfo {
        UserInfo {
            htl_access_token: String::new(),
            username,
            usid,
            htl_class,
            htl_division,
            favs: vec![],
            uploaded: vec![],
        }
    }
}

impl From<Value> for UserInfo {
    fn from(value: Value) -> Self {
        get_user_from_raw_json(value)
    }
}

pub fn get_user_from_raw_json(user_info_raw: Value) -> UserInfo {
    let username = user_info_raw["0"]["displayname"]["0"].as_str().unwrap();

    let mut htl_related_ids = user_info_raw["0"]["dn"].as_str().unwrap().split(",");

    let usid = &htl_related_ids.next().unwrap()[3..];
    let htl_class = &htl_related_ids.next().unwrap()[3..];
    let mut htl_division = &htl_related_ids.next().unwrap()[3..];

    if htl_division == "WI" {
        htl_division = &htl_class[3..]
    }

    UserInfo::new(
        username.to_string(),
        usid.to_string(),
        htl_class.to_string(),
        htl_division.to_string(),
    )
}


#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::UserInfo;

    #[test]
    fn test_from_raw_userinfo() -> Result<(), Box<dyn std::error::Error>> {
        let raw_user_info = json!(
            {"count": 1, "0": {"mail": {"count": 2, "0": "email1", "1": "email2"}, "0": "mail", "displayname": {"count": 1, "0": "A Name"}, "1": "displayname", "count": 2, "dn": "cn=111111,ou=1AFET,ou=ET,o=HTBL"}}
        );

        let user_info = UserInfo::from(raw_user_info);
        assert_eq!("A Name", user_info.username);
        assert_eq!("111111", user_info.usid);
        assert_eq!("1AFET", user_info.htl_class);
        assert_eq!("ET", user_info.htl_division);

        Ok(())
    }

    #[test]
    fn test_from_raw_userinfo_wi() -> Result<(), Box<dyn std::error::Error>> {
        let raw_user_info = json!(
            {"count": 1, "0": {"mail": {"count": 2, "0": "email1", "1": "email2"}, "0": "mail", "displayname": {"count": 1, "0": "A Name"}, "1": "displayname", "count": 2, "dn": "cn=111111,ou=1AHWII,ou=WI,o=HTBL"}}
        );

        let user_info = UserInfo::from(raw_user_info);
        assert_eq!("A Name", user_info.username);
        assert_eq!("111111", user_info.usid);
        assert_eq!("1AHWII", user_info.htl_class);
        assert_eq!("WII", user_info.htl_division);

        let raw_user_info = json!(
            {"count": 1, "0": {"mail": {"count": 2, "0": "email1", "1": "email2"}, "0": "mail", "displayname": {"count": 1, "0": "A Name"}, "1": "displayname", "count": 2, "dn": "cn=111111,ou=1AHWIL,ou=WI,o=HTBL"}}
        );

        let user_info = UserInfo::from(raw_user_info);
        assert_eq!("WIL", user_info.htl_division);
        Ok(())
    }
}