use crate::profile::Profile;
use crate::profile_list_item::ProfileListItem;
use candidate::Candidate;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, IconName, InputGroup, Text, H1, H2};

pub struct App {
    candidates: HashMap<&'static str, CandidateInfo>,
}

#[derive(Debug, Clone)]
pub struct Tech {
    pub name: &'static str,
    pub professional: bool,
    pub public: bool,
}

impl Tech {
    fn from_tech(name: &'static str) -> Tech {
        Tech {
            name,
            professional: false,
            public: false,
        }
    }

    fn from_pro_tech(name: &'static str) -> Tech {
        Tech {
            name,
            professional: true,
            public: false,
        }
    }

    fn from_pub_tech(name: &'static str) -> Tech {
        Tech {
            name,
            professional: false,
            public: true,
        }
    }
}

impl Hash for Tech {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Eq for Tech {}

impl PartialEq for Tech {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub struct TechSet<T>(HashSet<T>);

impl<T: Eq + Hash> TechSet<T> {
    fn new() -> TechSet<T> {
        TechSet(HashSet::new())
    }

    fn add(&mut self, elem: T) {
        self.0.insert(elem);
    }
}

impl<T: Eq + Hash> Extend<T> for TechSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.add(elem);
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

        techs.extend(candidate.asked_techs.iter().map(|x| Tech::from_tech(x)));

        let jobs_techs = candidate.jobs.iter().map(|x| x.techs);
        for s in jobs_techs {
            for v in s.iter().map(|x| Tech::from_pro_tech(x)) {
                if let Some(mut v) = techs.take(&v) {
                    v.professional = true;
                    techs.insert(v);
                } else {
                    techs.insert(v);
                }
            }
        }

        let contribs_techs = candidate.contributions.iter().map(|x| x.techs);
        for s in contribs_techs {
            for v in s.iter().map(|x| Tech::from_pub_tech(x)) {
                if let Some(mut v) = techs.take(&v) {
                    v.public = true;
                    techs.insert(v);
                } else {
                    techs.insert(v);
                }
            }
        }

        let personal_techs = candidate.personal_projects.iter().map(|x| x.techs);
        for s in personal_techs {
            for v in s.iter().map(|x| Tech::from_pub_tech(x)) {
                if let Some(mut v) = techs.take(&v) {
                    v.public = true;
                    techs.insert(v);
                } else {
                    techs.insert(v);
                }
            }
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
        let candidate_1_info = CandidateInfo::from_candidate(yozhgoor::candidate(), "yozhgoor");
        let candidate_2_info = CandidateInfo::from_candidate(yozhgoor::candidate(), "yozhgoor2");

        candidates.insert(candidate_1_info.url, candidate_1_info);
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
                                                    techs={&x.techs}
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
