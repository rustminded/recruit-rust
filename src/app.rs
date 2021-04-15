use crate::profile::Profile;
use crate::profile_list_item::ProfileListItem;
use crate::profile_not_found::ProfileNotFound;
use crate::techs::{Tech, TechSet};
use candidate::{Availability, Candidate};
use chrono::{Duration, TimeZone, Utc};
use chrono_tz::OffsetComponents;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, Collapse, IconName, InputGroup, Slider, Tag};
use yewprint::{Text, H1, H2, H3};

pub const TZ_RANGE: i64 = 3;

pub struct App {
    candidates: Rc<HashMap<&'static str, CandidateInfo>>,
    link: ComponentLink<Self>,
    entries: Rc<TechSet>,
    searched_value: String,
    selected_timezone: Duration,
    collapsed: bool,
}

pub enum Msg {
    AddEntry,
    UpdateSearch(String),
    SelectTimezone(Duration),
    ToggleCollapse,
    Noop,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateInfo {
    candidate: &'static Candidate,
    techs: TechSet,
    url: &'static str,
    tz_offsets: HashSet<Duration>,
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

        let tz_offsets = candidate
            .timezones
            .iter()
            .map(|tz| {
                vec![
                    tz.offset_from_utc_datetime(&Utc::now().naive_utc())
                        .base_utc_offset(),
                    tz.offset_from_utc_datetime(&Utc::now().naive_utc())
                        .dst_offset(),
                ]
            })
            .flatten()
            .collect::<HashSet<Duration>>();

        CandidateInfo {
            candidate,
            techs,
            url,
            tz_offsets,
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let entries = TechSet::new();
        let searched_value = String::new();
        let selected_timezone = Duration::hours(1);
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
            searched_value,
            selected_timezone,
            collapsed: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddEntry => {
                self.entries = Rc::new(
                    self.searched_value
                        .split_whitespace()
                        .map(|x| Tech::from(x.to_string()))
                        .collect(),
                );
                true
            }
            Msg::UpdateSearch(val) => {
                self.searched_value = val;
                true
            }
            Msg::SelectTimezone(tz) => {
                self.selected_timezone = tz;
                true
            }
            Msg::ToggleCollapse => {
                self.collapsed ^= true;
                true
            }
            Msg::Noop => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let candidates = Rc::clone(&self.candidates);
        let entries = Rc::clone(&self.entries);
        let selected_timezone = self.selected_timezone.clone();
        let tz_range = Duration::hours(TZ_RANGE);

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
                            value=&self.searched_value
                            oninput=self.link.callback(|e: InputData| Msg::UpdateSearch(e.value))
                            onkeydown=self.link.callback(|e: KeyboardEvent| {
                                if e.key() == "Enter" { Msg::AddEntry } else { Msg::Noop }
                            })
                            right_element=html! {
                                <Button
                                    icon=IconName::Search
                                    minimal=true
                                    onclick=self.link.callback(|_| Msg::AddEntry)
                                />
                            }
                        />
                    </div>
                </div>
                <Text class=classes!("app-intro")>
                    {"The place to be hired as an awesome Rustacean"}
                </Text>
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
                        <div class="timezone">
                            <Button
                                icon=if self.collapsed {
                                    IconName::Filter
                                } else {
                                    IconName::FilterOpen
                                }
                                minimal=true
                                class=classes!("timezone-button")
                                onclick=self.link.callback(|_| Msg::ToggleCollapse)
                            />
                            <Collapse
                                class=classes!("timezone-collapse")
                                is_open=!self.collapsed
                                keep_children_mounted=true
                            >
                                <div class="timezone-info">
                                    <Text>{"Timezone range: "}</Text>
                                    <Tag
                                        style="margin-left: 5px;"
                                        minimal=true
                                    >
                                        {
                                            format!(
                                                "UTC {} to {}",
                                                (self.selected_timezone - tz_range)
                                                    .num_hours(),
                                                (self.selected_timezone + tz_range)
                                                    .num_hours(),
                                            )
                                        }
                                    </Tag>
                                </div>
                                <Slider<Duration>
                                    class=classes!("timezone-slider")
                                    selected=self.selected_timezone
                                    values=(-12_i64..=14_i64)
                                        .map(|x| match x {
                                            -6 => (
                                                Duration::hours(x),
                                                Some(String::from("North America")),
                                            ),
                                            -4 => (
                                                Duration::hours(x),
                                                Some(String::from("South America")),
                                            ),
                                            1 => (
                                                Duration::hours(x),
                                                Some(String::from("Europe/Africa")),
                                            ),
                                            7 => (Duration::hours(x), Some(String::from("Asia"))),
                                            9 => (
                                                Duration::hours(x),
                                                Some(String::from("Australia"))
                                            ),
                                            _ => (Duration::hours(x), None),
                                        })
                                        .collect::<Vec<_>>()
                                    onchange=self.link.callback(|x| Msg::SelectTimezone(x))
                                />
                            </Collapse>
                        </div>
                        <H3>{"Profiles:"}</H3>
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
                                            .filter(|x|
                                                x.tz_offsets
                                                    .iter()
                                                    .any(|tz|
                                                        selected_timezone >= *tz - tz_range || selected_timezone <= *tz + tz_range
                                                    )
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
