#[macro_use]
extern crate lazy_static;
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

pub struct Beluga {
    line: usize,
}

impl Beluga {
    pub fn new() -> Beluga {
        Beluga { line: 0 }
    }

    pub fn sing(&self) -> &str {
        song[self.line]
    }

    pub fn next(&mut self) {
        self.line = (self.line + 1) % song.len()
    }
}

pub struct Model {
    baby: Beluga,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            baby: Beluga::new(),
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
            <div>
                <button class="pure-button pure-button-primary", onclick=|_| Msg::Click,>{ "Click" }</button>
                <p>
                { self.baby.sing() }
                </p>
            </div>
        }
    }
}
