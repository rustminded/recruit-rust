use yew::prelude::*;
use yewprint::{Text, H3};

pub struct PageNotFound {}

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PageNotFound {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="page_not_found">
                <H3>{"Page not found"}</H3>
                <Text>{"This profile does not seem to exist"}</Text>
                <Text>
                    {"You may find what you were looking for on our "}
                    <a href="/" >
                        {"homepage"}
                    </a>
                </Text>
            </div>
        }
    }
}
