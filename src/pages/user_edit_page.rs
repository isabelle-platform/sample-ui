use serde::Deserialize;

use crate::common::*;
use isabelle_dm::data_model::item::Item;

use crate::components::baloon::BaloonView;

use crate::util::accessor::*;

use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::util::dt_conv::*;
use crate::util::input::*;
use isabelle_dm::transfer_model::detailed_login_user::DetailedLoginUser;

pub enum Msg {
    UpdateCurrentUserData(FetchState<DetailedLoginUser>),
    UpdateCurrentUser(FetchState<HashMap<u64, Item>>),
    UpdateItemList(FetchState<HashMap<u64, Item>>),
    UpdateStr(String, String),
    UpdateBool(String, bool),
    UpdateU64(String, u64),
    SaveData,
    SaveDataSucceeded,
    SaveDataFailed,
}

pub struct UserEditPage {
    login_user_id: FetchState<DetailedLoginUser>,
    login_user: FetchState<HashMap<u64, Item>>,
    queried_id: u64,
    item: Item,
    orig_item: Item,
    items: FetchState<HashMap<u64, Item>>,
    in_progress: bool,
    failed: bool,
}

impl Component for UserEditPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();

        #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
        pub struct IdParams {
            #[serde(default = "unset_max")]
            pub id: u64,
        }

        let q = &location.query::<IdParams>().unwrap();

        Self {
            login_user_id: FetchState::Fetching,
            login_user: FetchState::Fetching,
            items: FetchState::Fetching,
            queried_id: q.id,
            item: Item::new(),
            orig_item: Item::new(),
            in_progress: false,
            failed: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateCurrentUserData(fetch_state) => {
                self.login_user_id = fetch_state;
            }
            Msg::UpdateCurrentUser(fetch_state) => {
                self.login_user = fetch_state;
            }
            Msg::UpdateItemList(fetch_state) => {
                self.items = fetch_state;
                match &self.items {
                    FetchState::Success(new_items) => {
                        if new_items.contains_key(&self.queried_id) {
                            self.item = new_items[&self.queried_id].clone();
                            self.orig_item = new_items[&self.queried_id].clone();
                        }
                    }
                    _ => {}
                }
            }
            Msg::SaveData => {
                let queried_id = self.queried_id;
                let itm = self.item.clone();
                if itm.safe_str_with_empty("email", "") != "" {
                    self.in_progress = true;
                    ctx.link().send_future(async move {
                        match post_itm_edit("user", queried_id, true, itm).await {
                            Ok(_res) => Msg::SaveDataSucceeded,
                            Err(_err) => Msg::SaveDataFailed,
                        }
                    });
                }
            }
            Msg::UpdateStr(name, val) => {
                self.item.set_str(&name, &val);
            }
            Msg::UpdateBool(name, val) => {
                self.item.set_bool(&name, val);
            }
            Msg::UpdateU64(name, val) => {
                self.item.set_u64(&name, val);
            }
            Msg::SaveDataSucceeded => {
                self.in_progress = false;
                self.failed = false;
                let new_url = "/user";
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(new_url)
                    .unwrap();
            }
            Msg::SaveDataFailed => {
                self.in_progress = false;
                self.failed = true;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let lu: DetailedLoginUser;
        let lus: &HashMap<u64, Item>;
        let items: &HashMap<u64, Item>;

        match &self.login_user_id {
            FetchState::Fetching => {
                ctx.link().send_future(async move {
                    match fetch_is_logged_in().await {
                        Ok(md) => Msg::UpdateCurrentUserData(FetchState::Success(md)),
                        Err(err) => Msg::UpdateCurrentUserData(FetchState::Failed(err)),
                    }
                });
                return html! {
                    <>
                        //<BaloonView message={ "Loading items..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(new_login_user_data) => {
                lu = new_login_user_data.clone();
            }
            FetchState::Failed(err) => {
                return html! {
                    <>
                        <BaloonView message={ err.to_string() } style="error"/>
                    </>
                }
            }
        };

        match &self.login_user {
            FetchState::Fetching => {
                let id = lu.id;
                ctx.link().send_future(async move {
                    match fetch_itm_list(
                        "user",
                        "full",
                        id,
                        u64::MAX,
                        u64::MAX,
                        "name",
                        "",
                        u64::MAX,
                        u64::MAX,
                        Vec::new(),
                    )
                    .await
                    {
                        Ok(md) => Msg::UpdateCurrentUser(FetchState::Success(md)),
                        Err(err) => Msg::UpdateCurrentUser(FetchState::Failed(err)),
                    }
                });
                return html! {
                    <>
                        //<BaloonView message={ "Loading items..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(new_login_users) => {
                lus = new_login_users;
            }
            FetchState::Failed(err) => {
                return html! {
                    <>
                        <BaloonView message={ err.to_string() } style="error"/>
                    </>
                }
            }
        };

        match &self.items {
            FetchState::Fetching => {
                let queried_id = self.queried_id;
                ctx.link().send_future(async move {
                    match fetch_itm_list(
                        "user",
                        "full",
                        queried_id,
                        u64::MAX,
                        u64::MAX,
                        "name",
                        "",
                        u64::MAX,
                        u64::MAX,
                        Vec::new(),
                    )
                    .await
                    {
                        Ok(md) => Msg::UpdateItemList(FetchState::Success(md)),
                        Err(err) => Msg::UpdateItemList(FetchState::Failed(err)),
                    }
                });
                return html! {
                    <>
                        //<BaloonView message={ "Loading items..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(_items) => {
                items = _items;
                return html! {
                    <>
                    <div class="section container">
                        <h1 class="title">{ "Edit user" }</h1>
                        { self.add_form(ctx, lu, lus, items) }
                    </div>
                    </>
                };
            }
            FetchState::Failed(err) => {
                return html! {
                    <>
                        <BaloonView message={ err.to_string() } style="error"/>
                    </>
                }
            }
        }
    }
}

impl UserEditPage {
    fn add_form(
        &self,
        ctx: &Context<Self>,
        lu: DetailedLoginUser,
        lus: &HashMap<u64, Item>,
        _items: &HashMap<u64, Item>,
    ) -> Html {
        let signup_date: String;
        let birth_date: String;
        {
            let mut datetime = self.item.safe_u64("signup_date", 0);
            if datetime == 0 {
                datetime = chrono::Local::now().timestamp() as u64;
            }

            let dt = ts2date(datetime);
            signup_date = dt.format("%Y-%m-%d").to_string();
        }
        {
            let mut datetime = self.item.safe_u64("birth_date", 0);
            if datetime == 0 {
                datetime = chrono::Local::now().timestamp() as u64;
            }

            let dt = ts2date(datetime);
            birth_date = dt.format("%Y-%m-%d").to_string();
        }

        /* FIXME: check current user, not the one that we edit */
        let is_admin = if lus.contains_key(&lu.id) {
            lus[&lu.id].safe_bool("role_is_admin", false)
        } else {
            false
        };

        let signup_date_htm = if is_admin {
            html! {
                <>
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "Signup date" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input type="date" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateU64("signup_date".to_string(), get_simple_date_ts(event)))} class="input" name="tmp_signup_date" value={ signup_date.to_string() }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-right-to-bracket"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };

        let roles = if is_admin {
            html! {
                <>
                    <h2 class="subtitle">{ "Roles" }</h2>

                    <div class="field is-horizontal">
                        <div class="field-label">
                            <label class="label">{ "" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control">
                                    <label class="checkbox">
                                      <input type="checkbox" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateBool("role_is_active".to_string(), get_checked(event)))} name="bools[role_is_active]" value="true" checked={ self.item.safe_bool("role_is_active", false) }/>
                                      { " Active?" }
                                    </label>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label">
                            <label class="label">{ "" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control">
                                    <label class="checkbox">
                                      <input type="checkbox" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateBool("role_is_admin".to_string(), get_checked(event)))} name="bools[role_is_admin]" value="true" checked={ self.item.safe_bool("role_is_admin", false) }/>
                                      { " Administrator?" }
                                    </label>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label">
                            <label class="label">{ "" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control">
                                    <label class="checkbox">
                                      <input type="checkbox" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateBool("role_is_contractor".to_string(), get_checked(event)))} name="bools[role_is_contractor]" value="true" checked={ self.item.safe_bool("role_is_contractor", false) }/>
                                      { " Contractor?" }
                                    </label>
                                </div>
                            </div>
                        </div>
                    </div>

                    <hr/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };

        let pwd_htm = if self.queried_id != u64::MAX {
            html! {
                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label"></label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control">
                                <p class="help"><a href={"/user/pwd?id=".to_owned() + &self.queried_id.to_string() }>{ "Change" }</a>{" the password" } </p>
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <>
                </>
            }
        };

        let err_htm = if self.failed {
            html! {
                <>
                    <BaloonView message={ "Failed to save user" } style="error"/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        html! {
            <>
                { err_htm }
                //<h2 class="subtitle">{ "Logging in" }</h2>

                <div class="field is-horizontal is-hidden">
                    <div class="field-label is-normal">
                        <label class="label">{ "ID" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control">
                                <input class="input is-static" type="text" name="id" readonly=true value={ self.queried_id.to_string() }/>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Login" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("login".to_string(), get_input(event)))} autocomplete="new-password" type="text" name="strs[login]" value={ self.item.safe_str("login", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-user"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                { pwd_htm }

                <hr/>

                { roles }

                <h2 class="subtitle">{ "Personal information" }</h2>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Name" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("name".to_string(), get_input(event)))} placeholder="Complete name" required={ true } type="text" name="strs[name]" value={ self.item.safe_str("name", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-signature"></i>
                                </span>

                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label">
                        <label class="label is-normal">{ "Phone" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("phone".to_string(), get_input(event)))} placeholder="+351..." type="tel" name="strs[phone]" value={ self.item.safe_str("phone", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-phone"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "E-Mail*" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" style={ if self.item.safe_str("email", "") == "" { "background: pink;" } else { "" }} oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("email".to_string(), get_input(event)))} placeholder="student@example.com" type="email" name="strs[email]" value={ self.item.safe_str("email", "").clone() } required={true}/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-envelope"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>


                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Birth date" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input type="date" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateU64("birth_date".to_string(), get_simple_date_ts(event)))} class="input" name="tmp_birth_date" value={ birth_date.to_string() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-cake-candles"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                { signup_date_htm }

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Notes" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control">
                                <textarea class="textarea" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("notes".to_string(), get_text_content(event)))} name="strs[notes]" rows="10" value={ self.item.safe_str("notes", "").clone() }/>
                            </div>
                        </div>
                    </div>
                </div>

                <hr/>

                <div class="field is-horizontal">
                    <div class="field-label">
                        <label class="label">{ "" }</label>
                    </div>
                    <div class="field-body">
                        <button onclick={ctx.link().callback(|_| Msg::SaveData)} type="submit" class={"button is-link ".to_owned() + if self.in_progress { "is-loading"} else { "" }}>
                            <span class="icon is-small">
                                <i class="fas fa-floppy-disk"></i>
                            </span>
                            <span>
                                { "Save" }
                            </span>
                        </button>
                    </div>
                </div>
            </>
        }
    }
}
