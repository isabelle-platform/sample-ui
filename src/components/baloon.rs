use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Baloon {
    pub message: String,
    pub style: String,
}

pub struct BaloonView;

impl BaloonView {
    fn get_classes(&self, style: String) -> &'static str {
        match style.as_str() {
            "error" => "notification is-danger",
            "info" => "notification is-info is-light",
            _ => "notification",
        }
    }
}

impl Component for BaloonView {
    type Message = ();
    type Properties = Baloon;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cl = self.get_classes(ctx.props().style.clone());
        html! {
            <div class={cl}>
                { ctx.props().message.clone() }
            </div>
        }
    }
}
