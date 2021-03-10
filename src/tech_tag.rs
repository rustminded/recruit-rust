use crate::AppRoute;
use crate::Tech;
use yew::prelude::*;
use yew_router::agent::{RouteAgentDispatcher, RouteRequest};
use yewprint::{IconName, Intent, Tag};

pub struct TechTag {
    props: TechTagProps,
    link: ComponentLink<Self>,
    route_dispatcher: RouteAgentDispatcher,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TechTagProps {
    pub tech: Tech,
    pub url: &'static str,
}

pub enum Msg {
    GoToRoute(AppRoute),
}

impl Component for TechTag {
    type Message = Msg;
    type Properties = TechTagProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TechTag {
            props,
            link,
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoToRoute(app_route) => {
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(app_route.into()));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let url = self.props.url.clone();
        let value = self.props.tech.value.clone();
        let intent_value: Option<Intent>;
        let icon_value: Option<IconName>;
        if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == false
        {
            intent_value = Some(Intent::Warning);
            icon_value = None;
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            intent_value = Some(Intent::Success);
            icon_value = None;
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            intent_value = Some(Intent::Primary);
            icon_value = None;
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            intent_value = Some(Intent::Primary);
            icon_value = Some(IconName::Code);
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            intent_value = Some(Intent::Primary);
            icon_value = Some(IconName::People);
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            intent_value = Some(Intent::Warning);
            icon_value = Some(IconName::Star);
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            intent_value = Some(Intent::Warning);
            icon_value = Some(IconName::People);
        } else {
            intent_value = None;
            icon_value = None;
        }

        html! {
            <Tag
                class=classes!("pro-tag")
                interactive=true
                intent=intent_value
                right_icon=icon_value
                onclick=self.link.callback(move |_| Msg::GoToRoute(
                    AppRoute::ProfileHl(
                        url.to_string(),
                        value.to_string(),
                    )
                ))
            >
                {self.props.tech.value}
            </Tag>
        }
    }
}
