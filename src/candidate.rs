use candidate::Candidate;
use yew::prelude::*;

pub struct CandidateComponent {
    props: Props,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub candidate: Candidate,
}

impl Component for CandidateComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CandidateComponent { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="candidate">
                <h1>{self.props.candidate.name}</h1>
                <div>{self.props.candidate.bio}</div>
            </div>
        }
    }
}
