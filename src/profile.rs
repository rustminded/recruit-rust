use candidate::{Candidate, ContractType};
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

        html! {
            <div class="candidate">
                <Card>
                    <div class="header">
                        <H1>{self.props.candidate.name}</H1>
                        <Text>{self.props.candidate.bio}</Text>
                    </div>
                    <div class="tag">
                        {tags}
                    </div>
                    <div class="footer">
                        <Text>{contract}</Text>
                    </div>
                </Card>
            </div>
        }
    }
}
