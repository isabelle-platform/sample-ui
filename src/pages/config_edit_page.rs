use serde::Deserialize;

use crate::common::*;
use isabelle_dm::data_model::item::Item;

use crate::components::baloon::BaloonView;

use crate::util::accessor::*;

use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::util::input::*;

pub enum Msg {
    UpdateItemList(FetchState<HashMap<u64, Item>>),
    UpdateStr(String, String),
    UpdateBool(String, bool),
    UpdateU64(String, u64),
    SaveData,
    SaveDataSucceeded,
    SaveDataFailed,
}

pub struct ConfigEditPage {
    queried_id: u64,
    item: Item,
    orig_item: Item,
    items: FetchState<HashMap<u64, Item>>,
    in_progress: bool,
    failed: bool,
}

impl Component for ConfigEditPage {
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
                self.in_progress = true;
                ctx.link().send_future(async move {
                    match post_itm_edit("config", queried_id, true, itm).await {
                        Ok(_res) => Msg::SaveDataSucceeded,
                        Err(_err) => Msg::SaveDataFailed,
                    }
                });
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
                let new_url = "/config";
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
        match &self.items {
            FetchState::Fetching => {
                let queried_id = self.queried_id;
                ctx.link().send_future(async move {
                    match fetch_itm_list(
                        "config",
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
                        <BaloonView message={ "Loading items..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(_items) => {
                return html! {
                    <>
                    <div class="section container">
                        <h1 class="title">{ "Edit configuration" }</h1>
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

impl ConfigEditPage {
    fn add_form(&self, ctx: &Context<Self>) -> Html {
        let err_htm = if self.failed {
            html! {
                <>
                    <BaloonView message={ "Failed to save configuration" } style="error"/>
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
                        <label class="label">{ "Name" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("name".to_string(), get_input(event)))} type="text" value={ self.item.safe_str("name", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-umbrella"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Phone" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("phone".to_string(), get_input(event)))} type="tel" value={ self.item.safe_str("phone", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-umbrella"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "E-Mail" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("email".to_string(), get_input(event)))} type="email" value={ self.item.safe_str("email", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-envelope"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "Organization" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input class="input" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateStr("organization".to_string(), get_input(event)))} type="text" value={ self.item.safe_str("organization", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-building"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

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
