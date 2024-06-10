use serde::Deserialize;

use crate::common::*;
use isabelle_dm::data_model::item::Item;

use crate::components::baloon::BaloonView;

use crate::util::accessor::*;

use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    UpdateItemList(FetchState<HashMap<u64, Item>>),
    UpdateGroups(FetchState<HashMap<u64, Item>>),
    UpdateStr(String, String),
    UpdateBool(String, bool),
    UpdateU64(String, u64),
    UpdateId(String, u64),
    Connect,
    ConnectSucceeded,
    ConnectFailed,
    Fetch,
    FetchSucceeded,
    FetchFailed,
    SaveData,
    SaveDataSucceeded,
    SaveDataFailed,
}

pub struct DeviceManagePage {
    queried_id: u64,
    item: Item,
    orig_item: Item,
    items: FetchState<HashMap<u64, Item>>,
    groups: FetchState<HashMap<u64, Item>>,
    in_progress: bool,
    failed: bool,
    connected: bool,
    connect_in_progress: bool,
    fetch_in_progress: bool,
    fetched_data: String,
}

impl Component for DeviceManagePage {
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
            groups: FetchState::Fetching,
            queried_id: q.id,
            item: Item::new(),
            orig_item: Item::new(),
            in_progress: false,
            failed: false,
            connected: false,
            connect_in_progress: false,
            fetch_in_progress: false,
            fetched_data: "".to_string(),
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
            Msg::UpdateGroups(fetch_state) => {
                self.groups = fetch_state;
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
            Msg::UpdateId(name, val) => {
                self.item.set_id(&name, val);
            }
            Msg::SaveData => {
                let queried_id = self.queried_id;
                let itm = self.item.clone();
                self.in_progress = true;
                ctx.link().send_future(async move {
                    match post_itm_edit("device", queried_id, true, itm).await {
                        Ok(_res) => Msg::SaveDataSucceeded,
                        Err(_err) => Msg::SaveDataFailed,
                    }
                });
            }
            Msg::SaveDataSucceeded => {
                self.in_progress = false;
                self.failed = false;
                let new_url = "/device";
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
            Msg::Connect => {
                self.connected = false;
                self.connect_in_progress = true;
                ctx.link().send_future(async move { Msg::ConnectSucceeded });
            }
            Msg::ConnectSucceeded => {
                self.connected = true;
                self.connect_in_progress = false;
            }
            Msg::ConnectFailed => {
                self.connected = false;
                self.connect_in_progress = false;
            }
            Msg::Fetch => {
                self.fetch_in_progress = true;
                self.fetched_data = "".to_string();
                ctx.link().send_future(async move { Msg::FetchSucceeded });
            }
            Msg::FetchSucceeded => {
                self.fetch_in_progress = false;
                self.fetched_data = "Fetched data".to_string();
            }
            Msg::FetchFailed => {
                self.fetch_in_progress = false;
                self.fetched_data = "Failed to get data".to_string();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let groups;
        match &self.groups {
            FetchState::Fetching => {
                ctx.link().send_future(async move {
                    match fetch_itm_list(
                        "device_group",
                        "list",
                        u64::MAX,
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
                        Ok(md) => Msg::UpdateGroups(FetchState::Success(md)),
                        Err(err) => Msg::UpdateGroups(FetchState::Failed(err)),
                    }
                });
                return html! {
                    <>
                        <BaloonView message={ "Loading groups..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(_items) => {
                groups = _items;
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
                        "device",
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
                        <h1 class="title">{ "Manage device" }</h1>
                        { self.add_form(ctx, groups.clone()) }
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

impl DeviceManagePage {
    fn add_form(&self, ctx: &Context<Self>, groups: HashMap<u64, Item>) -> Html {
        let conn_htm = if self.connected {
            html! {
                <>
                    <BaloonView message={ "Connected" } style="info"/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        let fetching_htm = if self.fetch_in_progress {
            html! {
                <>
                    <BaloonView message={ "Fetching..." } style="info"/>
                </>
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
                    <BaloonView message={ "Failed to save device" } style="error"/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        let fetched_data_htm = html! {
            <div class="field is-horizontal">
                <div class="field-label is-normal">
                    <label class="label">{ "Fetched data" }</label>
                </div>
                <div class="field-body">
                    <div class="field">
                        <div class="control">
                            <input class="input is-static" type="text" name="fetched_data" readonly=true value={ self.fetched_data.clone() }/>
                        </div>
                    </div>
                </div>
            </div>
        };
        let _groups_list = groups.into_iter().map(|el| {
            html! {
                <option selected={ self.item.safe_id("group", u64::MAX) == el.0 } value={ el.1.id.to_string() }>{ el.1.safe_str("name", "") }</option>
            }
        });

        html! {
            <>
                { conn_htm }
                { fetching_htm }
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
                                <input disabled=true class="input" type="text" value={ self.item.safe_str("name", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-umbrella"></i>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal">
                        <label class="label">{ "IP/FQDN" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control has-icons-left">
                                <input disabled=true class="input" type="text" value={ self.item.safe_str("ip_fqdn", "").clone() }/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-umbrella"></i>
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
                        <button onclick={ctx.link().callback(|_| Msg::Connect)} type="submit" class={"button is-info ".to_owned() + if self.connect_in_progress { "is-loading"} else { "" }}>
                            <span>
                                { "Connect" }
                            </span>
                        </button>
                    </div>
                </div>

                <div class="field is-horizontal">
                    <div class="field-label">
                        <label class="label">{ "" }</label>
                    </div>
                    <div class="field-body">
                        <button onclick={ctx.link().callback(|_| Msg::Fetch)} type="submit" class={"button is-success ".to_owned() + if self.fetch_in_progress { "is-loading"} else { "" }}>
                            <span>
                                { "Fetch data" }
                            </span>
                        </button>
                    </div>
                </div>

                { fetched_data_htm }

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
