use candidate::Job;
use yew::prelude::*;
use yewprint::{Tag, Text};

pub struct Jobs {
    props: Props,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub jobs: &'static Job,
}

impl Component for Jobs {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Jobs { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let jobs_tags = self
            .props
            .jobs
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
            <div class="candidate-alone-job">
                <div class="candidate-tag">
                    {jobs_tags}
                </div>
                <div class="jobs-header">
                    <div class="jobs-header-link">
                        <a href={self.props.jobs.website}>
                            {self.props.jobs.company}
                        </a>
                    </div>
                    <div class="jobs-header-separator">
                        <Text>{"|"}</Text>
                    </div>
                    <div class="jobs-header-period">
                        <Text>{self.props.jobs.period}</Text>
                    </div>
                </div>
                <div class="jobs-description">
                    <Text>{self.props.jobs.description}</Text>
                </div>
                <div class="separator">
                </div>
            </div>
        }
    }
}
