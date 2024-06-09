use crate::common::*;
use crate::util::accessor::*;
use isabelle_dm::data_model::del_param::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    PageUpdated,
}

pub struct ItmEditPage {
    next: String,
}

impl Component for ItmEditPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();

        #[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
        struct NextPageDetails {
            #[serde(default = "unset_str")]
            next: String,
        }

        let q2 = location.query::<NextPageDetails>().unwrap();
        let q = location.query::<DelParam>().unwrap();
        let path = location.search().clone();
        ctx.link().send_future(async move {
            let res;
            if q.del {
                res = post_directly("/itm/del", &path).await;
            } else {
                res = post_directly("/itm/edit", &path).await;
            }
            match res {
                Ok(_md) => Msg::PageUpdated,
                Err(_err) => Msg::PageUpdated,
            }
        });

        let link = ctx.link().clone();
        let _listener = ctx.link().history().unwrap().listen(move || {
            link.send_message(Msg::PageUpdated);
        });

        Self { next: q2.next }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PageUpdated => {
                let next: &str = &self.next;
                let new_url = match next {
                    "project_list" => "/project",
                    "project_diary_list" => "/project_diary",
                    "user_list" => "/user",
                    "contact_list" => "/contact",
                    "event_list" => "/event",
                    "plan_list" => "/plan",
                    &_ => "/",
                };
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(new_url)
                    .unwrap();
                /*
                self.page = current_page(ctx);
                web_sys::window().unwrap().history().unwrap().back().ok();
                let location = web_sys::window().unwrap().location();
                let href = location.href().unwrap_or_else(|_| "".into());
                let _err = location.set_href(&href);
                */
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="section container">
                <h1 class="title">{ "Entry edited!" }</h1>
            </div>
            </>
        }
    }
}
