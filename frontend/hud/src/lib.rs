#![recursion_limit = "128"]
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
            <div>
                <div class="pure-menu", width="10em",>
                    <span class="pure-menu-heading",>{ "🦐 Prawnalith 🦐" }</span>
                    <ul class="pure-menu-list",>
                        <li class="pure-menu-item",>
                        {
                            if let Some(_auth_token) = &self.auth_token {
                                html! {
                                    <button
                                        class="pure-button",
                                        onclick=|_| Msg::SignOut,>
                                    { "Sign Out" }
                                    </button>
                                }
                            } else {
                                html! {
                                    <button
                                        class="pure-button pure-button-primary",
                                        onclick=|_| Msg::SignIn,>
                                    { "Sign In" }
                                    </button>
                                }
                            }
                        }
                        </li>
                    </ul>
                </div>
                <br/>
                <div>{ if let Some(_auth_token) = &self.auth_token { "🦐 Ready 🦐" } else { "" } }</div>
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

// You may not need to trigger an additional fetch, if the Google AuthProvider
// js data structure already contains a token.
fn firebase_on_auth_state_change(token_callback: Callback<String>) {
    let callback = move |token: String| token_callback.emit(token);
    js! {
        // Yew magic interpolation
        var callback = @{callback};
        firebase.auth()
            .onAuthStateChanged(function(user) {
                var user_json = user.toJSON();
                if (user_json.stsTokenManager && user_json.stsTokenManager.accessToken) {
                    callback(user_json.stsTokenManager.accessToken);
                    callback.drop();
                } else {
                    user.getIdToken(false).then(
                        function(token){
                            callback(token);
                            callback.drop();
                        });
                    }
            } );
    }
}
