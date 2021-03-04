use candidate::{Availability, Candidate, ContractType};
use std::collections::HashSet;
use yew::prelude::*;
use yewprint::{Card, Tag, Text};

pub struct ProfileListItem {
    props: ProfileListItemProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileListItemProps {
    pub candidate: (&'static Candidate, HashSet<&'static str>),
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
        let candidate_info = self.props.candidate.0;
        let candidate_tech = &self.props.candidate.1;
        let candidate_tech = candidate_tech
            .iter()
            .map(|x| {
                html! {
                    <Tag>
                        {x}
                    </Tag>
                }
            })
            .collect::<Html>();
        let contract = match candidate_info.contract_type {
            ContractType::Contractor => "Contractor",
            ContractType::Employee => "Employee",
            ContractType::Any => "Any",
        };

        let availability = match candidate_info.availability {
            Availability::FullTime => "Full time",
            Availability::PartTime => "Part time",
            Availability::NotAvailable => "Not available",
        };

        html! {
            <Card class=classes!("profile-list")>
                <div class="profile-list-header">
                    <a href=format!("/{}", candidate_info.slug)>
                        {candidate_info.name}
                    </a>
                    <Text>{availability}</Text>
                    <Text>{contract}</Text>
                </div>
                <div class="profile-list-footer">
                    {candidate_tech}
                </div>
            </Card>
        }
    }
}
