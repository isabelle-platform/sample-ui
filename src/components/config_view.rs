use isabelle_dm::data_model::item::Item;
use yew::prelude::*;

pub struct ConfigView;

impl Component for ConfigView {
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
                    <div class="list-item-title pr-5"><a href={ "/config/edit?id=".to_owned() + &ctx.props().id.to_string() }>{ ctx.props().safe_str("name", "").clone()}</a>
                        <span class="pl-5">
                            <span class={"tag is-danger ".to_string() + if ctx.props().safe_bool("role_is_active", true) { "is-hidden" } else { "" }}>{ "Inactive" }</span>
                        </span>
                    </div>
                    <div>
                        <i>
                            { "Modules:" }
                        </i>
                    </div>
                    <a class="list-item-description" href={ "/config/dhcp/edit?id=".to_owned() + &ctx.props().id.to_string() }>
                        {"· DHCP"}
                    </a>
                </div>

                <div class="list-item-controls">
                  <div class="buttons is-right">
                    <a href={ "/config/edit?id=".to_owned() + &ctx.props().id.to_string() }>
                        <button class="button is-link is-outlined">
                          <span>{"Edit"}</span>
                        </button>
                    </a>
                    <a href={ "/itm/edit?next=config_list&del=true&collection=config&id=".to_owned() + &ctx.props().id.to_string() }>
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
