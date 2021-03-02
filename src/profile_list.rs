use candidate::{Availability, Candidate, ContractType};
use yew::prelude::*;
use yewprint::{Card, Tag, Text};

pub struct ProfileList {
    props: ProfileListProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileListProps {
    pub candidate: &'static Candidate,
}

impl Component for ProfileList {
    type Message = ();
    type Properties = ProfileListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ProfileList { props }
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
                    <Tag class=classes!("tag")>
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
            <Card class=classes!("profile-list")>
                <div class="profile-list-header">
                    <div class="profile-list-tag">
                        {tags}
                    </div>
                    <a href=format!("/profile/{}", self.props.candidate.slug)>
                        {self.props.candidate.name}
                    </a>
                </div>
                <div class="profile_list_footer">
                    <Text>{availability}</Text>
                    <Text>{contract}</Text>
                </div>
            </Card>
        }
    }
}
