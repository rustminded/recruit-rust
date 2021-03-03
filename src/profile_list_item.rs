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
        let mut tech_list: HashSet<&str> = HashSet::new();
        tech_list.extend(self.props.candidate.asked_tech);
        let jobs_list = self
            .props
            .candidate
            .jobs
            .iter()
            .map(|x| x.tech)
            .collect::<HashSet<&[&str]>>();
        for s in jobs_list.iter() {
            tech_list.extend(s.iter());
        }

        let contribs_list = self
            .props
            .candidate
            .contributions
            .iter()
            .map(|x| x.tech)
            .collect::<HashSet<&[&str]>>();
        for s in contribs_list {
            tech_list.extend(s.iter());
        }

        let personnal_list = self
            .props
            .candidate
            .personnal_projects
            .iter()
            .map(|x| x.tech)
            .collect::<HashSet<&[&str]>>();
        for s in personnal_list {
            tech_list.extend(s.iter());
        }
        let tech_list = tech_list
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
