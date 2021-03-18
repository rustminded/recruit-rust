use candidate::Job;
use yew::prelude::*;
use yewprint::{Intent, Tag, Text};

pub struct Jobs {
    props: JobsProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct JobsProps {
    pub jobs: &'static Job,
    pub highlighted_tech: Option<String>,
}

pub enum Msg {
    Click(String),
}

impl Component for Jobs {
    type Message = Msg;
    type Properties = JobsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Jobs { props, link }
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
        let jobs_tags = self
            .props
            .jobs
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
                                _ => Intent::Warning
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
