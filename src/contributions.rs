use candidate::Contribution;
use yew::prelude::*;
use yewprint::{Intent, Tag, Text};

pub struct Contributions {
    props: ContributionProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ContributionProps {
    pub contribution: &'static Contribution,
    pub highlighted_tech: Option<String>,
    pub onclick: Callback<String>,
}

pub enum Msg {
    HighLight(String),
}

impl Component for Contributions {
    type Message = Msg;
    type Properties = ContributionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Contributions { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HighLight(value) => self.props.onclick.emit(value),
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
            .contribution
            .techs
            .iter()
            .map(|x| {
                html! {
                    <Tag
                        class=classes!("tag")
                        intent={
                            match self.props.highlighted_tech.as_ref() {
                                Some(highlighted_tech) if highlighted_tech == x => Intent::Danger,
                                _ => Intent::Success
                            }
                        }
                        onclick=self.link.callback(move |_| Msg::HighLight(x.to_string()))
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
                    <a class="contribution-link" href={self.props.contribution.website}>
                        {self.props.contribution.project}
                    </a>
                    <Text class=classes!("contribution-description")>
                        {self.props.contribution.description}
                    </Text>
                <div class="separator">
                </div>
            </div>
        }
    }
}
