use candidate::Contribution;
use yew::prelude::*;
use yewprint::{Intent, Tag, Text};

pub struct Contributions {
    props: ContributionProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ContributionProps {
    pub contributions: &'static Contribution,
    pub highlighted_tech: Option<String>,
}

pub enum Msg {
    Click(String),
}

impl Component for Contributions {
    type Message = Msg;
    type Properties = ContributionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Contributions { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(value) => {
                if let Some(highlighted_tech) = self.props.highlighted_tech.as_mut() {
                    highlighted_tech.clear();
                    highlighted_tech.push_str(&value);
                } else {
                    let highlighted_tech = String::from(&value);
                    self.props.highlighted_tech = Some(highlighted_tech);
                }
            }
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
        let contrib_tags = self
            .props
            .contributions
            .techs
            .iter()
            .map(|x| {
                html! {
                    <Tag
                        class=classes!("tag")
                        interactive=true
                        intent={
                            match self.props.highlighted_tech.as_ref() {
                                Some(highlighted_tech) if highlighted_tech == x => Intent::Danger,
                                _ => Intent::Success
                            }
                        }
                        onclick=self.link.callback(move |_| Msg::Click(x.to_string()))
                    >
                        {x}
                    </Tag>
                }
            })
            .collect::<Html>();

        html! {
            <div class="candidate-alone-contribution">
                <div class="candidate-tag">
                    {contrib_tags}
                </div>
                    <a class="contribution-link" href={self.props.contributions.website}>
                        {self.props.contributions.project}
                    </a>
                    <Text class=classes!("contribution-description")>
                        {self.props.contributions.description}
                    </Text>
                <div class="separator">
                </div>
            </div>
        }
    }
}
