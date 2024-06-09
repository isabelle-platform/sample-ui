use crate::common::*;
use crate::components::baloon::BaloonView;
use crate::components::module_view::ModuleView;
use crate::util::accessor::*;
use crate::util::cmp::name_cmp;
use isabelle_dm::data_model::list_result::ListResult;

use serde::Deserialize;

use yew::prelude::*;
use yew_router::prelude::*;

pub enum Msg {
    UpdateState(FetchState<ListResult>),
}

pub struct ModuleListPage {
    state: FetchState<ListResult>,
    filter: String,
    find: String,
    skip: u64,
    limit: u64,
}

impl Component for ModuleListPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();

        #[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
        pub struct FilterParams {
            #[serde(default = "unset_str")]
            pub filter: String,
            #[serde(default = "unset_str")]
            pub find: String,
            #[serde(default = "unset_max")]
            pub skip: u64,
            #[serde(default = "unset_max")]
            pub limit: u64,
        }

        let q = &location.query::<FilterParams>().unwrap();

        Self {
            state: FetchState::Fetching,
            filter: q.filter.clone(),
            find: q.find.clone(),
            skip: if q.skip != u64::MAX { q.skip } else { 0 },
            limit: if q.limit != u64::MAX { q.limit } else { 10 },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateState(fetch_state) => {
                self.state = fetch_state;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let skip = self.skip;
        let limit = self.limit;

        match &self.state {
            FetchState::Fetching => {
                ctx.link().send_future(async move {
                    match fetch_itm(
                        "module",
                        "list",
                        u64::MAX,
                        u64::MAX,
                        u64::MAX,
                        "name",
                        "",
                        skip,
                        limit,
                        Vec::new(),
                    )
                    .await
                    {
                        Ok(md) => Msg::UpdateState(FetchState::Success(md)),
                        Err(err) => Msg::UpdateState(FetchState::Failed(err)),
                    }
                });
                html! {
                <>
                </>
                }
            }
            FetchState::Success(data) => html! {
                <>
                    <div class="section container">
                        <h1 class="title">{ "Modules" }</h1>
                        <hr/>
                        { self.view_modules(ctx, data) }
                    </div>
                </>
            },
            FetchState::Failed(err) => html! {
                <>
                    <BaloonView message={ err.to_string() } style="error"/>
                </>
            },
        }
    }
}

