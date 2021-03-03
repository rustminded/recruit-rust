use candidate::{Availability, Candidate, ContractType};
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
        let mut tech_list: Vec<&str> = Vec::new();
        tech_list.extend(self.props.candidate.asked_tech);
        let jobs_list = self
            .props
            .candidate
            .jobs
            .iter()
            .map(|x| x.tech)
            .collect::<Vec<&[&str]>>();
        for s in jobs_list.iter() {
            tech_list.extend(s.iter());
        }

        let contribs_list = self
            .props
            .candidate
            .contributions
            .iter()
            .map(|x| x.tech)
            .collect::<Vec<&[&str]>>();
        for s in contribs_list.iter() {
            tech_list.extend(s.iter());
        }

        let personnal_list = self
            .props
            .candidate
            .personnal_projects
            .iter()
            .map(|x| x.tech)
            .collect::<Vec<&[&str]>>();
        for s in personnal_list.iter() {
            tech_list.extend(s.iter());
        }

        crate::log!("{:?}", tech_list);

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
                    <a href=format!("/{}", self.props.candidate.slug)>
                        {self.props.candidate.name}
                    </a>
                    <Text>{availability}</Text>
                    <Text>{contract}</Text>
                </div>
                <div class="profile-list-footer">
                    {tags}
                </div>
            </Card>
        }
    }
}
