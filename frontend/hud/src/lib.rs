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

#[derive(Default, PartialEq, Eq, Clone)]
pub struct AuthToken(pub String);

pub struct Model {
    auth_token: Option<AuthToken>,
    baby: HeadsUpDisplay,
    link: ComponentLink<Model>,
}

pub enum Msg {
    SignIn,
    ConjureToken,
    TokenPayload(String),
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct Props {
    auth_token: Option<AuthToken>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            auth_token: None,
            baby: HeadsUpDisplay::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SignIn => {
                firebase_login();
                false
            }
            Msg::ConjureToken => {
                firebase_conjure_token(self.link.send_back(Msg::TokenPayload));
                false
            }
            Msg::TokenPayload(auth_token) => self.change(Self::Properties {
                auth_token: Some(AuthToken(auth_token)),
            }),
        }
    }

    fn change(&mut self, Self::Properties { auth_token }: Self::Properties) -> ShouldRender {
        if auth_token == self.auth_token {
            false
        } else {
            self.auth_token = auth_token;
            true
        }
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

        }
    }
}
fn firebase_login() {
    js! { firebase.auth().signInWithRedirect(new firebase.auth.GoogleAuthProvider()) }
}

fn firebase_conjure_token(token_callback: Callback<String>) {
    let callback = move |token: String| token_callback.emit(token);
    js! {
        firebase.auth()
            .getRedirectResult()
            .then(function(result) {
                if (result.credential) {
                    return result.credential.accessToken;
                } else {
                    return "";
                }
            })
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
