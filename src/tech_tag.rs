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
        if self.props.tech.professional == true
            && self.props.tech.public == false
            && self.props.tech.asked == false
        {
            html! {
                <Tag
                    class=classes!("pro-tag")
                    intent=Intent::Warning
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == false
            && self.props.tech.public == true
            && self.props.tech.asked == false
        {
            html! {
                <Tag
                    class=classes!("pub-tag")
                    intent=Intent::Success
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == false
            && self.props.tech.public == false
            && self.props.tech.asked == true
        {
            html! {
                <Tag
                    class=classes!("asked-tag")
                    intent=Intent::Primary
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == true
            && self.props.tech.public == false
            && self.props.tech.asked == true
        {
            html! {
                <Tag
                    class=classes!("asked-pro-tag")
                    right_icon=IconName::Code
                    intent=Intent::Primary
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == false
            && self.props.tech.public == true
            && self.props.tech.asked == true
        {
            html! {
                <Tag
                    class=classes!("asked-pub-tag")
                    intent=Intent::Primary
                    right_icon=IconName::People
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == true
            && self.props.tech.public == true
            && self.props.tech.asked == true
        {
            html! {
                <Tag
                    class=classes!("three-flag-tag")
                    intent=Intent::Warning
                    right_icon=IconName::Star
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else if self.props.tech.professional == true
            && self.props.tech.public == true
            && self.props.tech.asked == false
        {
            html! {
                <Tag
                    class=classes!("pro-pub-tag")
                    intent=Intent::Warning
                    right_icon=IconName::People
                >
                    {self.props.tech.name}
                </Tag>
            }
        } else {
            html! {
                <Tag
                    class=classes!("no-flag-tag")
                >
                    {self.props.tech.name}
                </Tag>
            }
        }
    }
}
