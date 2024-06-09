use crate::common::*;
use yew::prelude::*;

pub enum Msg {
    Test,
}

pub struct LogoutPage {}

impl Component for LogoutPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match post_logout().await {
                Ok(_md) => Msg::Test,
                Err(_err) => Msg::Test,
            }
        });

        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::Test => {
                let new_url = "/";
                web_sys::window()
                    .unwrap()
                    .location()
                    .set_href(new_url)
                    .unwrap();
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="section container">
                    <h1 class="title">{ "Logged out!" }</h1>
                </div>
            </>
        }
    }
}
