use crate::contributions::Contributions;
use crate::jobs::Jobs;
use candidate::{Availability, Candidate, ContractType};
use yew::prelude::*;
use yewprint::{Card, Tag, Text, H1, H2};

pub struct Profile {
    props: Props,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub candidate: &'static Candidate,
}

impl Component for Profile {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Profile { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let tags = self
            .props
            .candidate
            .asked_tech
            .iter()
            .map(|x| {
                html! {
                    <div class="tag">
                        <Tag>
                            {x}
                        </Tag>
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
        let jobs_list = self
            .props
            .candidate
            .jobs
            .iter()
            .map(|x| {
                html! {
                    <Jobs jobs={x} />
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
                    <Contributions contributions={x} />
                }
            })
            .collect::<Html>();
        let personnal_list = self
            .props
            .candidate
            .personnal_projects
            .iter()
            .map(|x| {
                html! {
                    <Contributions contributions={x} />
                }
            })
            .collect::<Html>();

        html! {
            <div class="candidate">
                <Card interactive=true>
                    <div class="candidate-header-top">
                        <div class="candidate-tag">
                            {tags}
                        </div>
                        <div class="candidate-bio">
                            <Text>{self.props.candidate.bio}</Text>
                        </div>
                    </div>
                    <div class="candidate-header-bottom">
                        <div class="candidate-name">
                            <H1>{self.props.candidate.name}</H1>
                        </div>
                        <div class="candidate-urls">
                            {urls}
                        </div>
                    </div>
                    <div class="candidate-info">
                        <Text>{contract}</Text>
                        <Text>{availability}</Text>
                    </div>
                    <div class="candidate-jobs">
                        <H2>{"Jobs"}</H2>
                        {jobs_list}
                    </div>
                    <div class="candidate-contributions">
                        <H2>{"Contribution"}</H2>
                        {contrib_list}
                    </div>
                    <div class="candidate-personnal">
                        <H2>{"Personnal projects"}</H2>
                        {personnal_list}
                    </div>
                </Card>
            </div>
        }
    }
}
