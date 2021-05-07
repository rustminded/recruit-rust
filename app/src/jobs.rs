use candidate::Job;
use yew::prelude::*;
use yewprint::{Intent, Tag, Text};

pub struct Jobs {
    props: JobsProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct JobsProps {
    pub job: &'static Job,
    pub highlighted_tech: Option<String>,
    pub onclick: Callback<String>,
}

pub enum Msg {
    Highlight(String),
}

impl Component for Jobs {
    type Message = Msg;
    type Properties = JobsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Jobs { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Highlight(value) => self.props.onclick.emit(value),
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
        let jobs_tags = self
            .props
            .job
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
                        onclick=self.link.callback(move |_| Msg::Highlight(x.to_string()))
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
                    <a class="jobs-header-link" href={self.props.job.website}>
                        {self.props.job.company}
                    </a>
                    <Text class=classes!("jobs-header-separator")>
                        {"|"}
                    </Text>
                    <Text class=classes!("jobs-header-period")>
                        {self.props.job.period}
                    </Text>
                </div>
                <Text class=classes!("jobs-description")>
                    {self.props.job.description}
                </Text>
                <div class="separator">
                </div>
            </div>
        }
    }
}
