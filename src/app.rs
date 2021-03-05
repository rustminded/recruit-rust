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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tech {
    pub tech: &'static str,
    pub professional: bool,
    pub public: bool,
}

impl Tech {
    fn from_tech(candidate_tech: &'static str) -> Tech {
        let tech = candidate_tech;
        let professional = false;
        let public = false;
        Tech {
            tech,
            professional,
            public,
        }
    }
    fn from_pro_tech(candidate_tech: &'static str) -> Tech {
        let tech = candidate_tech;
        let professional = true;
        let public = false;
        Tech {
            tech,
            professional,
            public,
        }
    }
    fn from_pub_tech(candidate_tech: &'static str) -> Tech {
        let tech = candidate_tech;
        let professional = false;
        let public = true;
        Tech {
            tech,
            professional,
            public,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateInfo {
    candidate: &'static Candidate,
    techs: HashSet<Tech>,
    url: &'static str,
}

impl CandidateInfo {
    fn from_candidate(
        candidate_info: &'static Candidate,
        candidate_url: &'static str,
    ) -> CandidateInfo {
        let candidate = candidate_info;
        let url = candidate_url;
        let mut techs: HashSet<Tech> = HashSet::new();

        let candidate_asked_techs = candidate.asked_techs.iter().map(|x| Tech::from_tech(x));
        techs.extend(candidate_asked_techs);

        let jobs_techs = candidate.jobs.iter().map(|x| x.techs);
        for s in jobs_techs {
            let candidate_jobs_techs = s.iter().map(|x| Tech::from_pro_tech(x));
            techs.extend(candidate_jobs_techs);
        }

        let contribs_techs = candidate.contributions.iter().map(|x| x.techs);
        for s in contribs_techs {
            let candidate_contribs_techs = s.iter().map(|x| Tech::from_pub_tech(x));
            techs.extend(candidate_contribs_techs);
        }

        let personal_techs = candidate.personal_projects.iter().map(|x| x.techs);
        for s in personal_techs {
            let candidate_personal_techs = s.iter().map(|x| Tech::from_pub_tech(x));
            techs.extend(candidate_personal_techs);
        }

        CandidateInfo {
            candidate,
            techs,
            url,
        }
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut candidates = HashMap::new();
        let candidate = (yozhgoor::candidate(), yozhgoor::candidate());
        let candidate_1 = candidate.0;
        let candidate_2 = candidate.1;

        let candidate_1_info = CandidateInfo::from_candidate(candidate_1, "yozhgoor");
        let candidate_2_info = CandidateInfo::from_candidate(candidate_2, "yozhgoor2");

        candidates.insert(candidate_1.slug, candidate_1_info);
        candidates.insert(candidate_2_info.url, candidate_2_info);
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
                                                    url={x.url}
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
