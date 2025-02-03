use wechat_pay_rust_sdk::model::{AppParams, H5Params, H5SceneInfo, JsapiParams, MicroParams, NativeParams};
use wechat_pay_rust_sdk::pay::{WechatPay, WechatPayTrait};

use qx_rs_server::env::DEFAULT;
use qx_rs_server::err::{Error, Result};
use qx_rs_server::env;
use qx_rs_server::util;
use qx_rs_server::time;

use super::def::*;
use super::pay;


pub async fn pay_app_prepare_id(
    order_id: &String,
    amount: f64,
    desc: &str
) -> Result<PrepayId> {
    let (_, prepay_id) = _pay_app_prepare_id(order_id, amount, desc).await?;
    Ok(PrepayId { prepay_id })
}

pub async fn pay_jsapi_prepare_id(
    order_id: &String,
    amount: f64,
    desc: &str,
    client_open_id: &String,
) -> Result<PrepayId> {
    let (_, prepay_id) = _pay_jsapi_prepare_id(order_id, amount, desc, client_open_id).await?;
    Ok(PrepayId { prepay_id })
}

pub async fn pay_mini_program_payment(
    order_id: &String,
    amount: f64,
    desc: &str,
    client_open_id: &String,
) -> Result<PaymentPackage> {
    let (pay, prepay_id) = _pay_mini_program_prepare_id(order_id, amount, desc, client_open_id).await?;
    let payment = _package_prepare_id(pay, &order_id, &prepay_id).await?;
    Ok(payment)
}

pub async fn pay_native_prepare_id(order_id: &String, amount: f64, desc: &str) -> Result<CodeUrl> {
    let (_, code_url) = _pay_native_prepare_id(order_id, amount, desc).await?;
    Ok(CodeUrl { code_url })
}

pub async fn pay_h5_url(
    order_id: &String,
    amount: f64,
    desc: &str,
    client_ip: &String,
    client_name: &String,
    client_url: &String,
) -> Result<H5Url> {
    let (_, h5_url) =
        _pay_h5_url_with_pay(order_id, amount, desc, client_ip, client_name, client_url).await?;
    Ok(H5Url { h5_url })
}

