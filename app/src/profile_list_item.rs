use crate::tech_tag::TechTag;
use crate::techs::TechSet;
use candidate::{Availability, Candidate, ContractType};
use yew::prelude::*;
use yewprint::{Card, Text};

pub struct ProfileListItem {
    props: ProfileListItemProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileListItemProps {
    pub candidate: &'static Candidate,
    pub techs: TechSet,
    pub url: &'static str,
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let candidate_techs = self
            .props
            .techs
            .iter()
            .map(|x| {
                html! {
                    <TechTag
                        tech={x}
                        url={self.props.url}
                    />
                }
            })
            .collect::<Html>();
        let contract = match self.props.candidate.contract_type {
            ContractType::Contractor => "Contractor",
            ContractType::Employee => "Employee",
            ContractType::Relocate => "Employee",
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
                    <Text class=classes!("profile-certifications")>
                        {x}
                    </Text>
                }
            })
            .collect::<Html>();

        html! {
            <Card class=classes!("profile-list")>
                <div class="profile-list-header">
                    <a href=self.props.url>
                        {self.props.candidate.name}
                    </a>
                    <Text>{availability}</Text>
                    <Text>{contract}</Text>

                </div>
                <div class="profile-list-footer">
                    {candidate_techs}
                    {certifications}
                </div>
            </Card>
        }
    }
}
