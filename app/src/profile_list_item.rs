use crate::tech_tag::TechTag;
use crate::techs::TechSet;
use candidate::{Availability, Candidate, ContractType};
use yew::prelude::*;
use yewprint::{Button, ButtonGroup, Card, Intent, Text};

pub struct ProfileListItem {
    link: ComponentLink<Self>,
    props: ProfileListItemProps,
    status: CandidateStatus,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileListItemProps {
    pub candidate: &'static Candidate,
    pub techs: TechSet,
    pub url: &'static str,
    pub collect_status: Callback<(&'static str, CandidateStatus)>,
}

pub enum Msg {
    CandidateSelectionStatus(CandidateStatus),
}

impl Component for ProfileListItem {
    type Message = Msg;
    type Properties = ProfileListItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ProfileListItem {
            link,
            props,
            status: CandidateStatus::Pending,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CandidateSelectionStatus(status) => match status {
                CandidateStatus::Pending => self.status = status,
                CandidateStatus::Select => {
                    self.status = status;
                    let slug = self.props.candidate.slug.clone();
                    self.props.collect_status.emit((slug, status.clone()));
                }
                CandidateStatus::Unselect => {
                    self.status = status;
                    let slug = self.props.candidate.slug.clone();
                    self.props.collect_status.emit((slug, status.clone()));
                }
            },
        }
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
                <div class="candidate-selection-status">
                    <ButtonGroup
                        vertical=true
                    >
                        <Button
                            onclick=self.link.callback(|_| Msg::CandidateSelectionStatus(CandidateStatus::Pending))
                            intent={if self.status == CandidateStatus::Pending {
                                Some(Intent::Primary)
                            } else {
                                None
                            }}
                        >
                            {"Pending"}
                        </Button>
                        <Button
                            onclick=self.link.callback(|_| Msg::CandidateSelectionStatus(CandidateStatus::Select))
                            intent={if self.status == CandidateStatus::Select {
                                Some(Intent::Success)
                            } else {
                                None
                            }}

                        >
                            {"Select"}
                        </Button>
                        <Button
                            onclick=self.link.callback(|_| Msg::CandidateSelectionStatus(CandidateStatus::Unselect))
                            intent={if self.status == CandidateStatus::Unselect {
                                Some(Intent::Warning)
                            } else {
                                None
                            }}
                        >
                            {"Unselect"}
                        </Button>
                    </ButtonGroup>
                </div>
            </Card>
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CandidateStatus {
    Pending,
    Select,
    Unselect,
}
