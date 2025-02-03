use wechat_pay_rust_sdk::pay::WechatPay;

use qx_rs_server::env::DEFAULT;
use qx_rs_server::err::{Error, Result};
use qx_rs_server::env;



pub async fn get_client_private_key(which_pay: &'static str) -> Result<String> {
    let mut which = "WECHAT_PAY".to_string();
    if which_pay != DEFAULT {
        which = format!("WECHAT_PAY.{}", which_pay);
    }
    let private_key_path = env::str(&format!("{}.API_CLIENT_PRIVATE_KEY_PATH", which))?;
    let mut private_key_url = format!(".env.res{}", private_key_path);
    if cfg!(debug_assertions) {
        private_key_url = format!(".env.res.dev{}", private_key_path);
    }
    let private_key = std::fs::read_to_string(private_key_url).map_err(|err| {
        let err = format!("wechat get_client_private_key read_to_string failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    return Ok(private_key);
}

pub async fn get_pay(which_pay: &'static str) -> Result<WechatPay> {
    let mut which = "WECHAT_PAY".to_string();
    if which_pay != DEFAULT {
        which = format!("WECHAT_PAY.{}", which_pay);
    }
    let app_id = env::str(&format!("{}.APP_ID", which))?;
    let mch_id = env::str(&format!("{}.MCH_ID", which))?;
    let private_key_path = env::str(&format!("{}.PRIVATE_KEY_PATH", which))?;
    let mut private_key_url = format!(".env.res{}", private_key_path);
    if cfg!(debug_assertions) {
        private_key_url = format!(".env.res.dev{}", private_key_path);
    }
    let serial_no = env::str(&format!("{}.SERIAL_NO", which))?;
    let v3_key = env::str(&format!("{}.V3_KEY", which))?;
    let notify_path = env::str(&format!("{}.NOTIFY_PATH", which))?;
    let public_url: String = env::str("APP.PUBLIC_URL")?;
    let notify_url = format!("{}{}", public_url, notify_path);
    let private_key = std::fs::read_to_string(private_key_url).map_err(|err| {
        let err = format!("wechat get_pay read_to_string failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    tracing::info!("notify_url:{}", notify_url);
    let pay = WechatPay::new(app_id, mch_id, private_key, serial_no, v3_key, notify_url);
    Ok(pay)
}
