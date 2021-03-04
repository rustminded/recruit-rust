use candidate::{Availability, Candidate, ContractType};
use std::collections::HashSet;
use yew::prelude::*;
use yewprint::{Card, Tag, Text};

pub struct ProfileListItem {
    props: ProfileListItemProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileListItemProps {
    pub candidate: &'static Candidate,
}

impl Component for ProfileListItem {
    type Message = ();
    type Properties = ProfileListItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ProfileListItem { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
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
                    <a href=format!("/{}", self.props.candidate.slug)>
                        {self.props.candidate.name}
                    </a>
                    <Text>{availability}</Text>
                    <Text>{contract}</Text>
                </div>
                <div class="profile-list-footer">
                    {tech_list}
                </div>
            </Card>
        }
    }
}
