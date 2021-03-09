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
    pub url: String,
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
        if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == false
        {
            html! {
                <Tag
                    class=classes!("pro-tag")
                    intent=Intent::Warning
                    onclick=self.link.callback(|_| Msg::GoToRoute(AppRoute::ProfileHl(self.props.url, self.props.tech.value.to_string())))
                >
                    {self.props.tech.value}
                </Tag>
            }
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            html! {
                <Tag
                    class=classes!("pub-tag")
                    intent=Intent::Success
                >
                    {self.props.tech.value}
                </Tag>
            }
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            html! {
                <Tag
                    class=classes!("asked-tag")
                    intent=Intent::Primary
                >
                    {self.props.tech.value}
                </Tag>
            }
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == false
            && self.props.tech.is_asked == true
        {
            html! {
                <Tag
                    class=classes!("asked-pro-tag")
                    right_icon=IconName::Code
                    intent=Intent::Primary
                >
                    {self.props.tech.value}
                </Tag>
            }
        } else if self.props.tech.is_professional == false
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            html! {
                <Tag
                    class=classes!("asked-pub-tag")
                    intent=Intent::Primary
                    right_icon=IconName::People
                >
                    {self.props.tech.value}
                </Tag>
            }
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == true
        {
            html! {
                <Tag
                    class=classes!("three-flag-tag")
                    intent=Intent::Warning
                    right_icon=IconName::Star
                >
                    {self.props.tech.value}
                </Tag>
            }
        } else if self.props.tech.is_professional == true
            && self.props.tech.is_public == true
            && self.props.tech.is_asked == false
        {
            html! {
                <Tag
                    class=classes!("pro-pub-tag")
                    intent=Intent::Warning
                    right_icon=IconName::People
                >
                    {self.props.tech.value}
                </Tag>
            }
        } else {
            html! {
                <Tag
                    class=classes!("no-flag-tag")
                >
                    {self.props.tech.value}
                </Tag>
            }
        }
    }
}
