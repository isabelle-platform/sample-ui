use crate::util::query::*;
use isabelle_dm::data_model::item::Item;
use isabelle_dm::data_model::list_result::ListResult;
use isabelle_dm::data_model::process_result::ProcessResult;
use isabelle_dm::transfer_model::detailed_login_user::DetailedLoginUser;
use std::collections::HashMap;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::window;
use web_sys::FormData;
use web_sys::{Request, RequestInit, RequestMode, Response};

const BASE_URL: &str = "http://localhost:8090";

const LOGIN_URL: &str = "/login";
const LOGOUT_URL: &str = "/logout";
const IS_LOGGED_IN_URL: &str = "/is_logged_in";
const GEN_OTP_URL: &str = "/gen_otp";

const ITM_EDIT_URL: &str = "/itm/edit";
const ITM_DEL_URL: &str = "/itm/del";
const ITM_LIST_URL: &str = "/itm/list";

const SETTING_EDIT_URL: &str = "/setting/edit";
const SETTING_LIST_URL: &str = "/setting/list";
const SETTING_GCAL_AUTH_URL: &str = "/setting/gcal_auth";

pub fn get_base_url() -> String {
    let hostname = window().unwrap().location().hostname();
    match hostname {
        Ok(val) => {
            if val.to_string() == "localhost" {
                return "http://".to_owned() + &val.to_string() + &":8090".to_string();
            } else {
                return "https://".to_owned() + &val.to_string() + &"/api".to_string();
            }
        }
        Err(_err) => {}
    }

    return BASE_URL.to_string();
}

pub enum ServerUrl {
    Login,
    Logout,
    IsLoggedIn,
    GenOtp,

    ItmList,
    #[allow(dead_code)]
    ItmEdit,
    #[allow(dead_code)]
    ItmDel,

    SettingEdit,
    SettingList,
    SettingGcalAuth,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: String,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self {
            err: value.as_string().unwrap(),
        }
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            err: value.to_string(),
        }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub fn get_server_url(url: ServerUrl) -> String {
    match url {
        ServerUrl::Login => String::from(get_base_url() + LOGIN_URL),
        ServerUrl::Logout => String::from(get_base_url() + LOGOUT_URL),
        ServerUrl::IsLoggedIn => String::from(get_base_url() + IS_LOGGED_IN_URL),
        ServerUrl::GenOtp => String::from(get_base_url() + GEN_OTP_URL),

        ServerUrl::ItmList => String::from(get_base_url() + ITM_LIST_URL),
        ServerUrl::ItmEdit => String::from(get_base_url() + ITM_EDIT_URL),
        ServerUrl::ItmDel => String::from(get_base_url() + ITM_DEL_URL),

        ServerUrl::SettingEdit => String::from(get_base_url() + SETTING_EDIT_URL),
        ServerUrl::SettingList => String::from(get_base_url() + SETTING_LIST_URL),
        ServerUrl::SettingGcalAuth => String::from(get_base_url() + SETTING_GCAL_AUTH_URL),
    }
}

pub async fn fetch_itm(
    collection: &str,
    context: &str,
    id: u64,
    id_min: u64,
    id_max: u64,
    sort_key: &str,
    filter: &str,
    skip: u64,
    limit: u64,
    id_list: Vec<u64>,
) -> Result<ListResult, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let mut url = get_server_url(ServerUrl::ItmList)
        + "?collection="
        + &collection.to_string()
        + "&context="
        + &context.to_string()
        + "&id="
        + &id.to_string()
        + "&id_min="
        + &id_min.to_string()
        + "&id_max="
        + &id_max.to_string()
        + "&sort_key="
        + &sort_key.to_string()
        + "&filter="
        + &filter.to_string()
        + "&skip="
        + &skip.to_string()
        + "&limit="
        + &limit.to_string();
    for tmp in id_list {
        url += &("&id_list[]=".to_owned() + &tmp.to_string());
    }

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let resp_text = resp.text();
    if !resp_text.is_ok() {
        return Err(FetchError {
            err: "Unauthorized".to_string(),
        });
    }

    let text = JsFuture::from(resp_text.unwrap()).await?;
    let s = text.as_string();
    if s.is_none() || s.as_ref().unwrap() == "" {
        return Err(FetchError {
            err: "Unauthorized".to_string(),
        });
    }
    let val: ListResult = serde_json::from_str(&s.as_ref().unwrap())?;

    Ok(val)
}

