use serde::Deserialize;
use crate::common::*;
use isabelle_dm::data_model::item::Item;
use crate::components::baloon::BaloonView;
use crate::components::config_edit_dhcp_pool_view::ConfigEditDhcpPoolView;
use crate::util::accessor::*;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::util::input::*;
use crate::data::lan::*;

pub struct ConfigEditDhcpPage {
    queried_id: u64,
    item: Item,
    orig_item: Item,
    items: FetchState<HashMap<u64, Item>>,
    in_progress: bool,
    failed: bool,
    lan_pools: Vec<LanPool>,
}

pub enum Msg {
    UpdateItemList(FetchState<HashMap<u64, Item>>),
    UpdateStr(String, String),
    UpdateBool(String, bool),
    UpdateU64(String, u64),
    UpdateLanPool(LanPool),
    SaveData,
    SaveDataSucceeded,
    SaveDataFailed,
    AddPool,
}

impl Component for ConfigEditDhcpPage {
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
            lan_pools: Vec::new(),
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

                            let pool_str = self.item.safe_str("lan_pool", "");

                            match serde_json::from_str::<Vec<LanPool>>(&pool_str) {
                                Ok(obj) => {
                                    self.lan_pools = obj.clone();
                                }
                                Err(_e) => {
                                    self.lan_pools = Vec::new();
                                }
                            }
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
            Msg::UpdateLanPool(pool) => {
                match self.lan_pools
                          .iter()
                          .position(|x| x.id == pool.id) {
                    Some(index) => {
                        self.lan_pools[index] = pool;
                    }
                    None => {},
                }
                self.item.set_str("lan_pool",
                    &serde_json::to_string(&self.lan_pools).unwrap());
            }
            Msg::AddPool => {
                let p = LanPool {
                    id: self.lan_pools.len() as u64,
                    enable: false,
                    ipv4_start: "".to_string(),
                    ipv4_end: "".to_string(),
                    interface: "".to_string(),
                    routers: Vec::new(),
                    client_subnet_mask: "".to_string(),
                    lease_time: 0,
                    registered: Vec::new(),
                    domain: "".to_string(),
                    dns: Vec::new(),
                    broadcast: "".to_string(),
                };
                self.lan_pools.push(p);
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
                        <h1 class="title">{ "Edit DHCP module for " }{ self.item.safe_str("name", "") }</h1>
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

impl ConfigEditDhcpPage {
    fn add_form(&self, ctx: &Context<Self>) -> Html {
        let list = self.lan_pools.clone().into_iter()
            .map(|el| {
            html! {
                <ConfigEditDhcpPoolView pool={ el } on_change={ctx.link().callback(Msg::UpdateLanPool)}/>
            }
        });
        let pools = html! {
            <>
                { for list }
            </>
        };
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

                <div class="field is-horizontal">
                    <div class="field-label">
                        <label class="label">{ "" }</label>
                    </div>
                    <div class="field-body">
                        <div class="field">
                            <div class="control">
                                <label class="checkbox">
                                  <input type="checkbox" oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateBool("mod_dhcp".to_string(), get_checked(event)))} value="true" checked={ self.item.safe_bool("mod_dhcp", false) }/>
                                  { " Enable" }
                                </label>
                            </div>
                        </div>
                    </div>
                </div>

                <hr/>

                <h2>{ "Pools"}</h2>

                { pools }

                <br/>

                <div class="field is-horizontal">
                    <div class="field-label">
                        <label class="label">{ "" }</label>
                    </div>
                    <div class="field-body">
                        <button onclick={ctx.link().callback(|_| Msg::AddPool)} type="submit" class={"button is-danger"}>
                            <span class="icon is-small">
                                <i class="fas fa-floppy-disk"></i>
                            </span>
                            <span>
                                { "Add pool" }
                            </span>
                        </button>
                    </div>
                </div>

                <hr/>

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
