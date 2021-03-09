use crate::profile::Profile;
use crate::profile_list_item::ProfileListItem;
use candidate::Candidate;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, IconName, InputGroup, Text, H1, H2};

pub struct App {
    candidates: Rc<HashMap<&'static str, CandidateInfo>>,
}

#[derive(Debug, Clone)]
pub struct Tech {
    pub value: &'static str,
    pub is_professional: bool,
    pub is_public: bool,
    pub is_asked: bool,
}

impl Hash for Tech {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Eq for Tech {}

impl PartialEq for Tech {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(other.value)
    }
}

#[derive(Debug, Clone)]
pub struct TechSet(HashSet<Tech>);

impl TechSet {
    pub fn new() -> TechSet {
        TechSet(HashSet::new())
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Tech> {
        self.0.iter()
    }
}

impl Eq for TechSet {}

impl PartialEq for TechSet {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Extend<Tech> for TechSet {
    fn extend<I: IntoIterator<Item = Tech>>(&mut self, iter: I) {
        for elem in iter {
            if let Some(mut v) = self.0.take(&elem) {
                v.is_professional |= elem.is_professional;
                v.is_public |= elem.is_public;
                v.is_asked |= elem.is_asked;
                self.0.insert(v);
            } else {
                self.0.insert(elem);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateInfo {
    candidate: &'static Candidate,
    techs: TechSet,
    url: &'static str,
}

impl CandidateInfo {
    fn from_candidate(
        candidate_info: &'static Candidate,
        candidate_url: &'static str,
    ) -> CandidateInfo {
        let candidate = candidate_info;
        let url = candidate_url;
        let mut techs: TechSet = TechSet::new();

        techs.extend(candidate.asked_techs.iter().map(|x| Tech {
            value: x,
            is_professional: false,
            is_public: false,
            is_asked: true,
        }));

        let jobs_techs = candidate.jobs.iter().map(|x| x.techs);
        for s in jobs_techs {
            techs.extend(s.iter().map(|x| Tech {
                value: x,
                is_professional: true,
                is_public: false,
                is_asked: false,
            }));
        }

        let contribs_techs = candidate.contributions.iter().map(|x| x.techs);
        for s in contribs_techs {
            techs.extend(s.iter().map(|x| Tech {
                value: x,
                is_professional: false,
                is_public: true,
                is_asked: false,
            }));
        }

        let personal_techs = candidate.personal_projects.iter().map(|x| x.techs);
        for s in personal_techs {
            techs.extend(s.iter().map(|x| Tech {
                value: x,
                is_professional: false,
                is_public: true,
                is_asked: false,
            }));
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
        let candidates = Rc::new(candidates);

        App { candidates }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let candidates = Rc::clone(&self.candidates);

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
                <Text class=classes!("reddit-link")>
                    {"Hey, want to be even more visible ? \
                    Do not hesitate to check this "}
                    <a
                        href="https://www.reddit.com/r/rust/comments/kob284/official_rrust_whos_hiring_thread_for_jobseekers/"
                    >
                        {"hiring thread on Reddit"}
                    </a>
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
