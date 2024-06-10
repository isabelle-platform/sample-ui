use crate::data::lan::LanPool;
use crate::util::input::get_input;
use yew::html::Scope;

use yew::prelude::*;

pub struct ConfigEditDhcpPoolView {
    scope: Scope<Self>,
    props: Props,
    pool: LanPool,
}

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct Props {
    pub on_change: Callback<LanPool>,
    pub pool: LanPool,
}

pub enum Msg {
    UpdateLanPoolStr(String, String),
}

impl Component for ConfigEditDhcpPoolView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            scope: ctx.link().clone(),
            props: ctx.props().clone(),
            pool: ctx.props().clone().pool.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateLanPoolStr(name, value) => {
                match name.as_str() {
                    "ipv4_start" => {
                        self.pool.ipv4_start = value.clone();
                    }
                    _ => {}
                }
                self.props.on_change.emit(self.pool.clone());
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        //let id = ctx.props().id
        let scope = self.scope.clone();
        html! {
            <>
                <h3>{ "Pool " }{ self.pool.id.to_string() }</h3>
                <div class="list-item">
                    <div class="list-item-content">
                        <div class="field is-horizontal">
                            <div class="field-label is-normal">
                                <label class="label">{ "Start address (IPv4):" }</label>
                            </div>
                            <div class="field-body">
                                <div class="field">
                                    <div class="control">
                                        <input class="input" oninput={scope.callback(move |event: InputEvent| Msg::UpdateLanPoolStr("ipv4_start".to_string(), get_input(event)))} type="text"  value={ self.pool.ipv4_start.clone() }/>
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
