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
        let tech = &self.props.tech;
        let value = tech.value.clone();

        let (intent_value, icon_value): (Option<Intent>, Option<IconName>) = {
            if tech.is_professional && tech.is_public && tech.is_asked {
                (Some(Intent::Warning), Some(IconName::Star))
            } else if tech.is_professional && tech.is_public {
                (Some(Intent::Warning), Some(IconName::People))
            } else if tech.is_professional && tech.is_asked {
                (Some(Intent::Primary), Some(IconName::Code))
            } else if tech.is_public && tech.is_asked {
                (Some(Intent::Primary), Some(IconName::People))
            } else if tech.is_professional {
                (Some(Intent::Warning), None)
            } else if tech.is_public {
                (Some(Intent::Success), None)
            } else if tech.is_asked {
                (Some(Intent::Primary), None)
            } else {
                (None, None)
            }
        };

        html! {
            <a href=format!("{}#{}", url, value)>
                <Tag
                    class=classes!("tech-tag")
                    interactive=true
                    intent=intent_value
                    right_icon=icon_value
                >
                    {value.clone()}
                </Tag>
            </a>
        }
    }
}
