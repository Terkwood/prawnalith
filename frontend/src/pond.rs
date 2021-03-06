use crate::Tank;
use failure::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

// Thanks to https://github.com/DenisKolodin/yew/blob/master/examples/npm_and_rest/src/gravatar.rs

#[derive(Default)]
pub struct PondService {
    web: FetchService,
    host: String,
}

/// Fetch from the "pond" service, which will return our tank data
impl PondService {
    pub fn new(host: &str) -> Self {
        Self {
            web: FetchService::new(),
            host: host.to_string(),
        }
    }

    pub fn tanks(
        &mut self,
        token: crate::AuthToken,
        callback: Callback<Result<Vec<Tank>, Error>>,
    ) -> FetchTask {
        let url = format!("https://{}/tanks", self.host);

        let handler = move |response: Response<Json<Result<Vec<Tank>, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                callback.emit(Err(format_err!(
                    "{}: error fetching tank status",
                    meta.status
                )))
            }
        };

        let token_header = format!("Bearer {}", token.0);

        // Origin header required for CORS
        let request = Request::get(url.as_str())
            .header("Authorization", token_header)
            .header("Origin", "prawn.farm")
            .header("Accept", "application/json")
            .body(Nothing)
            .unwrap();
        self.web.fetch(request, handler.into())
    }
}
