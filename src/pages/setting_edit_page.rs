use crate::common::*;
use crate::components::baloon::BaloonView;
use crate::util::input::*;
use isabelle_dm::data_model::item::Item;
use yew::prelude::*;

pub enum Msg {
    UpdateSettingList(FetchState<Item>),
    SaveData,
    SaveDataSucceeded,
    SaveDataFailed,
    UpdateStr(String, String),
    UpdateBool(String, bool),
}

pub struct SettingEditPage {
    all_settings: FetchState<Item>,
    item: Item,
    in_progress: bool,
}

impl Component for SettingEditPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            all_settings: FetchState::Fetching,
            item: Item::new(),
            in_progress: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSettingList(fetch_state) => {
                self.all_settings = fetch_state;
                self.item = match &self.all_settings {
                    FetchState::Success(_all_settings) => _all_settings.clone(),
                    _ => Item::new(),
                }
            }
            Msg::SaveData => {
                self.in_progress = true;
                let itm = self.item.clone();
                ctx.link().send_future(async move {
                    match post_setting_edit(itm).await {
                        Ok(_res) => Msg::SaveDataSucceeded,
                        Err(_err) => Msg::SaveDataFailed,
                    }
                });
            }
            Msg::SaveDataSucceeded => {
                self.in_progress = false;
                let new_url = "/";
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(new_url)
                    .unwrap();
            }
            Msg::SaveDataFailed => {
                self.in_progress = false;
            }
            Msg::UpdateStr(name, val) => {
                self.item.set_str(&name, &val);
            }
            Msg::UpdateBool(name, val) => {
                self.item.set_bool(&name, val);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.all_settings {
            FetchState::Fetching => {
                ctx.link().send_future(async {
                    match fetch_setting_list().await {
                        Ok(md) => Msg::UpdateSettingList(FetchState::Success(md)),
                        Err(err) => Msg::UpdateSettingList(FetchState::Failed(err)),
                    }
                });
                return html! {
                    <>
                        //<BaloonView message={ "Loading settings..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(_all_settings) => {
                return html! {
                    <>
                    <div class="section container">
                        <h1 class="title">{ "Settings" }</h1>
                        { self.add_form(ctx) }
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

impl SettingEditPage {
    fn add_form(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <>
                    <h2 class="subtitle">{ "Globals" }</h2>
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "Site name:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("site_name".to_string(), get_input(event)))} class="input" type="text" placeholder="Sample UI" name="strs[site_name]" value={ self.item.safe_str("site_name", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-sitemap"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "Site logo URL:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("site_logo".to_string(), get_input(event)))} class="input" type="text" placeholder="direct address to logo " name="strs[site_logo]" value={ self.item.safe_str("site_logo", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-image"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "Licensed to:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("licensed_to".to_string(), get_input(event)))} class="input" type="text" placeholder="The Company" name="strs[licensed_to]" value={ self.item.safe_str("licensed_to", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-scale-balanced"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <h2 class="subtitle">{ "E-Mail notifications" }</h2>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "Contact email for web requests:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("web_contact_email".to_string(), get_input(event)))} class="input" type="text" placeholder="info@example.com" name="strs[web_contact_email]" value={ self.item.safe_str("web_contact_email", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-inbox"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "SMTP server:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("smtp_server".to_string(), get_input(event)))} class="input" type="text" placeholder="smtp.gmail.com" name="strs[smtp_server]" value={ self.item.safe_str("smtp_server", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-server"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "SMTP login:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("smtp_login".to_string(), get_input(event)))} class="input" type="text" placeholder="johndoe" name="strs[smtp_login]" value={ self.item.safe_str("smtp_login", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-envelope"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "SMTP password:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("smtp_password".to_string(), get_input(event)))} class="input" type="password" name="strs[smtp_password]" value={ self.item.safe_str("smtp_password", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-key"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "SMTP From:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("smtp_from".to_string(), get_input(event)))} class="input" type="text" placeholder="John Doe <John.Doe@My.Email>" name="strs[smtp_from]" value={ self.item.safe_str("smtp_from", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-signature"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <hr/>

                    <h2 class="subtitle">{ "Google Calendar sync" }</h2>

                    <div class="field is-horizontal">
                        <div class="field-label">
                            <label class="label">{ "" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control">
                                    <label class="checkbox">
                                      <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateBool("sync_google_cal".to_string(), get_checked(event)))} type="checkbox" name="bools[sync_google_cal]" value="true" checked={ self.item.safe_bool("sync_google_cal", false) }/>
                                      { " Sync recurring events with Google" }
                                    </label>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "E-Mail:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("sync_google_email".to_string(), get_input(event)))} class="input" type="text" placeholder="John Doe <John.Doe@gmail.com>" name="strs[sync_google_email]" value={ self.item.safe_str("sync_google_email", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-envelope"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "Credentials:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("sync_google_creds".to_string(), get_input(event)))} class="input" type="text" placeholder="...long line from Google Credentials..." name="strs[sync_google_creds]" value={ self.item.safe_str("sync_google_creds", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-key"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <p class="help">{ "Please go to "}<a href="https://console.cloud.google.com">{ "Google" }</a>{ " and create APIs and Services Credentials."}</p>
                    <br/>
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{ "Calendar name:" }</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <div class="control has-icons-left">
                                    <input oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("sync_google_cal_name".to_string(), get_input(event)))} class="input" type="text" placeholder="My Calendar" name="strs[sync_google_cal_name]" value={ self.item.safe_str("sync_google_cal_name", "") }/>
                                    <span class="icon is-small is-left">
                                        <i class="fas fa-calendar"></i>
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>


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

                <hr/>
                <h2 class="subtitle">{ "Google OAuth" }</h2>
                <div class="field is-horizontal">
                    <div class="field-label">
                        <label class="label">{ "Authenticate with Google:" }</label>
                    </div>
                    <div class="field-body">
                        <button onclick={self.auth_google()} class="button is-danger">
                            <span class="icon is-small">
                                <i class="fa-brands fa-google"></i>
                            </span>
                            <span>
                                { "Start the process" }
                            </span>
                        </button>
                    </div>
                </div>
                <p class="help">{" Please save the other settings first and then authenticate." }</p>
            </>
        }
    }

    fn auth_google(&self) -> Callback<MouseEvent> {
        Callback::from(|_| {
            let new_url = "/setting/gcal_auth";
            web_sys::window()
                .unwrap()
                .location()
                .set_href(new_url)
                .unwrap();
        })
    }
}
