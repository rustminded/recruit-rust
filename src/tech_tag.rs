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
        let value_tuple: (Option<Intent>, Option<IconName>);
        let value_tuple = if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == false
        {
            (value_tuple.0 = Some(Intent::Warning), value_tuple.1 = None)
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            (value_tuple.0 = Some(Intent::Success), value_tuple.1 = None)
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            (value_tuple.0 = Some(Intent::Primary), value_tuple.1 = None)
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            (
                value_tuple.0 = Some(Intent::Primary),
                value_tuple.1 = Some(IconName::Code),
            )
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            (
                value_tuple.0 = Some(Intent::Primary),
                value_tuple.1 = Some(IconName::People),
            )
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            (
                value_tuple.0 = Some(Intent::Warning),
                value_tuple.1 = Some(IconName::Star),
            )
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            (
                value_tuple.0 = Some(Intent::Warning),
                value_tuple.1 = Some(IconName::People),
            )
        } else {
            (value_tuple.0 = None, value_tuple.1 = None)
        };
        let (intent_value, icon_value) = value_tuple;

        html! {
            <Tag
                class=classes!("pro-tag")
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
