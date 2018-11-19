#![recursion_limit = "128"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
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

#[derive(Default, PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct AuthToken(pub String);

pub struct Model {
    auth_token: Option<AuthToken>,
    hud: HeadsUpDisplay,
    link: ComponentLink<Model>,
}

pub enum Msg {
    SignIn,
    SignOut,
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
            Msg::SignOut => {
                firebase_logout();
                true
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
            <div style="font-size: 300%",>
             {
                if let Some(_auth_token) = &self.auth_token {
                    html! { <div>{ "🦐 Ready 🦐" }</div> }
                } else {
                    html! {
                        <button
                            class="button-xlarge pure-button pure-button-primary",
                            onclick=|_| Msg::SignIn,>
                        { "Sign In" }
                        </button>
                    }
                }
            }
            </div>
        }
    }
}

fn firebase_login() {
    js! { firebase.auth().signInWithRedirect(new firebase.auth.GoogleAuthProvider()) }
}

fn firebase_logout() {
    js! { firebase.auth().signOut(); }
}

#[derive(Deserialize, Serialize)]
struct AuthUser {
    uid: String,
    display_name: Option<String>,
    photo_url: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
    phone_number: Option<String>,
    api_key: Option<String>,
    sts_token_manager: Option<StsTokenManager>,
}

#[derive(Deserialize, Serialize)]
struct StsTokenManager {
    access_token: Option<AuthToken>,
}

js_serializable!(AuthUser);

// You may not need to trigger an additional fetch, if the Google AuthProvider
// js data structure already contains a token.
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
