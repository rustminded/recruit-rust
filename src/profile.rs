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

        html! {
            <div class="candidate">
                <Card>
                    <div class="candidate-header">
                        <div class="candidate-tag">
                            {tags}
                        </div>
                        <div class="candidate-bio">
                            <Text>{self.props.candidate.bio}</Text>
                        </div>
                    </div>
                    <div class="candidate-footer">
                        <div class="candidate-name">
                            <H1>{self.props.candidate.name}</H1>
                        </div>
                        <div class="candidate-info">
                            <Text>{contract}</Text>
                            <Text>{availability}</Text>
                        </div>
                    </div>
                </Card>
            </div>
        }
    }
}
