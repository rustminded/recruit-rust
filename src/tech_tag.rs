use crate::Tech;
use yew::prelude::*;
use yewprint::{IconName, Intent, Tag};

pub struct TechTag {
    props: TechTagProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TechTagProps {
    pub tech: Tech,
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
        if self.props.tech.professional == true && self.props.tech.public == false {
            html! {
                <Tag
                    large=true
                    icon=IconName::Office
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == false && self.props.tech.public == true {
            html! {
                <Tag
                    large=true
                    right_icon=IconName::GitPush
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == true && self.props.tech.public == true {
            html! {
                <Tag
                    large=true
                    icon=IconName::Office
                    right_icon=IconName::GitPush
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else {
            html! {
                <Tag
                    large=true
                    intent=Intent::Primary
                >
                    {self.props.tech.name}
                </Tag>
            }
        }
    }
}
