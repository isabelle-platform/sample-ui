use isabelle_dm::data_model::item::Item;
use yew::prelude::*;

pub struct UserView;

impl Component for UserView {
    type Message = ();
    type Properties = Item;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="list-item">
                <div class="list-item-image">
                  <figure class="image is-64x64">
                    <img class="is-rounded" src="/avatar.png"/>
                  </figure>
                </div>

                <div class="list-item-content">
                    <div class="list-item-title pr-5"><a href={ "/user/edit?id=".to_owned() + &ctx.props().id.to_string() }>{ ctx.props().safe_str("name", "").clone()}</a>
                        <span class="pl-5">
                            <span class={"tag is-danger ".to_string() + if ctx.props().safe_bool("role_is_active", true) { "is-hidden" } else { "" }}>{ "Inactive" }</span>
                        </span>
                    </div>
                    <div class="list-item-description">{ &ctx.props().safe_str("phone", "") }</div>
                </div>

                <div class="list-item-controls">
                  <div class="buttons is-right">
                    <a href={ "tel://".to_owned() + &ctx.props().safe_str("phone", "") }
                       class={if ctx.props().safe_str("phone", "") == "" { "is-hidden" } else { "" }}>
                        <button class="button is-success is-outlined">
                          <span>{"Call"}</span>
                        </button>
                    </a>
                    <a class={if ctx.props().safe_str("phone", "") == "" { "" } else { "is-hidden" }}>
                        <button disabled=true class="button is-success is-outlined">
                          <span>{"Call"}</span>
                        </button>
                    </a>
                    <a href={ "/user/edit?id=".to_owned() + &ctx.props().id.to_string() }>
                        <button class="button is-link is-outlined">
                          <span>{"Edit"}</span>
                        </button>
                    </a>
                    <a href={ "/itm/edit?next=user_list&del=true&collection=user&id=".to_owned() + &ctx.props().id.to_string() }>
                        <button class="button is-danger is-outlined">
                          <span>{"Remove"}</span>
                        </button>
                    </a>
                  </div>
                </div>
            </div>
        }
    }
}
