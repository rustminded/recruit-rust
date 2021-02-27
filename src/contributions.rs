use candidate::Contribution;
use yew::prelude::*;
use yewprint::{Tag, Text};

pub struct Contributions {
    props: Props,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub contributions: &'static Contribution,
}

impl Component for Contributions {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Contributions { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let contrib_tags = self
            .props
            .contributions
            .tech
            .iter()
            .map(|x| {
                html! {
                    <Tag class=classes!("tag")>
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
                <div class="contribution-link">
                    <a href={self.props.contributions.website}>
                        {self.props.contributions.project}
                    </a>
                </div>
                <div class="contribution-description">
                    <Text>{self.props.contributions.description}</Text>
                </div>
                <div class="separator">
                </div>
            </div>
        }
    }
}
