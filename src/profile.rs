use crate::contributions::Contributions;
use crate::jobs::Jobs;
use candidate::{Availability, Candidate, ContractType};
use chrono::{Datelike, NaiveDate, Utc};
use itertools::Itertools;
use yew::prelude::*;
use yewprint::{Card, Intent, Tag, Text, H1, H2, H3};

pub struct Profile {
    props: ProfileProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileProps {
    pub candidate: &'static Candidate,
    #[prop_or_default]
    pub highlighted_tech: Option<String>,
}

pub enum Msg {
    Highlight(String),
}

impl Component for Profile {
    type Message = Msg;
    type Properties = ProfileProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Profile { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Highlight(value) => {
                if let Some(highlighted_tech) = self.props.highlighted_tech.as_mut() {
                    *highlighted_tech = value;
                } else {
                    self.props.highlighted_tech = Some(value);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let tags = self
            .props
            .candidate
            .asked_techs
            .iter()
            .map(|x| {
                html! {
                    <Tag
                        class=classes!("tag")
                        interactive=true
                        intent={
                            match self.props.highlighted_tech.as_ref() {
                                Some(highlighted_tech) if highlighted_tech == x => Intent::Danger,
                                _ => Intent::Primary,
                            }
                        }
                        onclick=self.link.callback(move |_| Msg::Highlight(x.to_string()))
                    >
                        {x}
                    </Tag>
                }
            })
            .collect::<Html>();
        let pronouns = self.props.candidate.pronouns.iter().join("/");
        let urls = self
            .props
            .candidate
            .urls
            .iter()
            .map(|(x, y)| {
                html! {
                    <div class="url">
                        <a href={y}>{x}</a>
                        {" | "}
                    </div>
                }
            })
            .collect::<Html>();
        let contract = match self.props.candidate.contract_type {
            ContractType::Contractor => "Contractor",
            ContractType::Employee => "Employee",
            ContractType::Any => "Any",
        };
        let availability = match self.props.candidate.availability {
            Availability::FullTime => "Full time",
            Availability::PartTime => "Part time",
            Availability::NotAvailable => "Not available",
        };
        let certifications = self
            .props
            .candidate
            .certifications
            .iter()
            .map(|x| {
                html! {
                    <Text>{x}</Text>
                }
            })
            .collect::<Html>();
        let jobs_list = self
            .props
            .candidate
            .jobs
            .iter()
            .map(|x| {
                html! {
                    <Jobs
                        job={x}
                        highlighted_tech=self.props.highlighted_tech.clone()
                        onclick=self.link.callback(|x| Msg::Highlight(x))
                    />
                }
            })
            .collect::<Html>();
        let contrib_list = self
            .props
            .candidate
            .contributions
            .iter()
            .map(|x| {
                html! {
                    <Contributions
                        contribution={x}
                        highlighted_tech=self.props.highlighted_tech.clone()
                        onclick=self.link.callback(|x| Msg::Highlight(x))
                    />
                }
            })
            .collect::<Html>();
        let personal_list = self
            .props
            .candidate
            .personal_projects
            .iter()
            .map(|x| {
                html! {
                    <Contributions
                        contribution={x}
                        highlighted_tech=self.props.highlighted_tech.clone()
                        onclick=self.link.callback(|x| Msg::Highlight(x))
                    />
                }
            })
            .collect::<Html>();
        let age = {
            let (y, m, d) = self.props.candidate.birthday_ymd;
            let birth_date = NaiveDate::from_ymd(*y, *m, *d);
            let now = Utc::now().naive_utc().date();
            let age = now.year() - birth_date.year();

            if (now.month(), now.day()) < (birth_date.month(), birth_date.day()) {
                age - 1
            } else {
                age
            }
        };

        html! {
            <Card
                class=classes!("candidate")
            >
                <div class="candidate-header-top">
                    <div class="candidate-tag">
                        {tags}
                    </div>
                    <Text class=classes!("candidate-bio")>{self.props.candidate.bio}</Text>
                </div>
                <div class="candidate-header-center">
                    <div class="candidate-name">
                        <H1>{self.props.candidate.name}</H1>
                        <Text class=classes!("candidate-pronouns")>{"("}{pronouns}{")"}</Text>
                    </div>
                    <div class="candidate-urls">
                        {urls}
                    </div>
                </div>
                <div class="candidate-header-bottom">
                    <div class="candidate-other">
                        <Text>{format!("{:?} years old", age)}</Text>
                        <Text>{contract}</Text>
                        <Text>{availability}</Text>
                    </div>
                    <div class ="candidate-certifications">
                        <H3>{"Certifications"}</H3>
                        {certifications}
                    </div>
                </div>
                <div class="candidate-jobs">
                    <H2>{"Jobs"}</H2>
                    {jobs_list}
                </div>
                <div class="candidate-contributions">
                    <H2>{"Contribution"}</H2>
                    {contrib_list}
                </div>
                <div class="candidate-personal">
                    <H2>{"Personal projects"}</H2>
                    {personal_list}
                </div>
            </Card>
        }
    }
}
