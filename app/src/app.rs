use crate::profile::Profile;
use crate::profile_list_item::{CandidateStatus, ProfileListItem};
use crate::profile_not_found::ProfileNotFound;
use crate::techs::{Tech, TechSet};
use crate::utc_offset_set::UtcOffsetSet;
use candidate::{Availability, Candidate, ContractType};
use chrono::Duration;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew_router::{router::Router, Switch as RouteurSwitch};
use yewprint::{Button, Collapse, IconName, InputGroup, Slider, Switch, Tag};
use yewprint::{Text, H1, H2, H3};

pub const TZ_RANGE: i64 = 2;

pub struct App {
    candidates: Rc<HashMap<&'static str, CandidateInfo>>,
    link: ComponentLink<Self>,
    entries: Rc<TechSet>,
    searched_value: String,
    selected_timezone: Duration,
    show_contractor: bool,
    show_employee: bool,
    show_pending: bool,
    show_select: bool,
    show_unselect: bool,
    collapsed: bool,
    candidates_selection: HashMap<String, CandidateStatus>,
    local_storage: Option<StorageService>,
}

pub enum Msg {
    AddEntry,
    UpdateSearch(String),
    SelectTimeZone(Duration),
    ToggleEmployee,
    ToggleContractor,
    TogglePending,
    ToggleSelect,
    ToggleUnselect,
    ToggleCollapse,
    CollectStatus((&'static str, CandidateStatus)),
    ClearSelection,
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

        techs.extend(
            candidate
                .not_wanted_techs
                .iter()
                .map(|x| Tech::from(*x).with_not_wanted()),
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

        techs.sort();

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
        candidates.insert(candidate_1_info.url, candidate_1_info);

        #[cfg(feature = "mock")]
        for candidate in crate::mock::mock_candidates() {
            let info = CandidateInfo::from_candidate(candidate, candidate.slug);
            candidates.insert(info.url, info);
        }

        let candidates = Rc::new(candidates);

        let local_storage = match StorageService::new(Area::Local) {
            Ok(storage) => Some(storage),
            Err(e) => {
                crate::log!("Local storage not supported: {:?}", e);
                None
            }
        };

        let candidates_selection = local_storage
            .as_ref()
            .and_then(|storage| {
                use yew::format::Json;
                match storage.restore::<Json<Result<HashMap<String, CandidateStatus>, _>>>(
                    "candidates-selection",
                ) {
                    Json(Ok(map)) => Some(map),
                    Json(Err(_)) => None,
                }
            })
            .unwrap_or_default();

        crate::log!("Local storage: {:?}", candidates_selection);

        App {
            candidates,
            link,
            entries: Default::default(),
            searched_value: Default::default(),
            selected_timezone,
            show_contractor: true,
            show_employee: true,
            show_pending: true,
            show_select: false,
            show_unselect: false,
            collapsed: true,
            candidates_selection,
            local_storage,
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
            Msg::ToggleCollapse => {
                self.collapsed ^= true;
                true
            }
            Msg::SelectTimeZone(tz) => {
                self.selected_timezone = tz;
                true
            }
            Msg::ToggleContractor => {
                self.show_contractor ^= true;
                true
            }
            Msg::ToggleEmployee => {
                self.show_employee ^= true;
                true
            }
            Msg::TogglePending => {
                self.show_pending ^= true;
                true
            }
            Msg::ToggleSelect => {
                self.show_select ^= true;
                true
            }
            Msg::ToggleUnselect => {
                self.show_unselect ^= true;
                true
            }
            Msg::CollectStatus((slug, status)) => {
                self.candidates_selection
                    .insert(slug.to_string().clone(), status.clone());

                if let Some(storage) = &mut self.local_storage {
                    use yew::format::Json;
                    storage.store("candidates-selection", Json(&self.candidates_selection));
                }
                true
            }
            Msg::ClearSelection => {
                if let Some(storage) = &mut self.local_storage {
                    storage.remove("candidates-selection");
                }
                self.candidates_selection.clear();
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
        let show_contractor = self.show_contractor.clone();
        let show_employee = self.show_employee.clone();
        let show_pending = self.show_pending.clone();
        let show_select = self.show_select.clone();
        let show_unselect = self.show_unselect.clone();
        let link = self.link.clone();
        let candidates_selection = self.candidates_selection.clone();

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
                <Text class=classes!("rust-jobs-link")>
                    {"You can also check the Rust-Jobs "}
                    <a
                        href="https://rustjobs.dev/"
                    >
                        {"website"}
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
                                                "UTC {} to {} ({})",
                                                (self.selected_timezone - tz_range)
                                                    .num_hours(),
                                                (self.selected_timezone + tz_range)
                                                    .num_hours(),
                                                self.selected_timezone.num_hours(),
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
                                <Switch
                                    label=html!("Contractor")
                                    onclick=self.link.callback(|_| Msg::ToggleContractor)
                                    checked=show_contractor
                                />
                                <Switch
                                    label=html!("Employee")
                                    onclick=self.link.callback(|_| Msg::ToggleEmployee)
                                    checked=show_employee
                                />
                                <Switch
                                    label=html!("Pending")
                                    onclick=self.link.callback(|_| Msg::TogglePending)
                                    checked=show_pending
                                />
                                <Switch
                                    label=html!("Select")
                                    onclick=self.link.callback(|_| Msg::ToggleSelect)
                                    checked=show_select
                                />
                                <Switch
                                    label=html!("Unselect")
                                    onclick=self.link.callback(|_| Msg::ToggleUnselect)
                                    checked=show_unselect
                                />
                                <Button
                                    onclick=self.link.callback(|_| Msg::ClearSelection)
                                >
                                    {"Clear candidates selection"}
                                </Button>
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
                                                    collapsed || match candidates_selection
                                                        .get(x.candidate.slug) {
                                                            Some(CandidateStatus::Select) =>
                                                            show_select,
                                                            Some(CandidateStatus::Unselect) =>
                                                            show_unselect,
                                                            Some(CandidateStatus::Pending) | None => show_pending,
                                                        }
                                                )
                                                .filter(|x|
                                                    collapsed || match x.candidate.contract_type {
                                                        ContractType::Employee => show_employee,
                                                        ContractType::Relocate => show_employee,
                                                        ContractType::Contractor => show_contractor,
                                                        ContractType::Any =>
                                                            show_employee || show_contractor,
                                                    }
                                                )
                                                .filter(|x|
                                                    collapsed || x.tz_offsets.is_empty() ||
                                                        x.tz_offsets
                                                            .iter()
                                                            .any(|x| match selected_timezone.num_hours() {
                                                                -12 | -11 | 13 | 14 => {
                                                                    -12 == x.num_hours() ||
                                                                    -11 == x.num_hours() ||
                                                                     13 == x.num_hours() ||
                                                                     14 == x.num_hours()
                                                                },
                                                                _ => ((selected_timezone - tz_range)..=(selected_timezone + tz_range)).contains(x),
                                                            })
                                                )
                                                .collect::<Vec<_>>();
                                                sorted_vec.sort_by(|a, b|
                                                    a.tz_offsets.gap(selected_timezone)
                                                    .cmp(
                                                        &b.tz_offsets.gap(selected_timezone)
                                                    )
                                                );
                                                sorted_vec.iter().map(|x| {
                                                    let stored_status = candidates_selection.get(x.candidate.slug).map(|x| x.clone());

                                                    html! {
                                                        <ProfileListItem
                                                            candidate={x.candidate}
                                                            techs={&x.techs}
                                                            url={x.url}
                                                            stored_status={stored_status}
                                                            collect_status=link.callback(|x| Msg::CollectStatus(x))
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

#[derive(RouteurSwitch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/{slug}#{hl_tech}"]
    ProfileHl(String, String),
    #[to = "/{slug}"]
    Profile(String),
    #[to = "/"]
    Home,
}
