use crate::common::*;
use crate::components::baloon::BaloonView;
use yew::prelude::*;

use isabelle_dm::data_model::item::Item;

use std::collections::HashMap;

pub enum Msg {
    UpdateDevices(FetchState<HashMap<u64, Item>>),
    UpdateUsers(FetchState<HashMap<u64, Item>>),
}

pub struct Home {
    pub devices: FetchState<HashMap<u64, Item>>,
    pub users: FetchState<HashMap<u64, Item>>,
    pub t_start: u64,
    pub t_end: u64,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let t_start = (chrono::Local::now().timestamp() as u64) / 86400 * 86400;
        let t_end = t_start + 86400;

        Self {
            devices: FetchState::Fetching,
            users: FetchState::Fetching,
            t_start: t_start,
            t_end: t_end,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateDevices(fetch_state) => {
                self.devices = fetch_state;
            }
            Msg::UpdateUsers(fetch_state) => {
                self.users = fetch_state;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let devices: &HashMap<u64, Item>;
        let users: &HashMap<u64, Item>;

        let filter = "{ \"$and\": [ { \"u64s.time\": { \"$gte\": ".to_owned()
            + &self.t_start.to_string()
            + " } }, { \"u64s.time\": { \"$lt\": "
            + &self.t_end.to_string()
            + " } }] }";

        match &self.devices {
            FetchState::Fetching => {
                ctx.link().send_future(async move {
                    match fetch_itm_list(
                        "device",
                        "full",
                        u64::MAX,
                        0,
                        u64::MAX,
                        "time",
                        &filter,
                        u64::MAX,
                        u64::MAX,
                        Vec::new(),
                    )
                    .await
                    {
                        Ok(md) => Msg::UpdateDevices(FetchState::Success(md)),
                        Err(err) => Msg::UpdateDevices(FetchState::Failed(err)),
                    }
                });
                return html! {
                    <>
                        <BaloonView message={ "Loading items..." } style="info"/>
                    </>
                };
            }
            FetchState::Success(state) => {
                devices = state;
            }
            FetchState::Failed(err) => {
                return html! {
                    <>
                        <BaloonView message={ err.to_string() } style="error"/>
                    </>
                }
            }
        };
        let _current_time = chrono::offset::Local::now();
        match &self.users {
            FetchState::Fetching => {
                ctx.link().send_future(async {
                    match fetch_itm_list(
                        "user",
                        "list",
                        u64::MAX,
                        0,
                        u64::MAX,
                        "name",
                        "",
                        u64::MAX,
                        u64::MAX,
                        Vec::new(),
                    )
                    .await
                    {
                        Ok(md) => Msg::UpdateUsers(FetchState::Success(md)),
                        Err(err) => Msg::UpdateUsers(FetchState::Failed(err)),
                    }
                });
                html! {
                <>
                    <BaloonView message={ "Loading..." } style="info"/>
                </>
                }
            }
            FetchState::Success(state) => {
                users = state;
                return html! {
                    <>
                        <div class="section container">
                            <h1 class="title">{ "Today" }</h1>
                        </div>
                    </>
                };
            }
            FetchState::Failed(err) => html! {
                <>
                    <BaloonView message={ err.to_string() } style="error"/>
                </>
            },
        }
    }
}
