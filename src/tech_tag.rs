use crate::Tech;
use yew::prelude::*;
use yewprint::{IconName, Intent, Tag};

pub struct TechTag {
    props: TechTagProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TechTagProps {
    pub techs: Tech,
}

impl Component for TechTag {
    type Message = ();
    type Properties = TechTagProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TechTag { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if self.props.techs.professional == true && self.props.techs.public == false {
            html! {
                <Tag
                    large=true
                    icon=IconName::Office
                >
                    {self.props.techs.tech}
                </Tag>
            }
        } else if self.props.techs.professional == false && self.props.techs.public == true {
            html! {
                <Tag
                    large=true
                    right_icon=IconName::GitPush
                >
                    {self.props.techs.tech}
                </Tag>
            }
        } else if self.props.techs.professional == true && self.props.techs.public == true {
            html! {
                <Tag
                    large=true
                    icon=IconName::Office
                    right_icon=IconName::GitPush
                >
                    {self.props.techs.tech}
                </Tag>
            }
        } else {
            html! {
                <Tag
                    large=true
                    intent=Intent::Primary
                >
                    {self.props.techs.tech}
                </Tag>
            }
        }
    }
}
