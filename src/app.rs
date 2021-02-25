use crate::profile::Profile;
use candidate::Candidate;
use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Text, H1, H2};

pub struct App {
    candidates: Vec<&'static Candidate>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let candidates = vec![yozhgoor::candidate()];
        crate::log!("{:?}", candidates);
        App { candidates }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let profile_list = self
            .candidates
            .iter()
            .map(|x| {
                html! {
                    <Profile candidate={x} />
                }
            })
            .collect::<Html>();

        html! {
            <div class="app-root bp3-dark">
                <div class="app-header">
                    <div class="app-title">
                    <H1>{"Welcome on Recruit-Rust.dev!"}</H1>
                    </div>
                    <div class="app-search-field">
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
                </div>
                <div class="app-intro">
                    <Text>{"The place to be hired as an awesome Rustacean"}</Text>
                </div>
                <div class="app-content" role="main">
                    <div class="profile-list">
                        <H2>{"Discover the community"}</H2>
                        {profile_list}
                    </div>
                </div>
            </div>
        }
    }
}
