use crate::jobs::Jobs;
use candidate::{Availability, Candidate, ContractType};
use yew::prelude::*;
use yewprint::{Card, Tag, Text, H1};

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
                    <Tag>
                        {x}
                    </Tag>
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
                    <li>
                        <a href={y}>{x}</a>
                    </li>
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

        html! {
            <div class="candidate">
                <Card interactive=true>
                    <div class="candidate-tag">
                        {tags}
                    </div>
                    <div class="candidate-name">
                        <H1>{self.props.candidate.name}</H1>
                    </div>
                    <div class="candidate-urls">
                        <ul>
                            {urls}
                        </ul>
                    </div>
                    <div class="candidate-bio">
                        <Text>{self.props.candidate.bio}</Text>
                    </div>
                    <div class="candidate-info">
                        <Text>{contract}</Text>
                        <Text>{availability}</Text>
                    </div>
                    <div class="candidate-jobs">
                        {jobs_list}
                    </div>
                </Card>
            </div>
        }
    }
}
