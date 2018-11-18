#![recursion_limit = "1024"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use stdweb::Value;
use yew::prelude::*;

pub struct HeadsUpDisplay {}

impl HeadsUpDisplay {
    pub fn new() -> HeadsUpDisplay {
        HeadsUpDisplay {}
    }

    pub fn show(&self) -> &str {
        "hi"
    }

    pub fn update(&mut self) {}
}

pub struct AuthToken(pub String);

pub struct Model {
    auth_token: Option<AuthToken>,
    baby: HeadsUpDisplay,
}

pub enum Msg {
    SignIn,
    ConjureToken,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            auth_token: None,
            baby: HeadsUpDisplay::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SignIn => firebase_login(),
            Msg::ConjureToken => firebase_conjure_token(),
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>{"Hello"}</div>
            <br/>
            <button class="pure-button", onclick=|_| Msg::SignIn,>{ "Sign In" }</button>
            <br/>
            <button class="pure-button", onclick=|_| Msg::ConjureToken,>{ "Conjure Token" }</button>

            
                /*if let Some(t) = &self.auth_token {
                    format!("Token: {}", t.0)
                } else {
                    "No Token for you!".to_string()
                }*/
            
        }
    }
}
fn firebase_login() {
    js! { firebase.auth().signInWithRedirect(new firebase.auth.GoogleAuthProvider()) }
}

fn firebase_conjure_token() {
    js! {
        firebase.auth()
            .getRedirectResult()
            .then(function(result) { if (result.credential) { current_token = result.credential.accessToken; } })
    }
}

fn firebase_retrieve_token() -> Option<String> {
    let v: Value = js! {
        if (current_token === undefined) {
            current_token = "";
        }
        return current_token;
    };
    let v: String = stdweb::unstable::TryInto::try_into(v).expect("can't extract token");
    if v == "" {
        None
    } else {
        Some(v)
    }
}

// TODO next best steps for this frankenstein:
// https://github.com/DenisKolodin/yew/blob/master/examples/js_callback/src/lib.rs
