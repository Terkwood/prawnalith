#![recursion_limit = "256"]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

mod pond;

use crate::pond::PondService;
use failure::Error;
use std::time::Duration;
use stdweb::unstable::TryInto;
use stdweb::Value;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

/// A struct to hold data returned by the HTTP request
/// for tanks' temp & ph info.
#[derive(Debug, Deserialize)]
pub struct Tank {
    pub id: u16,
    pub name: Option<String>,
    pub temp_f: Option<f32>,
    pub temp_c: Option<f32>,
    pub temp_update_time: Option<u64>,
    pub temp_update_count: Option<u32>,

    pub ph: Option<f32>,
    pub ph_mv: Option<f32>,
    pub ph_update_time: Option<u64>,
    pub ph_update_count: Option<u32>,
}

pub struct Tanks(pub Vec<Tank>);

impl Tanks {
    pub fn new() -> Tanks {
        Tanks(vec![])
    }

    pub fn show(&self) -> String {
        let mut r = String::new();
        for tank in &self.0 {
            r.push_str(&format!("{:?}", tank))
        }
        r
    }

    pub fn update(&mut self) {}
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
pub struct AuthToken(pub String);

/// `auth_token` lets us know whom we're dealing with
/// `tanks` is the current set of temp & ph data for all tanks in the system, the payload we're interested in showing to the end user
/// `link` is used by the javascript.  rust compiler will tell you that you can get rid of it.  DON'T BELIEVE ITS LIES.
/// `callback_tanks` is invoked when the HTTP request to get recent data is completed
/// `interval` sends a Tick message every so often, triggering an HTTP fetch of the tank data
pub struct Model {
    auth_token: Option<AuthToken>,
    tanks: Tanks,
    _link: ComponentLink<Model>,
    pond: PondService,
    callback_tanks: Callback<Result<Vec<Tank>, Error>>,
    _interval: IntervalService,
    _callback_tick: Callback<()>,
    _interval_job: Option<Box<Task>>,
    fetch_job: Option<Box<Task>>,
    console: ConsoleService,
}

pub enum Msg {
    SignIn,
    SignOut,
    TokenPayload(String),
    Tick,
    TanksFetched(Result<Vec<Tank>, Error>),
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

        let mut _interval = IntervalService::new();
        let _callback_tick = link.send_back(|_| Msg::Tick);
        let handle = _interval.spawn(Duration::from_secs(10), _callback_tick.clone().into());

        let callback_tanks = link.send_back(Msg::TanksFetched);

        Model {
            auth_token: None,
            tanks: Tanks::new(),
            _link: link,
            pond: PondService::new(&js_pond_host()),
            callback_tanks,
            _interval,
            _callback_tick,
            _interval_job: Some(Box::new(handle)),
            fetch_job: None,
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
            // Fetch the tanks
            Msg::Tick => {
                if let Some(token) = &self.auth_token {
                    let task = self.pond.tanks(token.clone(), self.callback_tanks.clone());
                    self.fetch_job = Some(Box::new(task));
                }
                false
            }
            Msg::TanksFetched(Ok(tanks)) => {
                self.tanks = Tanks(tanks);
                true
            }
            Msg::TanksFetched(Err(_e)) => {
                self.console.error("Failed to fetch data");
                false
            }
        }
    }

    fn change(&mut self, Self::Properties { auth_token }: Self::Properties) -> ShouldRender {
        if auth_token == self.auth_token {
            false
        } else {
            self.auth_token = auth_token;
            if let Some(token) = &self.auth_token {
                // Immediately fetch, so that the user isn't waiting around for
                // the next tick from IntervalService
                let task = self.pond.tanks(token.clone(), self.callback_tanks.clone());
                self.fetch_job = Some(Box::new(task));
            }
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
                        { self.tanks.show() }
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

/// Get the hostname for the data broker that we're going to talk to.
/// It's stored magically inside javascript!  Enjoy!
fn js_pond_host() -> String {
    let v: Value = js! {
            return pond_host;
    };
    v.try_into().expect("can't extract data host")
}
