use candidate::Candidate;
use yew::prelude::*;
use yewprint::Card;

pub struct Profile {
    props: Props,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub candidate: Candidate,
}

impl Component for Profile {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Profile { props }
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
                <Card>
                    <p>{self.props.candidate.name}</p>
                </Card>
            </div>
        }
    }
}
