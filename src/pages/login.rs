use crate::common::*;
use crate::components::baloon::BaloonView;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    Login,
    LoginSucceeded,
    LoginFailed,
    UpdateLogin(String),
    UpdatePassword(String),
    Otp,
    OtpFinished,
    DoNothing,
}

pub struct LoginPage {
    login: String,
    password: String,
    failed: bool,
    in_progress: bool,
    otp_done: bool,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            login: "".to_string(),
            password: "".to_string(),
            failed: false,
            in_progress: false,
            otp_done: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::Login => {
                self.otp_done = false;
                if self.login != "" && self.password != "" {
                    let username = self.login.clone();
                    let password = self.password.clone();

                    ctx.link().send_future(async move {
                        match post_login(username, password).await {
                            Ok(res) => {
                                if res.succeeded {
                                    Msg::LoginSucceeded
                                } else {
                                    Msg::LoginFailed
                                }
                            }
                            Err(_err) => Msg::LoginFailed,
                        }
                    });
                }
            }
            Msg::LoginSucceeded => {
                self.failed = false;
                self.otp_done = false;
                let new_url = "/";
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(new_url)
                    .unwrap();
            }
            Msg::LoginFailed => {
                self.failed = true;
            }
            Msg::UpdateLogin(text) => {
                self.login = text;
            }
            Msg::UpdatePassword(text) => {
                self.password = text;
            }
            Msg::Otp => {
                self.otp_done = false;
                if self.login != "" {
                    let username = self.login.clone();
                    self.in_progress = true;
                    ctx.link().send_future(async move {
                        match post_gen_otp(username).await {
                            Ok(_res) => Msg::OtpFinished,
                            Err(_err) => Msg::OtpFinished,
                        }
                    });
                }
            }
            Msg::OtpFinished => {
                self.in_progress = false;
                self.otp_done = true;
            }
            Msg::DoNothing => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fail_htm = match self.failed {
            true => {
                html! {
                    <BaloonView message={ "Failed to log in" } style="error"/>
                }
            }
            false => {
                html! {
                    <>
                    </>
                }
            }
        };
        let in_progress_htm = match self.in_progress {
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
                <div class="section container">
                    { fail_htm }

                    { in_progress_htm }

                    { otp_done_htm }

                    <div class="container">
                        <div class="columns">
                            <div class="column is-9 is-offset-1">
                                <div class="container mb-6">
                                    <div class="columns">
                                        <div class="column is-one-half is-offset-one-half">
                                            <h3 class="is-size-3 has-text-weight-bold mb-2 has-text-centered">{ "Log in" }</h3>
                                            <div class="box">
                                                <div class="field">
                                                    <label class="label" for="user_email">{ "Login/E-Mail" }</label>
                                                    <div class="control has-icons-left">
                                                        <input onkeypress={ctx.link().callback(|e: KeyboardEvent| { if e.key() == "Enter" { Msg::Login } else { Msg::DoNothing } })} oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateLogin(event.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().clone().value().into()))} class="input" type="text" width="30" id="username" name="username" value={ self.login.clone() }/>
                                                        <span class="icon is-small is-left">
                                                            <i class="fas fa-envelope"></i>
                                                        </span>
                                                    </div>
                                                </div>
                                                <div class="field">
                                                    <label class="label" for="user_password">{ "Password" }</label>
                                                    <div class="control has-icons-left">
                                                        <input onkeypress={ctx.link().callback(|e: KeyboardEvent| { if e.key() == "Enter" { Msg::Login } else { Msg::DoNothing } })} oninput={ctx.link().callback(|event: InputEvent| Msg::UpdatePassword(event.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().clone().value().into()))} class="input" type="password" width="30" id="password" name="password" value={ self.password.clone() }/>
                                                        <span class="icon is-small is-left">
                                                            <i class="fas fa-lock"></i>
                                                        </span>
                                                    </div>
                                                </div>

                                                <p>
                                                    { "By logging in, you agree to " }<a href="/privacy_policy">{"Privacy Policy"}</a>{ "." }
                                                </p>
                                                <br/>
                                                <div class="field">
                                                    <div class="control has-text-centered">
                                                        <button onclick={ctx.link().callback(|_| Msg::Login)} class="button is-link">
                                                            <span class="icon is-small">
                                                                <i class="fas fa-right-to-bracket"></i>
                                                            </span>
                                                            <span>
                                                                { "Log in" }
                                                            </span>
                                                        </button>
                                                        <br/>
                                                        <br/>
                                                        <button onclick={ctx.link().callback(|_| Msg::Otp)} class="button">
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
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
