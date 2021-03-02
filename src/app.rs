use crate::profile::Profile;
use candidate::Candidate;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, IconName, InputGroup, Text, H1, H2};

pub struct App {
    candidates: HashMap<&'static str, &'static Candidate>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut candidates = HashMap::new();
        let candidate = yozhgoor::candidate();
        candidates.insert(candidate.slug, candidate);
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
        /* let candidate_profile = self
            .candidates
            .iter()
            .map(|(x, y)| y)
            .map(|y| {
                html! {
                    <Profile candidate={y} />
                }
            })
            .collect::<Html>();
        */

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
                    <div class="profile-list">
                        <H2>{"Discover the community"}</H2>
                    </div>
                    <div>
                        <Router<AppRoute, ()>
                            render = Router::render(move |switch: AppRoute| {
                                match switch {
                                    AppRoute::Home => html!(),
                                    AppRoute::Profile(candidate_slug) =>
                                    html! (
                                            <Profile
                                                candidate = candidates
                                                    .get(&candidate_slug.as_str())
                                                    .unwrap()
                                            />
                                    ),
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
    #[to = "/profile/{candidate_slug}"]
    Profile(String),
    #[to = "/"]
    Home,
}
