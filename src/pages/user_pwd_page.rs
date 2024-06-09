use crate::common::*;
use crate::components::baloon::BaloonView;
use isabelle_dm::data_model::id_param::IdParam;
use isabelle_dm::data_model::item::Item;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    UpdateItemList(FetchState<HashMap<u64, Item>>),
    UpdateCurrentPassword(String),
    UpdateNewPassword1(String),
    UpdateNewPassword2(String),
    SubmitPasswordRequest,
    SubmitPasswordRequestSucceeded,
    SubmitPasswordRequestFailed,
    Otp,
    OtpFinished,
}

pub struct UserPwdPage {
    queried_id: u64,
    items: FetchState<HashMap<u64, Item>>,
    current_pwd: String,
    new_pwd1: String,
    new_pwd2: String,
    failed: bool,
    otp_in_progress: bool,
    otp_done: bool,
}

impl Component for UserPwdPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();

        let q = &location.query::<IdParam>().unwrap();

        Self {
            queried_id: q.id,
            items: FetchState::Fetching,
            current_pwd: "".to_string(),
            new_pwd1: "".to_string(),
            new_pwd2: "".to_string(),
            failed: false,
            otp_in_progress: false,
            otp_done: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateItemList(fetch_state) => {
                self.items = fetch_state;
            }
            Msg::UpdateCurrentPassword(s) => {
                self.current_pwd = s;
            }
            Msg::UpdateNewPassword1(s) => {
                self.new_pwd1 = s;
            }
            Msg::UpdateNewPassword2(s) => {
                self.new_pwd2 = s;
            }
            Msg::SubmitPasswordRequest => {
                self.otp_done = false;
                if self.new_pwd1 == "" || self.new_pwd2 == "" || self.new_pwd1 != self.new_pwd2 {
                    self.failed = true;
                } else {
                    self.failed = false;
                    let mut itm = Item::new();
                    itm.id = self.queried_id;
                    itm.set_str("__password", &self.current_pwd);
                    itm.set_str("__new_password1", &self.new_pwd1);
                    itm.set_str("__new_password2", &self.new_pwd2);
                    let queried_id = self.queried_id;
                    ctx.link().send_future(async move {
                        match post_itm_edit("user", queried_id, true, itm).await {
                            Ok(res) => {
                                if res.succeeded {
                                    Msg::SubmitPasswordRequestSucceeded
                                } else {
                                    Msg::SubmitPasswordRequestFailed
                                }
                            }
                            Err(_err) => Msg::SubmitPasswordRequestFailed,
                        }
                    });
                }
            }
            Msg::SubmitPasswordRequestSucceeded => {
                self.failed = false;
                web_sys::window().unwrap().history().unwrap().back().ok();
            }
            Msg::SubmitPasswordRequestFailed => {
                self.failed = true;
            }
            Msg::Otp => {
                self.otp_done = false;
                let queried_id = self.queried_id;
                let username = match &self.items {
                    FetchState::Success(_items) => {
                        if _items.contains_key(&queried_id) {
                            _items[&queried_id].safe_str("email", "")
                        } else {
                            "".to_string()
                        }
                    }
                    _ => "".to_string(),
                };
                if username != "" {
                    self.otp_in_progress = true;
                    ctx.link().send_future(async move {
                        match post_gen_otp(username).await {
                            Ok(_res) => Msg::OtpFinished,
                            Err(_err) => Msg::OtpFinished,
                        }
                    });
                }
            }
            Msg::OtpFinished => {
                self.otp_in_progress = false;
                self.otp_done = true;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items: &HashMap<u64, Item>;

        match &self.items {
            FetchState::Fetching => {
                let queried_id = self.queried_id;
                ctx.link().send_future(async move {
                    match fetch_itm_list(
                        "user",
                        "full",
                        u64::MAX,
                        queried_id,
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
                        <BaloonView message={ "Loading items..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(_items) => {
                items = _items;
                return html! {
                    <>
                    <div class="section container">
                        <h1 class="title">{ "Change password" }</h1>
                        { self.add_form(ctx, items) }
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

impl UserPwdPage {
    fn add_form(&self, ctx: &Context<Self>, items: &HashMap<u64, Item>) -> Html {
        if !items.contains_key(&self.queried_id) {
            return html! {
                <>
                </>
            };
        }

        let non_empty_html = if self.new_pwd1 == "" || self.new_pwd2 == "" {
            html! {
                <>
                    <BaloonView message={ "Passwords must not be empty" } style="info"/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        let dont_match_html = if self.new_pwd1 != self.new_pwd2 {
            html! {
                <>
                    <BaloonView message={ "Passwords don't match" } style="info"/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        let failed_html = if self.failed {
            html! {
                <>
                    <BaloonView message={ "Failed to change password" } style="error"/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        let otp_in_progress_htm = match self.otp_in_progress {
            true => {
                html! {
                    <BaloonView message={ "One-time password is being sent..." } style="info"/>
                }
            }
            false => {
                html! {
                    <>
                    </>
                }
            }
        };
        let otp_done_htm = match self.otp_done {
            true => {
                html! {
                    <BaloonView message={ "One-time password is sent to your email if there is such a user. Copy the received code to a password field." } style="success"/>
                }
            }
            false => {
                html! {
                    <>
                    </>
                }
            }
        };
        html! {
            <>
                <h2 class="subtitle">{ "Change password" }</h2>

                { otp_done_htm }
                { otp_in_progress_htm }

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Password" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateCurrentPassword(event.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().clone().value().into()))} autocomplete="new-password" type="password" name="old_password" />
                                <span class="icon is-small is-left">
                                    <i class="fas fa-unlock"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label"></label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control">
                                <button  onclick={ctx.link().callback(|_| Msg::Otp)} type="submit" class="button">
                                    <span class="icon is-small">
                                        <i class="fas fa-envelope"></i>
                                    </span>
                                    <span>
                                        { "Send one-time password" }
                                    </span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "New password" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateNewPassword1(event.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().clone().value().into()))} autocomplete="new-password" type="password" name="password1" />
                                <span class="icon is-small is-left">
                                    <i class="fas fa-key"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Confirm password" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateNewPassword2(event.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().clone().value().into()))} autocomplete="new-password" type="password" name="password2" />
                                <span class="icon is-small is-left">
                                    <i class="fas fa-key"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                { non_empty_html }
                { dont_match_html }
                { failed_html }

                <div class="field is-horizontal">
                    <div class="field-label">
                        <label class="label">{ "" }</label>
                    </div>
                    <div class="field-body">
                        <button onclick={ctx.link().callback(|_| Msg::SubmitPasswordRequest)} type="submit" class="button is-link">
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