pub async fn pay_wechat_pay_link(
    order_id: &String,
    amount: f64,
    desc: &str,
    client_ip: &String,
    client_name: &String,
    client_url: &String,
) -> Result<WechatLink> {
    let public_url: String = env::str("APP.PUBLIC_URL")?;
    let (pay, h5_url) =
        _pay_h5_url_with_pay(order_id, amount, desc, client_ip, client_name, client_url).await?;
    let wechat_link = pay.get_weixin(h5_url, public_url).await.map_err(|err| {
        let err = format!("wechat pay_wechat_pay_link get_weixin failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    if let Some(wechat_link) = wechat_link {
        Ok(WechatLink { wechat_link })
    } else {
        let err = format!("wechat pay_wechat_pay_link none");
        tracing::error!(err);
        Err(Error::ThirdPart(err))
    }
}


pub async fn _package_prepare_id(
    wechat_pay: WechatPay,
    order_id: &String,
    prepay_id: &String,
) -> Result<PaymentPackage> {
    let nonce_str = util::rand::rand_str(32);
    let package = format!("prepay_id={}", prepay_id);
    let timestamp = time::now().timestamp();
    let sign_type = "RSA".to_string();
    let content = format!("{}\n{}\n{}\n{}\n", wechat_pay.appid(), timestamp, nonce_str, package);
    let pay_sign = wechat_pay.rsa_sign(content);
    Ok(PaymentPackage {
        order_id: order_id.clone(),
        timestamp: timestamp.to_string(),
        nonce_str,
        package,
        sign_type,
        pay_sign,
    })
}

pub async fn _pay_app_prepare_id(
    order_id: &String,
    amount: f64,
    desc: &str,
) -> Result<(WechatPay, String)> {
    let amount = (amount * 100.0) as i32;
    let pay = pay::get_pay(DEFAULT).await?;
    let params = AppParams::new(desc, &order_id, amount.into());
    let body = pay.app_pay(params).await.map_err(|err| {
        let err = format!("wechat _pay_app_prepare_id app_pay failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    if let Some(prepay_id) = body.prepay_id {
        Ok((pay, prepay_id))
    } else {
        let err = format!("wechat _pay_app_prepare_id failed:[{:?}]{:?}", body.code, body.message);
        tracing::error!(err);
        Err(Error::ThirdPart(err))
    }
}

pub async fn _pay_jsapi_prepare_id(
    order_id: &String,
    amount: f64,
    desc: &str,
    client_open_id: &String,
) -> Result<(WechatPay, String)> {
    let amount = (amount * 100.0) as i32;
    let pay = pay::get_pay(DEFAULT).await?;
    let params = JsapiParams::new(
        desc,
        &order_id,
        amount.into(),
        client_open_id.as_str().into(),
    );
    let body = pay.jsapi_pay(params).await.map_err(|err| {
        let err = format!("wechat _pay_jsapi_prepare_id jsapi_pay failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    if let Some(prepay_id) = body.prepay_id {
        Ok((pay, prepay_id))
    } else {
        let err = format!("wechat _pay_jsapi_prepare_id failed:[{:?}]{:?}", body.code, body.message);
        tracing::error!(err);
        Err(Error::ThirdPart(err))
    }
}

pub async fn _pay_mini_program_prepare_id(
    order_id: &String,
    amount: f64,
    desc: &str,
    client_open_id: &String,
) -> Result<(WechatPay, String)> {
    let amount = (amount * 100.0) as i32;
    let pay = pay::get_pay(DEFAULT).await?;
    let params = MicroParams::new(
        desc,
        &order_id,
        amount.into(),
        client_open_id.as_str().into(),
    );
    let body = pay.micro_pay(params).await.map_err(|err| {
        let err = format!("wechat _pay_mini_program_prepare_id micro_pay failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    if let Some(prepay_id) = body.prepay_id {
        Ok((pay, prepay_id))
    } else {
        let err = format!("wechat _pay_mini_program_prepare_id failed:[{:?}]{:?}", body.code, body.message);
        tracing::error!(err);
        Err(Error::ThirdPart(err))
    }
}

pub async fn _pay_native_prepare_id(
    order_id: &String,
    amount: f64,
    desc: &str,
) -> Result<(WechatPay, String)> {
    let amount = (amount * 100.0) as i32;
    let pay = pay::get_pay(DEFAULT).await?;
    let params = NativeParams::new(&desc, &order_id.as_str(), amount.into());
    let body = pay.native_pay(params).await.map_err(|err| {
        let err = format!("wechat _pay_native_prepare_id native_pay failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    if let Some(code_url) = body.code_url {
        Ok((pay, code_url))
    } else {
        let err = format!("wechat _pay_native_prepare_id failed:[{:?}]{:?}", body.code, body.message);
        tracing::error!(err);
        Err(Error::ThirdPart(err))
    }
}

pub async fn _pay_h5_url_with_pay(
    order_id: &String,
    amount: f64,
    desc: &str,
    client_ip: &String,
    client_name: &String,
    client_url: &String,
) -> Result<(WechatPay, String)> {
    let amount = (amount * 100.0) as i32;
    let pay = pay::get_pay(DEFAULT).await?;
    let params = H5Params::new(
        desc,
        order_id,
        amount.into(),
        H5SceneInfo::new(client_ip, client_name, client_url),
    );
    let body = pay.h5_pay(params).await.map_err(|err| {
        let err = format!("wechat _pay_h5_url_with_pay h5_pay failed:{:?}", err);
        tracing::error!(err);
        Error::ThirdPart(err)
    })?;
    if let Some(h5_url) = body.h5_url {
        Ok((pay, h5_url))
    } else {
        let err = format!("wechat _pay_h5_url_with_pay failed:[{:?}]{:?}", body.code, body.message);
        tracing::error!(err);
        Err(Error::ThirdPart(err))
    }
}
