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
    use_fahrenheit: bool,
}

impl Model {
    fn view_tanks(&self) -> Html<Self> {
        let render = |tank: &Tank| {
            html! {
                <tr>
                    <td>{ tank.id }</td>
                    <td>{ tank.name.clone().unwrap_or("".to_owned()) }</td>
                    <td>
                    {
                      if self.use_fahrenheit {
                        tank.temp_f.map(|t| format!("{}℉", t))
                      } else {
                        tank.temp_c.map(|t| format!("{}℃", t))
                      }.unwrap_or("".to_owned())
                    }
                    </td>
                    <td>{ tank.ph.map(|ph| format!("{}",ph)).unwrap_or("".to_owned()) }</td>
                </tr>
            }
        };

        html! {  { for self.tanks.0.iter().map(render) } }
    }
}

pub enum Msg {
    SignIn,
    SignOut,
    TokenPayload(String),
    Tick,
    TanksFetched(Result<Vec<Tank>, Error>),
    ToggleTempUnits,
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
            use_fahrenheit: true,
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
            Msg::ToggleTempUnits => {
                self.use_fahrenheit = !self.use_fahrenheit;
                true
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
                    <h1>{ "Prawnalith" }</h1>
                    <h2>{ "🦐 A tank for the ages 🦐" }</h2>
                </div>
            { if let Some(_auth_token) = &self.auth_token {
                html! {
                    <div class="content",>
                        <h2 class="content-subhead",>{ "Tank Status" }</h2>
                        <table class="pure-table pure-table-horizontal",>
                            <thead>
                                <tr>
                                    <th>{"#"}</th>
                                    <th>{"Name"}</th>
                                    <th>{"Temp"}</th>
                                    <th>{"pH"}</th>
                                </tr>
                            </thead>
                            <tbody>
                            { self.view_tanks() }
                            </tbody>
                        </table>
                        <br/>
                        <div>
                            <input class="tgl tgl-friend", id="temp-units", type="checkbox", checked=self.use_fahrenheit, onclick=|_| Msg::ToggleTempUnits,/>
                            <label class="tgl-btn", data-tg-off="Temp ℃", data-tg-on="Temp ℉", for="temp-units",></label>
                        </div>
                        <div>
                            <p><a href="https://github.com/Terkwood/prawnalith",>{ "Github repo" }</a></p>
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
/// It's stored inside config.js, in the static dir.  Enjoy!
fn js_pond_host() -> String {
    let v: Value = js! {
            return pond_host;
    };
    v.try_into().expect("can't extract data host")
}
