use serde_json::json;
use qx_rs_server::def::resp_json::ApplicationJson;
use wechat_pay_rust_sdk::pay::PayNotifyTrait;
use wechat_pay_rust_sdk::model::{WechatPayDecodeData, WechatPayNotify};

use qx_rs_server::env::DEFAULT;
use qx_rs_server::err::{Error, Result};

use crate::pay::def::NotifyData;

use super::pay;


pub async fn parse_pay_notify(notify: WechatPayNotify) -> Result<NotifyData> {
    tracing::info!("WechatPayNotify {:?}", notify);    
    let nonce = &notify.resource.nonce;
    let ciphertext = &notify.resource.ciphertext;
    let associated_data = &notify.resource.associated_data.unwrap_or_default();
    let raw_data: WechatPayDecodeData = decript_data(ciphertext, nonce, associated_data).await?;
    tracing::info!("WechatPayDecodeData {:?}", &raw_data);
    let notify_data = NotifyData { order_id: raw_data.out_trade_no.clone(), client_open_id: raw_data.payer.openid.clone(), raw_data };
    Ok(notify_data)
}

pub async fn decript_data(ciphertext: &String, nonce: &String, associated_data: &String) -> Result<WechatPayDecodeData> {
    let pay = pay::get_pay(DEFAULT).await?;
    let decode_data = pay.decrypt_paydata(ciphertext, nonce, associated_data).map_err(|err| {
        let err = format!("wechat decript_data failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    Ok(decode_data)
}


pub fn parse_resp<T>(res: Result<T>) -> ApplicationJson<String> {
    match res {
        Ok(_) => {
            let res_json = json!({
                "code": "SUCCESS",
                "message": "成功"
            });
            let res_json = serde_json::to_string(&res_json).unwrap();
            ApplicationJson::new(res_json)
        },
        Err(err) => {
            tracing::error!("{}", err);
            parse_resp_fail()
        }
    }
}

pub fn parse_resp_fail() -> ApplicationJson<String> {
    let res_json = json!({
        "code": "FAIL",
        "message": "失败"
    });
    let res_json = serde_json::to_string(&res_json).unwrap();
    ApplicationJson::new(res_json)
}

