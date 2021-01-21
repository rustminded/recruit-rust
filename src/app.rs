use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Text, H1};

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="app-root bp3-dark">
                <div class="search-field">
                    <InputGroup
                        round=true
                        placeholder="Search..."
                        right_element=html! {
                            <Button
                                icon=IconName::Search
                                minimal=true
                            />
                        }
                    />
                </div>
                <div class="app-header">
                    <H1>{"Welcome on Recruit-Rust.dev!"}</H1>
                    <Text>{"The place to be hired as an awesome Rustacean"}</Text>
                </div>
                <div class="app-content" role="main">
                </div>
            </div>
        }
    }
}
