use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;

mod common;
mod components;
mod hooks;
mod pages;
mod util;

use crate::common::*;
use crate::components::baloon::BaloonView;
use isabelle_dm::transfer_model::detailed_login_user::DetailedLoginUser;

use pages::{
    config_edit_page::ConfigEditPage, config_list_page::ConfigListPage,
    device_edit_page::DeviceEditPage, device_manage_page::DeviceManagePage, device_list_page::DeviceListPage,
    device_group_edit_page::DeviceGroupEditPage, device_group_list_page::DeviceGroupListPage,
    home::Home,
    itm_edit_page::ItmEditPage, login::LoginPage, logout::LogoutPage, page_not_found::PageNotFound,
    privacy_policy::PrivacyPolicyPage,
    setting_edit_page::SettingEditPage,
    user_edit_page::UserEditPage, user_list_page::UserListPage, user_pwd_page::UserPwdPage,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,

    #[at("/config")]
    Configs,
    #[at("/config/edit")]
    ConfigEdit,
    #[at("/config/edit?id={id}")]
    ConfigEditId { id: String },

    #[at("/device")]
    Devices,
    #[at("/device/edit")]
    DeviceEdit,
    #[at("/device/edit?id={id}")]
    DeviceEditId { id: String },
    #[at("/device/manage")]
    DeviceManage,
    #[at("/device/manage?id={id}")]
    DeviceManageId { id: String },

    #[at("/device_group")]
    DeviceGroups,
    #[at("/device_group/edit")]
    DeviceGroupEdit,
    #[at("/device_group/edit?id={id}")]
    DeviceGroupEditId { id: String },

    #[at("/user")]
    Users,
    #[at("/user/edit")]
    UserEdit,
    #[at("/user/edit?id={id}")]
    UserEditId { id: String },

    #[at("/user/pwd")]
    UserPwd,

    #[at("/itm/edit")]
    ItmEdit,

    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/privacy_policy")]
    PrivacyPolicy,

    #[at("/setting/edit")]
    SettingEdit,
}

pub enum Msg {
    ToggleNavbar,
    UserLoaded(FetchState<DetailedLoginUser>),
}