impl ModuleListPage {
    fn view_modules(&self, _ctx: &Context<Self>, data: &ListResult) -> Html {
        let tmp_filter = self.find.clone().to_lowercase();
        let mut tmp_items = data.map.clone().into_iter().collect::<Vec<_>>();
        tmp_items.sort_by(|a, b| name_cmp(&a.1, &b.1));
        let list = tmp_items.into_iter()
            .filter(|x| (self.filter == "" || x.1.safe_bool(&self.filter, false)))
            .filter(|x| x.1.safe_str("name", "<unknown>").to_lowercase().contains(&tmp_filter))
            .map(|el| {
            html! {
                <ModuleView id={el.1.id} strs={el.1.strs.clone()} strids={el.1.strids.clone()} strstrs={el.1.strstrs.clone()} bools={el.1.bools.clone()} ids={el.1.ids.clone()} u64s={el.1.u64s.clone()}/>
            }
        });

        let mut eff_limit = self.limit;
        if eff_limit == u64::MAX {
            eff_limit = 10;
        }

        let mut eff_start = self.skip;
        if eff_start == u64::MAX {
            eff_start = 0;
        }

        let mut eff_end = eff_start + eff_limit;
        if eff_end > data.total_count {
            eff_end = data.total_count;
        }

        let prev_page = if eff_start > eff_limit {
            Some(eff_start - eff_limit)
        } else {
            if eff_start == 0 {
                None
            } else {
                Some(1)
            }
        };
        let next_page = if eff_end < data.total_count {
            Some(eff_end)
        } else {
            None
        };

        let mut prev_page_htm = html! {
            <>
            </>
        };
        if !prev_page.is_none() {
            prev_page_htm = html! {
                <>
                    <a href={ "/module?filter=".to_owned() + &self.filter.to_string() + "&find=" + &self.find.to_string() + "&skip=" + &((prev_page.unwrap() - 1) * eff_limit).to_string() + "&limit=" + &eff_limit.to_string() } class="pagination-previous">{"Previous"}</a>
                </>
            }
        }

        let mut next_page_htm = html! {
            <>
            </>
        };
        if !next_page.is_none() {
            next_page_htm = html! {
                <>
                    <a href={ "/module?filter=".to_owned() + &self.filter.to_string() + "&find=" + &self.find.to_string() + "&skip=" + &((next_page.unwrap() - 1) * eff_limit).to_string() + "&limit=" + &eff_limit.to_string() } class="pagination-next">{"Next"}</a>
                </>
            }
        }

        let mid_page = if data.total_count != 0 && eff_limit != 0 {
            (eff_start + eff_limit) / eff_limit
        } else {
            1
        };

        let last_page = if eff_limit != 0 {
            data.total_count / eff_limit
        } else {
            1
        };

        let mut prev_page_htm2 = html! {
            <>
            </>
        };
        if !prev_page.is_none() && mid_page > 1 {
            prev_page_htm2 = html! {
                <>
                    <li>
                        <a href={ "/module?filter=".to_owned() + &self.filter.to_string() + "&find=" + &self.find.to_string() + "&skip=" + &((mid_page - 2) * eff_limit).to_string() + "&limit=" + &eff_limit.to_string() } class="pagination-previous" aria-label={ "Goto page ".to_owned() + &(mid_page - 1).to_string() }>{ (mid_page - 1).to_string() }</a>
                    </li>
                </>
            }
        };

        let mut next_page_htm2 = html! {
            <>
            </>
        };
        if !next_page.is_none() && mid_page < last_page {
            next_page_htm2 = html! {
                <>
                    <li>
                        <a href={ "/module?filter=".to_owned() + &self.filter.to_string() + "&find=" + &self.find.to_string() + "&skip=" + &((mid_page) * eff_limit).to_string() + "&limit=" + &eff_limit.to_string() } class="pagination-previous" aria-label={ "Goto page ".to_owned() + &(mid_page + 1).to_string() }>{ (mid_page + 1).to_string() }</a>
                    </li>
                </>
            }
        };

        let first_page_ellipsis_htm2 = if mid_page > 3 {
            html! {
                <>
                    <li>
                      <span class="pagination-ellipsis">{"..."}</span>
                    </li>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        let first_page_htm2 = if mid_page > 2 {
            html! {
                <>
                    <li>
                      <a href={ "/module?filter=".to_owned() + &self.filter.to_string() + "&find=" + &self.find.to_string() + "&skip=0&limit=" + &eff_limit.to_string() } class="pagination-link" aria-label="Goto page 1">{"1"}</a>
                    </li>
                    { first_page_ellipsis_htm2 }
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };

        let last_page_ellipsis_htm2 = if (mid_page + 2) < last_page {
            html! {
                <>
                    <li>
                      <span class="pagination-ellipsis">{"..."}</span>
                    </li>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        };
        let last_page_htm2 = if (mid_page + 1) < last_page {
            html! {
                <>
                    { last_page_ellipsis_htm2 }
                    <li>
                      <a href={ "/module?filter=".to_owned() + &self.filter.to_string() + "&find=" + &self.find.to_string() + "&skip=" + &((last_page - 1) * eff_limit).to_string() + "&limit=" + &eff_limit.to_string() } class="pagination-link" aria-label={"Goto page ".to_owned() + &last_page.to_string()}>{ last_page.to_string() }</a>
                    </li>
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
                <nav class="level">
                    <form enctype={ "multipart/form-data" } class="level-left">
                      <div class="level-left">
                        <div class="level-item">
                          <p class="subtitle is-5">
                            <strong>{ data.total_count.to_string() }</strong>{" module(s)"}
                          </p>
                        </div>
                        <div class="level-item">
                          <div class="field has-addons">
                            <p class="control has-icons-left">
                                <input class="input" name="find" type="text" placeholder="Find module"/>
                                <span class="icon is-small is-left">
                                    <i class="fas fa-magnifying-glass"></i>
                                </span>
                            </p>
                            <p class="control">
                              <button class="button">
                                {"Search"}
                              </button>
                            </p>
                          </div>
                        </div>
                      </div>
                    </form>

                    <div class="level-right">
                        <p class="level-item"><a href="/module" class={if self.filter == "" { "has-text-weight-bold" } else { "" }}>{"All"}</a></p>
                        <p class="level-item pl-2">
                            <button onclick={self.add_module()} class="button">
                                <span class="icon is-small">
                                    <i class="fas fa-plus"></i>
                                </span>
                                <span>
                                    { "Add" }
                                </span>
                            </button>
                        </p>
                    </div>
                </nav>
                <nav class="pagination" role="navigation" aria-label="pagination">
                    { prev_page_htm }
                    { next_page_htm }
                    <ul class="pagination-list">
                        { first_page_htm2 }
                        { prev_page_htm2 }
                        <li>
                          <a class="pagination-link is-current" aria-label={"Page ".to_owned() + &mid_page.to_string() } aria-current="page">{ mid_page.to_string() }</a>
                        </li>
                        { next_page_htm2 }
                        { last_page_htm2 }
                    </ul>
                </nav>
                <div class="list">
                    { for list }
                </div>
            </>
        }
    }

    fn add_module(&self) -> Callback<MouseEvent> {
        Callback::from(|_| {
            let new_url = "/module/edit";
            web_sys::window()
                .unwrap()
                .location()
                .set_href(new_url)
                .unwrap();
        })
    }
}
