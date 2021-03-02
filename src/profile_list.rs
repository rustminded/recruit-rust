use candidate::Candidate;
use yew::prelude::*;

pub struct ProfileList {
    props: ProfileListProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileListProps {
    pub candidate: &'static Candidate,
}

impl Component for ProfileList {
    type Message = ();
    type Properties = ProfileListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ProfileList { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <p>{"Hello, world!"}</p>
        }
    }
}