pub async fn fetch_itm_list(
    collection: &str,
    context: &str,
    id: u64,
    id_min: u64,
    id_max: u64,
    sort_key: &str,
    filter: &str,
    skip: u64,
    limit: u64,
    id_list: Vec<u64>,
) -> Result<HashMap<u64, Item>, FetchError> {
    let itms = fetch_itm(
        collection, context, id, id_min, id_max, sort_key, filter, skip, limit, id_list,
    )
    .await;
    match itms {
        Ok(r) => {
            return Ok(r.map);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub async fn post_directly(url_part: &str, query: &str) -> Result<bool, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let fixed_params = fix_regular_params(query.to_string());
    let url = String::from(get_base_url() + url_part) + &fixed_params;

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let _text = JsFuture::from(resp.text()?).await?;
    Ok(true)
}

pub async fn post_itm_edit(
    collection: &str,
    id: u64,
    merge: bool,
    itm: Item,
) -> Result<ProcessResult, FetchError> {
    let fd = FormData::new().unwrap();
    let s = serde_json::to_string(&itm)?;

    fd.append_with_str("item", &s).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let js_value: JsValue = fd.clone().into();
    opts.body(Some(&js_value));

    let url = get_server_url(ServerUrl::ItmEdit)
        + "?collection="
        + collection
        + "&id="
        + &id.to_string()
        + "&merge="
        + (if merge { "true" } else { "false" });

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let js = JsFuture::from(resp.text()?).await?;
    let s = js.as_string().unwrap();
    let val: ProcessResult = serde_json::from_str(&s)?;
    Ok(val)
}

pub async fn post_start_gcal_auth() -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let url = get_server_url(ServerUrl::SettingGcalAuth);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text()?).await?;
    let s = text.as_string().unwrap();
    Ok(s)
}

pub async fn post_login(username: String, password: String) -> Result<ProcessResult, FetchError> {
    let fd = FormData::new().unwrap();
    fd.append_with_str("username", &username).unwrap();
    fd.append_with_str("password", &password).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let js_value: JsValue = fd.clone().into();
    opts.body(Some(&js_value));

    let url = get_server_url(ServerUrl::Login);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    let s = text.as_string().unwrap();
    let val: ProcessResult = serde_json::from_str(&s)?;
    Ok(val)
}

pub async fn post_gen_otp(username: String) -> Result<ProcessResult, FetchError> {
    let fd = FormData::new().unwrap();
    fd.append_with_str("username", &username).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let js_value: JsValue = fd.clone().into();
    opts.body(Some(&js_value));

    let url = get_server_url(ServerUrl::GenOtp);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    let s = text.as_string().unwrap();
    let val: ProcessResult = serde_json::from_str(&s)?;
    Ok(val)
}

pub async fn post_logout() -> Result<bool, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let url = get_server_url(ServerUrl::Logout);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let _text = JsFuture::from(resp.text()?).await?;
    Ok(true)
}

pub async fn fetch_is_logged_in() -> Result<DetailedLoginUser, FetchError> {
    console::log_2(&"test ".into(), &get_base_url().to_string().into());

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let url = get_server_url(ServerUrl::IsLoggedIn);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text()?).await?;
    let val: DetailedLoginUser = serde_json::from_str(&text.as_string().unwrap())?;
    Ok(val)
}

pub async fn post_setting_edit(itm: Item) -> Result<ProcessResult, FetchError> {
    let fd = FormData::new().unwrap();
    let s = serde_json::to_string(&itm)?;

    fd.append_with_str("item", &s).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let js_value: JsValue = fd.clone().into();
    opts.body(Some(&js_value));

    let url = get_server_url(ServerUrl::SettingEdit);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let js = JsFuture::from(resp.text()?).await?;
    let s = js.as_string().unwrap();
    let val: ProcessResult = serde_json::from_str(&s)?;
    Ok(val)
}

pub async fn fetch_setting_list() -> Result<Item, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(web_sys::RequestCredentials::Include);

    let url = get_server_url(ServerUrl::SettingList);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    let s = text.as_string();
    if s.is_none() || s.as_ref().unwrap() == "" {
        return Err(FetchError {
            err: "Unauthorized".to_string(),
        });
    }
    let val: Item = serde_json::from_str(&s.unwrap())?;
    Ok(val)
}
