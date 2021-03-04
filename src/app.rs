use crate::profile::Profile;
use crate::profile_list_item::ProfileListItem;
use candidate::Candidate;
use std::collections::HashMap;
use std::collections::HashSet;
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, IconName, InputGroup, Text, H1, H2};

pub struct App {
    candidates: HashMap<&'static str, CandidateInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateInfo {
    candidate: &'static Candidate,
    techs: HashSet<&'static str>,
}

impl CandidateInfo {
    fn from_candidate(candidate_info: &'static Candidate) -> CandidateInfo {
        let candidate = candidate_info;
        let mut techs: HashSet<&str> = HashSet::new();
        techs.extend(candidate.asked_techs);
        let jobs_techs = candidate
            .jobs
            .iter()
            .map(|x| x.techs)
            .collect::<HashSet<&[&str]>>();
        for s in jobs_techs.iter() {
            techs.extend(s.iter());
        }
        let contribs_techs = candidate
            .contributions
            .iter()
            .map(|x| x.techs)
            .collect::<HashSet<&[&str]>>();
        for s in contribs_techs {
            techs.extend(s.iter());
        }
        let personal_techs = candidate
            .personal_projects
            .iter()
            .map(|x| x.techs)
            .collect::<HashSet<&[&str]>>();
        for s in personal_techs {
            techs.extend(s.iter());
        }
        CandidateInfo { candidate, techs }
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut candidates = HashMap::new();
        let candidate = yozhgoor::candidate();
        let candidate_2 = yozhgoor::candidate();

        let candidate_info = CandidateInfo::from_candidate(candidate);
        let candidate_2_info = CandidateInfo::from_candidate(candidate_2);

        let candidate_2_slug = candidate_2.slug;
        let candidate_2_slug: &'static str = "yozgoor2";

        candidates.insert(candidate.slug, candidate_info);
        candidates.insert(candidate_2_slug, candidate_2_info);
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
                                                    candidate={x.candidate}
                                                    tech={&x.techs}
                                                />
                                            }
                                        })
                                        .collect::<Html>(),
                                    AppRoute::Profile(candidate_slug) => html! {
                                        <Profile
                                            candidate=candidates
                                                .get(&candidate_slug.as_str())
                                                .unwrap()
                                                .candidate
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
