use candidate::Job;
use yew::prelude::*;
use yewprint::{Intent, Tag, Text};

pub struct Jobs {
    props: JobsProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct JobsProps {
    pub jobs: &'static Job,
    pub highlighted_tech: Option<String>,
}

impl Component for Jobs {
    type Message = ();
    type Properties = JobsProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Jobs { props }
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
        let jobs_tags = self
            .props
            .jobs
            .techs
            .iter()
            .map(|x| {
                html! {
                    <Tag
                        class=classes!("tag")
                        intent={
                            match self.props.highlighted_tech.as_ref() {
                                Some(highlighted_tech) if highlighted_tech == x => Intent::Danger,
                                _ => Intent::Warning
                            }
                        }
                    >
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
                    <a class="jobs-header-link" href={self.props.jobs.website}>
                        {self.props.jobs.company}
                    </a>
                    <Text class=classes!("jobs-header-separator")>
                        {"|"}
                    </Text>
                    <Text class=classes!("jobs-header-period")>
                        {self.props.jobs.period}
                    </Text>
                </div>
                <Text class=classes!("jobs-description")>
                    {self.props.jobs.description}
                </Text>
                <div class="separator">
                </div>
            </div>
        }
    }
}
