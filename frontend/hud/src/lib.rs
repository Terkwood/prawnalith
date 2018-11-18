#![recursion_limit = "128"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct HeadsUpDisplay {}

impl HeadsUpDisplay {
    pub fn new() -> HeadsUpDisplay {
        HeadsUpDisplay {}
    }

    pub fn show(&self) -> &str {
        "Tank 1 blah blah Tank 2 blah blah"
    }

    pub fn update(&mut self) {}
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct AuthToken(pub String);

pub struct Model {
    auth_token: Option<AuthToken>,
    hud: HeadsUpDisplay,
    link: ComponentLink<Model>,
}

pub enum Msg {
    SignIn,
    TokenPayload(String),
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct Props {
    auth_token: Option<AuthToken>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        firebase_on_auth_state_change(link.send_back(Msg::TokenPayload));
        Model {
            auth_token: None,
            hud: HeadsUpDisplay::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SignIn => {
                firebase_login();
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
            <div>
                <button class="button-xlarge pure-button", onclick=|_| Msg::SignIn,>{ "Sign In" }</button>
                <br/>
                <div>{if let Some(_auth_token) = &self.auth_token { "🦐 Ready 🦐" } else { "" }}</div>
            </div>
        }
    }
}

fn firebase_login() {
    js! { firebase.auth().signInWithRedirect(new firebase.auth.GoogleAuthProvider()) }
}

fn firebase_on_auth_state_change(token_callback: Callback<String>) {
    let callback = move |token: String| token_callback.emit(token);
    js! {
        // Yew magic interpolation
        var callback = @{callback};
        firebase.auth()
            .onAuthStateChanged(function(user) {
                user.getIdToken(false).then(
                    function(token){
                        callback(token);
                        callback.drop();
                    });
            } );
    }
}

// TODO next best steps for this frankenstein:
// https://github.com/DenisKolodin/yew/blob/master/examples/js_callback/src/lib.rs
