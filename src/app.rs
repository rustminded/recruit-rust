use crate::profile::Profile;
use candidate::Candidate;
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, IconName, InputGroup, Text, H1, H2};

pub struct App {
    candidates: Vec<(&'static str, &'static Candidate)>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let candidates = vec![yozhgoor::candidate()];
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
        let candidate_profile = self
            .candidates
            .iter()
            .map(|(x, y)| y)
            .map(|y| {
                html! {
                    <Profile candidate={y} />
                }
            })
            .collect::<Html>();
        let candidate_slug = self
            .candidates
            .iter()
            .map(|(x, y)| x.to_string())
            .collect::<String>();

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
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Home => html! (<App />),
                            AppRoute::Profile(slug) => html! (
                                <Profile />
                            )
                        }
                   })
                />
                    <div class="profile-list">
                        <H2>{"Discover the community"}</H2>
                        {candidate_profile}
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/profile/{slug}"]
    Profile(String),
    #[to = "/"]
    Home,
}
