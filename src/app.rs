use crate::profile::Profile;
use crate::profile_list_item::ProfileListItem;
use crate::profile_not_found::ProfileNotFound;
use crate::techs::{Tech, TechSet};
use crate::utc_offset_set::UtcOffsetSet;
use candidate::{Availability, Candidate};
use chrono::Duration;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::{router::Router, Switch};
use yewprint::{Button, Collapse, IconName, InputGroup, Slider, Tag};
use yewprint::{Text, H1, H2, H3};

pub const TZ_RANGE: i64 = 2;

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
    SelectTimeZone(Duration),
    ToggleCollapse,
    Noop,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateInfo {
    candidate: &'static Candidate,
    techs: TechSet,
    url: &'static str,
    tz_offsets: UtcOffsetSet,
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

        let tz_offsets = candidate.timezones.into();

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
        let selected_timezone = Duration::hours(0);
        let mut candidates = HashMap::new();
        let candidate_1_info = CandidateInfo::from_candidate(yozhgoor::candidate(), "yozhgoor");
        let candidate_2_info = CandidateInfo::from_candidate(yozhgoor::candidate(), "yozhgoor2");
        let candidate_3_info =
            CandidateInfo::from_candidate(asia_yozhgoor::candidate(), "asia-yozhgoor");
        let candidate_4_info =
            CandidateInfo::from_candidate(australia_yozhgoor::candidate(), "australia-yozhgoor");
        let candidate_5_info =
            CandidateInfo::from_candidate(n_america_yozhgoor::candidate(), "n-america-yozhgoor");
        let candidate_6_info =
            CandidateInfo::from_candidate(s_america_yozhgoor::candidate(), "s-america-yozhgoor");
        let candidate_7_info =
            CandidateInfo::from_candidate(relocate_yozhgoor::candidate(), "relocate-yozhgoor");

        candidates.insert(candidate_1_info.url, candidate_1_info);
        candidates.insert(candidate_2_info.url, candidate_2_info);
        candidates.insert(candidate_3_info.url, candidate_3_info);
        candidates.insert(candidate_4_info.url, candidate_4_info);
        candidates.insert(candidate_5_info.url, candidate_5_info);
        candidates.insert(candidate_6_info.url, candidate_6_info);
        candidates.insert(candidate_7_info.url, candidate_7_info);

        for candidate in crate::mock::mock_candidates() {
            let info = CandidateInfo::from_candidate(candidate, candidate.slug);
            candidates.insert(info.url, info);
        }

        let candidates = Rc::new(candidates);

        App {
            candidates,
            link,
            entries: Default::default(),
            searched_value: Default::default(),
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
            Msg::SelectTimeZone(tz) => {
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
        let collapsed = self.collapsed.clone();

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
                                                    .num_hours().clamp(-12, 14),
                                                (self.selected_timezone + tz_range)
                                                    .num_hours().clamp(-12, 14),
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
                                    onchange=self.link.callback(|x| Msg::SelectTimeZone(x))
                                />
                            </Collapse>
                        </div>
                        <H3>{"Profiles:"}</H3>
                        <div>
                            <Router<AppRoute, ()>
                                render=Router::render(move |switch: AppRoute| {
                                    match switch {
                                        AppRoute::Home => {
                                            let mut sorted_vec = candidates
                                                .values()
                                                .filter(|x|
                                                    entries.is_empty() ||
                                                    !x.techs.is_disjoint(&entries)
                                                )
                                                .filter(|x|
                                                    x.candidate.availability !=
                                                    Availability::NotAvailable
                                                )
                                                .filter(|x|
                                                    collapsed || x.tz_offsets.is_empty() ||
                                                        x.tz_offsets
                                                            .iter()
                                                            .any(|x|
                                                                ((selected_timezone - tz_range)..=
                                                                    (selected_timezone + tz_range)
                                                                ).contains(x))
                                                )
                                                .collect::<Vec<_>>();
                                                sorted_vec.sort_by(|a, b|
                                                    a.tz_offsets.gap(selected_timezone)
                                                    .cmp(
                                                        &b.tz_offsets.gap(selected_timezone)
                                                    )
                                                );
                                                sorted_vec.iter().map(|x| {
                                                    html! {
                                                        <ProfileListItem
                                                            candidate={x.candidate}
                                                            techs={&x.techs}
                                                            url={x.url}
                                                        />
                                                    }
                                                })
                                                .collect()
                                        },
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
