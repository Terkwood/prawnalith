#![recursion_limit = "256"]
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use std::time::Duration;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

pub struct HeadsUpDisplay {}

impl HeadsUpDisplay {
    pub fn new() -> HeadsUpDisplay {
        HeadsUpDisplay {}
    }

    pub fn show(&self) -> &str {
        "They're a bit hungry"
    }

    pub fn update(&mut self) {}
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct AuthToken(pub String);

pub struct Model {
    auth_token: Option<AuthToken>,
    hud: HeadsUpDisplay,
    link: ComponentLink<Model>,
    interval: IntervalService,
    callback_tick: Callback<()>,
    job: Option<Box<Task>>,
    console: ConsoleService,
}

pub enum Msg {
    SignIn,
    SignOut,
    TokenPayload(String),
    Tick,
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

        let mut interval = IntervalService::new();
        let callback_tick = link.send_back(|_| Msg::Tick);
        let handle = interval.spawn(Duration::from_secs(10), callback_tick.clone().into());

        Model {
            auth_token: None,
            hud: HeadsUpDisplay::new(),
            link,
            interval,
            callback_tick,
            job: Some(Box::new(handle)),
            console: ConsoleService::new(),
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
            Msg::Tick => {
                self.console.count_named("Tick");
                false
            }
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

/// Render an HTML model of our information.
/// The layout is liberally lifted from https://purecss.io/layouts/side-menu/#
/// Thanks, PureCSS!
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
        <div id="layout",>
            // Menu toggle
            <a href="#menu", id="menuLink", class="menu-link",>
                <span></span>
            </a>

            <div id="menu",>
                <div class="pure-menu",>
                    <ul class="pure-menu-list",>
                        <li class="pure-menu-item centered-menu-item",>
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
            </div>
            <div id="main",>
                        <div class="header",>
                            <h1>{ "🦐 Prawnalith 🦐" }</h1>
                            <h2>{ "A tank for the ages" }</h2>
                        </div>
            { if let Some(_auth_token) = &self.auth_token {
                html! {
                    <div class="content",>
                        <h2 class="content-subhead",>{ "Let's check on the status of the prawns" }</h2>
                        <p>
                        { self.hud.show() }
                        </p>

                        <h2 class="content-subhead",>{ "There are things which exist" }</h2>
                        <p>
                        { "And some other text" }
                        </p>

                        <div class="pure-g",>
                            <div class="pure-u-1-4",>
                                <img class="pure-img-responsive", src="http://farm3.staticflickr.com/2875/9069037713_1752f5daeb.jpg", alt="Peyto Lake",></img>
                            </div>
                            <div class="pure-u-1-4",>
                                <img class="pure-img-responsive", src="http://farm3.staticflickr.com/2813/9069585985_80da8db54f.jpg", alt="Train",></img>
                            </div>
                            <div class="pure-u-1-4",>
                                <img class="pure-img-responsive", src="http://farm6.staticflickr.com/5456/9121446012_c1640e42d0.jpg", alt="T-Shirt Store",></img>
                            </div>
                            <div class="pure-u-1-4",>
                                <img class="pure-img-responsive", src="http://farm8.staticflickr.com/7357/9086701425_fda3024927.jpg", alt="Mountain",></img>
                            </div>
                        </div>

                        <h2 class="content-subhead",>{ "Try Resizing your Browser" }</h2>
                        <p>
                            { "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                        </p>
                    </div>
                    }
                } else {
                    html!{ <br/> }
                }

            }
            </div>
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
