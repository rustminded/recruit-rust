use crate::profile::Profile;
use crate::profile_list_item::ProfileListItem;
use candidate::Candidate;
use std::collections::HashMap;
use std::collections::HashSet;
use yew::{prelude::*, virtual_dom::VNode};
use yew_router::{router::Router, Switch};
use yewprint::{Button, IconName, InputGroup, Tag, Text, H1, H2};

pub struct App {
    candidates: HashMap<&'static str, (&'static Candidate, HashSet<&'static str>)>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut candidates = HashMap::new();
        let candidate = yozhgoor::candidate();
        let mut tech_list: HashSet<&str> = HashSet::new();
        tech_list.extend(candidate.asked_techs);
        let jobs_list = candidate
            .jobs
            .iter()
            .map(|x| x.techs)
            .collect::<HashSet<&[&str]>>();
        for s in jobs_list.iter() {
            tech_list.extend(s.iter());
        }

        let contribs_list = candidate
            .contributions
            .iter()
            .map(|x| x.techs)
            .collect::<HashSet<&[&str]>>();
        for s in contribs_list {
            tech_list.extend(s.iter());
        }

        let personal_list = candidate
            .personal_projects
            .iter()
            .map(|x| x.techs)
            .collect::<HashSet<&[&str]>>();
        for s in personal_list {
            tech_list.extend(s.iter());
        }
        candidates.insert(candidate.slug, (candidate, tech_list));
        crate::log!("{:?}", candidates);

        App { candidates }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let candidates = self.candidates.clone();

        html! {
            <div class="app-root bp3-dark">
                <div class="app-header">
                    <H1 class=classes!("app-title")>
                        {"Welcome on Recruit-Rust.dev!"}
                    </H1>
                    <div class="app-search-field">
                        <InputGroup
                            round=true
                            placeholder="Search..."
                            right_element=html! {
                                <Button
                                    icon=IconName::Search
                                    minimal=true
                                />
                            }
                        />
                    </div>
                </div>
                <Text class=classes!("app-intro")>
                    {"The place to be hired as an awesome Rustacean"}
                </Text>
                <div class="app-content" role="main">
                    <div class="profile-list-component">
                        <H2>{"Discover the community"}</H2>
                    </div>
                    <div>
                        <Router<AppRoute, ()>
                            render=Router::render(move |switch: AppRoute| {
                                match switch {
                                    AppRoute::Home => candidates
                                        .values()
                                        .map(|x| {
                                            html! {
                                                <ProfileListItem
                                                    candidate={x}
                                                />
                                            }
                                        })
                                        .collect::<Html>(),
                                    AppRoute::Profile(candidate_slug) => html! {
                                        <Profile
                                            candidate=candidates
                                                .get(&candidate_slug.as_str())
                                                .unwrap()
                                                .0
                                        />
                                    },
                                }
                            })
                        />
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/{candidate_slug}"]
    Profile(String),
    #[to = "/"]
    Home,
}
