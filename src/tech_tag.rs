use crate::techs::Tech;
use yew::prelude::*;
use yewprint::{IconName, Intent, Tag};

pub struct TechTag {
    props: TechTagProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TechTagProps {
    pub tech: Tech,
    pub url: &'static str,
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let url = self.props.url.clone();
        let value = self.props.tech.value.clone();
        let (intent_value, icon_value) = if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == false
        {
            (Some(Intent::Warning), None)
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            (Some(Intent::Success), None)
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            (Some(Intent::Primary), None)
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            (Some(Intent::Primary), Some(IconName::Code))
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            (Some(Intent::Primary), Some(IconName::People))
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            (Some(Intent::Warning), Some(IconName::Star))
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            (Some(Intent::Warning), Some(IconName::People))
        } else {
            (None, None)
        };

        html! {
            <a href=format!("{}#{}", url, value)>
                <Tag
                    class=classes!("pro-tag")
                    interactive=true
                    intent=intent_value
                    right_icon=icon_value
                >
                    {self.props.tech.value.clone()}
                </Tag>
            </a>
        }
    }
}