pub struct Model {
    navbar_active: bool,
    login_state: FetchState<DetailedLoginUser>,
    footer_opacity: u64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: true,
            login_state: FetchState::Fetching,
            footer_opacity: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::UserLoaded(login_state) => {
                self.login_state = login_state;
                if let Some(document) = window().unwrap().document() {
                    if let Some(element) = document.get_element_by_id("footer_bar") {
                        if let Ok(element) = element.dyn_into::<HtmlElement>() {
                            element
                                .style()
                                .set_property("opacity", &(1).to_string())
                                .unwrap();
                        }
                    }
                    if let Some(element) = document.get_element_by_id("real_content") {
                        if let Ok(element) = element.dyn_into::<HtmlElement>() {
                            element
                                .style()
                                .set_property("opacity", &(1).to_string())
                                .unwrap();
                        }
                    }
                    if let Some(element) = document.get_element_by_id("nav_bar") {
                        if let Ok(element) = element.dyn_into::<HtmlElement>() {
                            element
                                .style()
                                .set_property("opacity", &(1).to_string())
                                .unwrap();
                        }
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut error_msg_htm = html! {
            <>
            </>
        };

        let mut content = html! {
            <>
            </>
        };

        let mut login_state: Option<DetailedLoginUser> = None;
        match &self.login_state {
            FetchState::Fetching => {
                ctx.link().send_future(async {
                    match fetch_is_logged_in().await {
                        Ok(md) => Msg::UserLoaded(FetchState::Success(md)),
                        Err(err) => Msg::UserLoaded(FetchState::Failed(err)),
                    }
                });
            }
            FetchState::Success(new_login_state) => {
                login_state = Some(new_login_state.clone());
                content = html! {
                    <>
                        <Switch<Route> render={Switch::render(switch)} />
                    </>
                };
            }
            FetchState::Failed(err) => {
                error_msg_htm = html! {
                    <>
                        <BaloonView message={ err.to_string() } style="error"/>
                    </>
                };
            }
        };

        let empty_lu = DetailedLoginUser {
            username: "".to_string(),
            id: 0,
            role: Vec::new(),
            site_name: "".to_string(),
            site_logo: "".to_string(),
            licensed_to: "".to_string(),
        };
        let nav = html! {
            <>
                <div id="nav_bar" style={ "opacity: 0; transition: opacity 0.3s;".to_string()}>
                    { self.view_nav(ctx, if login_state.as_ref().is_none() { &empty_lu } else { &login_state.as_ref().unwrap() }) }
                </div>
            </>
        };

        let wrapped_content = html! {
            <>
                <div id="real_content" style={ "opacity: 0; transition: opacity 1s;".to_string()}>
                    { content }
                </div>
            </>
        };
        let footer = html! {
            <>
                /*
                <footer id="footer_bar" class="footer" style={ "bottom: 0; padding: 1em; position: sticky; opacity: ".to_owned() + &self.footer_opacity.to_string() + &"; transition: opacity 0.3s;".to_string() }>
                    <nav class="columns is-vcentered">
                        <div align="center" class="column">
                            { "Developed by "}<strong>{"Interpretica, Unipessoal Lda"}</strong>{". All rights reserved. Licensed to "}<strong>{ if login_state.as_ref().is_none() { "end user".to_string() } else { login_state.as_ref().unwrap().licensed_to.to_string() }}</strong>{ "." }
                            <br/>
                            <a href="/privacy_policy">{"Privacy policy"}</a>
                        </div>
                    </nav>
                </footer>
                */
            </>
        };
        return html! {
            <BrowserRouter>
                <main>
                    <section class="section m-0 p-0">
                        <div class="container">
                            <div class="columns">
                                <div class="column is-9 is-offset-1">
                                    { error_msg_htm }
                                    { nav }
                                    { wrapped_content }
                                    { footer }
                                </div>
                            </div>
                        </div>
                    </section>
                </main>
            </BrowserRouter>
        };
    }
}
impl Model {
    fn view_nav(&self, ctx: &Context<Self>, user: &DetailedLoginUser) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar mb-5 mt-5" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <div class="navbar-item">
                        <span>
                            <a href="/">
                                <img src={ user.site_logo.to_string() } style="height: 3rem; width: 3rem; max-height: 3rem"/>
                            </a>
                        </span>
                        <span class="pl-3">
                            <a href="/">
                                <strong style="color: black; font-size: x-large; text-shadow: 0 1px 0 rgba(0, 0, 0, 0.4);">{ user.site_name.to_string() }</strong>
                            </a>
                        </span>
                    </div>
                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={ctx.link().callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-end">
                        <Link<Route> to={Route::Configs} classes={ "navbar-item" }>{ "Configurations" }</Link<Route>>
                        <Link<Route> to={Route::Users} classes={ "navbar-item" }>{ "People" }</Link<Route>>
                        <Link<Route> to={Route::Devices} classes={ "navbar-item" }>{ "Devices" }</Link<Route>>
                        <Link<Route> to={Route::DeviceGroups} classes={ "navbar-item" }>{ "Device groups" }</Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <a class="navbar-link">
                              { if user.username.to_string() != "" { user.username.to_string() } else { "Extras".to_string() }}
                            </a>
                            <div class="navbar-dropdown is-boxed">
                                <Link<Route> to={Route::UserEditId { id: user.id.to_string() }} classes={classes!("navbar-item", if user.username.to_string() == "" { "is-hidden" } else { "" })}>{ "My profile" }</Link<Route>>
                                <hr class={"navbar-divider ".to_owned() + { if user.username.to_string() == "" { "is-hidden" } else { "" } }}/>
                                <Link<Route> to={Route::SettingEdit} classes={ "navbar-item" }>{ "Settings" }</Link<Route>>
                                <hr class={"navbar-divider ".to_owned() + { if user.username.to_string() == "" { "is-hidden" } else { "" } }}/>
                                <Link<Route> to={Route::Logout} classes={classes!("navbar-item", if user.username.to_string() == "" { "is-hidden" } else { "" })}>{ "Log out" }</Link<Route>>
                            </div>
                        </div>
                      <div class="navbar-item">
                        <div class="buttons">
                            <a href="/login" class={"navbar-item button is-link ".to_owned() + (if user.username.to_string() != "" { "is-hidden" } else { "" })}>
                                { "Log in"}
                            </a>
                        </div>
                      </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(route: &Route) -> Html {
    match route.clone() {
        Route::Login => {
            html! { <LoginPage /> }
        }
        Route::Logout => {
            html! { <LogoutPage /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::Configs => {
            html! { <ConfigListPage /> }
        }
        Route::ConfigEdit => {
            html! { <ConfigEditPage /> }
        }
        Route::ConfigEditId { id: _ } => {
            html! { <ConfigEditPage /> }
        }
        Route::ItmEdit => {
            html! { <ItmEditPage /> }
        }
        Route::Users => {
            html! { <UserListPage /> }
        }
        Route::UserEdit => {
            html! { <UserEditPage /> }
        }
        Route::UserEditId { id: _ } => {
            html! { <UserEditPage /> }
        }
        Route::UserPwd => {
            html! { <UserPwdPage /> }
        }
        Route::Devices => {
            html! { <DeviceListPage /> }
        }
        Route::DeviceEdit => {
            html! { <DeviceEditPage /> }
        }
        Route::DeviceEditId { id: _ } => {
            html! { <DeviceEditPage /> }
        }
        Route::DeviceManage => {
            html! { <DeviceManagePage /> }
        }
        Route::DeviceManageId { id: _ } => {
            html! { <DeviceManagePage /> }
        }
        Route::DeviceGroups => {
            html! { <DeviceGroupListPage /> }
        }
        Route::DeviceGroupEdit => {
            html! { <DeviceGroupEditPage /> }
        }
        Route::DeviceGroupEditId { id: _ } => {
            html! { <DeviceGroupEditPage /> }
        }
        Route::SettingEdit => {
            html! { <SettingEditPage /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::PrivacyPolicy => {
            html! { <PrivacyPolicyPage /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
