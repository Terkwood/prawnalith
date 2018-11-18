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

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_back(Msg::TokenPayload);
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
            <p>{format!("{:?}", self.auth_token)}</p>
        }
    }
}
fn firebase_login() {
    js! { firebase.auth().signInWithRedirect(new firebase.auth.GoogleAuthProvider()) }
}

fn firebase_conjure_token(token_callback: Callback<String>) {
    let callback = move |token: String| token_callback.emit(token);
    js! {
        // Yew magic
        var callback = @{callback};
        firebase.auth()
            .getRedirectResult()
            .then(function(result) {
                if (result.credential) {
                    callback(result.credential.accessToken);
                    callback.drop();
                }
            })
    }
}

fn firebase_on_auth_state_change(token_callback: Callback<String>) {
    let callback = move |token: String| token_callback.emit(token);
    js! {
        // Yew magic interpolation
        var callback = @{callback};
        firebase.auth()
            .onAuthStateChanged(function(user) {
                user.getIdToken(false).then(function(token){callback(result.credential.accessToken);
                    callback.drop();});
            } );
    }
}

// TODO next best steps for this frankenstein:
// https://github.com/DenisKolodin/yew/blob/master/examples/js_callback/src/lib.rs
