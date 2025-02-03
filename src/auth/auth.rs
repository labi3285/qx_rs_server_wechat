#[allow(unused)]


use std::collections::HashMap;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use qx_rs_server::err::{Error, Result};
use qx_rs_server::env::{self, DEFAULT};
use qx_rs_server::time;

use qx_rs_server::req::req;

use super::def::*;

pub async fn get_session_by_js_code(js_code: &String) -> Result<Session> {
    _get_session_by_js_code(DEFAULT, js_code).await
}

pub async fn get_phone_by_code(phone_req_code: &String) -> Result<PhoneInfo> {
    _get_phone_by_code(DEFAULT, phone_req_code).await
}


async fn _get_phone_by_code(
    which_miniprogram: &'static str,
    phone_req_code: &String,
) -> Result<PhoneInfo> {
    let access_token = _get_access_token(which_miniprogram).await?;
    let url = format!("https://api.weixin.qq.com/wxa/business/getuserphonenumber?access_token={}", access_token.access_token);
    let data = serde_json::json!({
        "code": phone_req_code,  
    });
    let res = req::post_application_json_for_object::<_, RawPhoneInfo>(&url, &None, &data).await?;
    if res.errcode == 0 {
        Ok(res.phone_info)
    } else {
        Err(Error::ThirdPart(format!("_get_phone_by_code failed:({:?}){:?}", res.errcode, res.errmsg)))
    }
}


async fn _get_session_by_js_code(
    which_miniprogram: &'static str,
    js_code: &String,
) -> Result<Session> {
    let mut which = "WECHAT_MINI_PROGRAM".to_string();
    if which_miniprogram != DEFAULT {
        which = format!("WECHAT_MINI_PROGRAM.{}", which_miniprogram);
    }
    let app_id = env::str(&format!("{}.APP_ID", which))?;
    let app_secret = env::str(&format!("{}.APP_SECRET", which))?;
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?grant_type=authorization_code&appid={}&secret={}&js_code={}", app_id, app_secret, js_code);
    let res = req::get_for_object::<RawSession>(&url, &None).await?;
    if let Some(errcode) = res.errcode {
        Err(Error::ThirdPart(format!("_get_session_by_js_code failed:({:?}){:?}", errcode, res.errmsg)))
    } else {
        Ok(Session { session_key: res.session_key.unwrap(), unionid: res.unionid, openid: res.openid.unwrap() })
    }
}

static ACCESS_TOKEN: Lazy<Mutex<HashMap<&'static str, AccessToken>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

async fn _get_access_token(which_miniprogram: &'static str) -> Result<AccessToken> {
    let mut map = ACCESS_TOKEN.lock().await;
    let res = map.get(which_miniprogram);
    if let Some(access_token) = res {
        if time::now().timestamp() - access_token.create_time.timestamp() < access_token.expires_in_secs - 60 {
            tracing::info!("use ACCESS_TOKEN");
            return Ok(access_token.clone());
        }
    }
    let mut which = "WECHAT_MINI_PROGRAM".to_string();
    if which_miniprogram != DEFAULT {
        which = format!("WECHAT_MINI_PROGRAM.{}", which_miniprogram);
    }
    let app_id = env::str(&format!("{}.APP_ID", which))?;
    let app_secret = env::str(&format!("{}.APP_SECRET", which))?;
    let url = format!(
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
        app_id, app_secret
    );
    let res = req::get_for_object::<RawAccessToken>(&url, &None).await?;
    let access_token = AccessToken {
        access_token: res.access_token,
        expires_in_secs: res.expires_in as i64,
        create_time: time::now(),
    };
    map.insert(which_miniprogram, access_token.clone());
    Ok(access_token)
}
