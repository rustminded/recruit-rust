use crate::profile::Profile;
use crate::profile_list_item::ProfileListItem;
use crate::profile_not_found::ProfileNotFound;
use crate::techs::{Tech, TechSet};
use candidate::{Availability, Candidate};
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, IconName, InputGroup, Slider, Text, H1, H2};

pub struct App {
    candidates: Rc<HashMap<&'static str, CandidateInfo>>,
    link: ComponentLink<Self>,
    entries: Rc<TechSet>,
    value: String,
    selected_timezone: i32,
}

pub enum Msg {
    AddEntry,
    Update(String),
    Nope,
    SelectTimezone(i32),
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

        techs.extend(
            candidate
                .asked_techs
                .iter()
                .map(|x| Tech::from(*x).with_asked()),
        );

        let jobs_techs = candidate.jobs.iter().map(|x| x.techs);
        for s in jobs_techs {
            techs.extend(s.iter().map(|x| Tech::from(*x).with_pro()));
        }

        let contribs_techs = candidate.contributions.iter().map(|x| x.techs);
        for s in contribs_techs {
            techs.extend(s.iter().map(|x| Tech::from(*x).with_pub()));
        }

        let personal_techs = candidate.personal_projects.iter().map(|x| x.techs);
        for s in personal_techs {
            techs.extend(s.iter().map(|x| Tech::from(*x).with_pub()));
        }

        CandidateInfo {
            candidate,
            techs,
            url,
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let entries = TechSet::new();
        let value = String::new();
        let selected_timezone = 0;
        let mut candidates = HashMap::new();
        let candidate_1_info = CandidateInfo::from_candidate(yozhgoor::candidate(), "yozhgoor");
        let candidate_2_info = CandidateInfo::from_candidate(yozhgoor::candidate(), "yozhgoor2");

        candidates.insert(candidate_1_info.url, candidate_1_info);
        candidates.insert(candidate_2_info.url, candidate_2_info);
        let candidates = Rc::new(candidates);
        let entries = Rc::new(entries);

        App {
            candidates,
            link,
            entries,
            value,
            selected_timezone,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddEntry => {
                self.entries = Rc::new(
                    self.value
                        .split_whitespace()
                        .map(|x| Tech::from(x.to_string()))
                        .collect(),
                );
            }
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::SelectTimezone(tz) => {
                self.selected_timezone = tz;
            }
            Msg::Nope => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let candidates = Rc::clone(&self.candidates);
        let entries = Rc::clone(&self.entries);

        html! {
            <div class="app-root bp3-dark">
                <div class="app-header">
                    <H1 class=classes!("app-title")>
                        {"Welcome on Recruit-Rust.dev!"}
                    </H1>
                    <div class="app-search-field">
                        <InputGroup
                            round=true
                            placeholder="Search Techs..."
                            value=&self.value
                            oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                            onkeypress=self.link.callback(|e: KeyboardEvent| {
                                if e.key() == "Enter" { Msg::AddEntry } else { Msg::Nope }
                            })
                            right_element=html! {
                                <Button
                                    icon=IconName::Search
                                    minimal=true
                                />
                            }
                        />
                    </div>
                </div>
                <div class=classes!("app-top")>
                    <Text class=classes!("app-intro")>
                        {"The place to be hired as an awesome Rustacean"}
                    </Text>
                    <Slider<i32>
                        selected=self.selected_timezone
                        values=vec![
                            (-12, Some(String::from("UTC -12"))),
                            (-11, Some(String::from("UTC -11"))),
                            (-10, Some(String::from("UTC -10"))),
                            (-9, Some(String::from("UTC -9"))),
                            (-8, Some(String::from("UTC -8"))),
                            (-7, Some(String::from("UTC -7"))),
                            (-6, Some(String::from("UTC -6"))),
                            (-5, Some(String::from("UTC -5"))),
                            (-4, Some(String::from("UTC -4"))),
                            (-3, Some(String::from("UTC -3"))),
                            (-2, Some(String::from("UTC -2"))),
                            (-1, Some(String::from("UTC -1"))),
                            (0, Some(String::from("UTC"))),
                            (1, Some(String::from("UTC + 1"))),
                            (2, Some(String::from("UTC + 2"))),
                            (3, Some(String::from("UTC + 3"))),
                            (4, Some(String::from("UTC + 4"))),
                            (5, Some(String::from("UTC + 5"))),
                            (6, Some(String::from("UTC + 6"))),
                            (7, Some(String::from("UTC + 7"))),
                            (8, Some(String::from("UTC + 8"))),
                            (9, Some(String::from("UTC + 9"))),
                            (10, Some(String::from("UTC + 10"))),
                            (11, Some(String::from("UTC + 11"))),
                            (12, Some(String::from("UTC + 12"))),
                            (13, Some(String::from("UTC + 13"))),
                            (14, Some(String::from("UTC + 14"))),
                        ]
                        onchange=self.link.callback(|x| Msg::SelectTimezone(x))

                    />

                </div>
                <Text class=classes!("reddit-link")>
                    {"If you want to share an opportunity, check this "}
                    <a
                        href="https://www.reddit.com/r/rust/comments/kob284/official_rrust_whos_hiring_thread_for_jobseekers/"
                    >
                        {"reddit post"}
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
                                        .filter(|x|
                                            entries.is_empty() || !x.techs.is_disjoint(&entries)
                                        )
                                        .filter(|x|
                                            x.candidate.availability != Availability::NotAvailable
                                        )
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
                                    AppRoute::Profile(slug) => if let Some(candidate) =
                                        candidates.get(&slug.as_str())
                                    {
                                        html! {
                                            <Profile
                                                candidate=candidate.candidate
                                            />
                                        }
                                    } else {
                                        html! {
                                            <ProfileNotFound />
                                        }
                                    }
                                    AppRoute::ProfileHl(slug, hl_tech) => if let Some(candidate) =
                                        candidates.get(&slug.as_str())
                                    {
                                        html! {
                                            <Profile
                                                candidate=candidate.candidate
                                                highlighted_tech=hl_tech
                                            />
                                        }
                                    } else {
                                        html! {
                                            <ProfileNotFound />
                                        }
                                    }
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
    #[to = "/{slug}#{hl_tech}"]
    ProfileHl(String, String),
    #[to = "/{slug}"]
    Profile(String),
    #[to = "/"]
    Home,
}
