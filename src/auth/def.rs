use chrono::{DateTime, Utc};


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct RawAccessToken {
    pub access_token: String,
    pub expires_in: u32,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct AccessToken {
    pub access_token: String,
    pub expires_in_secs: i64,
    pub create_time: DateTime<Utc>,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct RawPhoneInfo {
    pub errmsg: String,
    pub errcode: i32,
    pub phone_info: PhoneInfo,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneInfo {
    #[serde(rename = "purePhoneNumber")]
    pub phone: String,
    #[serde(rename = "phoneNumber")]
    pub full_phone: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
}


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawSession {
    pub session_key: Option<String>,
    pub unionid: Option<String>,
    pub openid: Option<String>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>,
    pub rid: Option<String>,
}


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub session_key: String,
    pub unionid: Option<String>,
    pub openid: String,
}

