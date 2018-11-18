#![recursion_limit="256"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;


use yew::prelude::*;

lazy_static! {
    static ref song: Vec<&'static str> = vec![
        "Baby Beluga in the deep blue sea 🌊",
        "swims so wild and you swim so free 🆓",
        "the waves roll in and the waves roll out 🌊",
        "see the water squirt out your spout 🐳",
        "🐳 BAAAAABY 🐳 BELUUUUUGA 🐳",
        "🐳 OH 🐳 BAAAABY 🐳 BELUUUUGA 🐳",
        "is the water warm 🔥",
        "is your mom home 👩‍👦",
        "🌈 with you 🌈 so happy 🌈",
    ];
}

pub struct HeadsUpDisplay {
    line: usize,
}

impl HeadsUpDisplay {
    pub fn new() -> HeadsUpDisplay {
        HeadsUpDisplay { line: 0 }
    }

    pub fn sing(&self) -> &str {
        song[self.line]
    }

    pub fn next(&mut self) {
        self.line = (self.line + 1) % song.len()
    }
}

pub struct Model {
    baby: HeadsUpDisplay,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {

        Model {
            baby: HeadsUpDisplay::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => self.baby.next(),
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="container",>
            <h3>{"Prawnalith"}</h3>
            <div id="loading",>{"Loading..."}</div>
            <div id="loaded", class="hidden",>
                <div id="main",>
                <div id="user-signed-in", class="hidden",>
                    <div id="user-info",>
                    <div id="photo-container",>
                        <img id="photo",/>
                    </div>
                    <div id="name",></div>
                    <div id="email",></div>
                    <div id="phone",></div>
                    <div id="is-new-user",></div>
                    <div class="clearfix",></div>
                    </div>
                    <p>
                    <button id="sign-out",>{"Sign Out"}</button>
                    <button id="delete-account",>{"Delete account"}</button>
                    </p>
                </div>
                <div id="user-signed-out", class="hidden",>
                    <h4>{"You are signed out."}</h4>
                    <div id="firebaseui-spa",>
                    <div id="firebaseui-container",></div>
                    </div>
                </div>
                </div>
            </div>
        </div>
        }
    }
}
