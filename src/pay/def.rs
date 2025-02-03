use wechat_pay_rust_sdk::model::WechatPayDecodeData;


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeUrl {
    pub code_url: String,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct H5Url {
    pub h5_url: String,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WechatLink {
    pub wechat_link: String,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrepayId {
    pub prepay_id: String,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotifyData {
    pub order_id: String,
    pub client_open_id: String,
    pub raw_data: WechatPayDecodeData,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentPackage {
    pub timestamp: String,
    pub nonce_str: String,
    pub package: String,
    pub sign_type: String,
    pub pay_sign: String,
    pub order_id: String,
}