use std::time::Duration;
use std::time::SystemTime;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{form::FromForm, fs::TempFile, Data, FromFormField, Request, Response};

use serde::{Deserialize, Serialize};

use usequence::term::Season;

#[derive(FromForm)]
pub struct RequestBody<'f> {
    pub include_summer: bool,
    pub starting_semester: SeasonForm,
    pub starting_year: u32,
    pub max_courses_per_term: u32,
    pub courses: TempFile<'f>,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, FromFormField)]
pub enum SeasonForm {
    Summer,
    Fall,
    Winter,
}

impl From<SeasonForm> for Season {
    fn from(season: SeasonForm) -> Self {
        match season {
            SeasonForm::Summer => Season::Summer,
            SeasonForm::Fall => Season::Fall,
            SeasonForm::Winter => Season::Winter,
        }
    }
}

/**
 * The following code was written by Natalia Maximo, used with permission
 * Source: https://github.com/cryptaliagy/websvc-rs/blob/main/src/models.rs
 */

/// Fairing for timing requests.
#[derive(Default)]
pub struct RequestTimer {}

/// Value stored in request-local state.
#[derive(Copy, Clone)]
pub struct TimerStart(Option<SystemTime>);

/// Fairing for timing requests
#[rocket::async_trait]
impl Fairing for RequestTimer {
    fn info(&self) -> Info {
        Info {
            name: "Request Timer",
            kind: Kind::Request | Kind::Response,
        }
    }

    /// Stores the start time of the request in request-local state. This is used
    /// during the response fairing to calculate the request runtime.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // Store a `TimerStart` instead of directly storing a `SystemTime`
        // to ensure that this usage doesn't conflict with anything else
        // that might store a `SystemTime` in request-local cache.
        request.local_cache(|| TimerStart(Some(SystemTime::now())));
    }

    /// Calculates the duration of time that the request has taken and prints it to the logs.
    /// This will also inject that duration as a header on the response.
    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            let formatted = format_duration(duration);
            info!(
                "{method:<7} | {duration:>12} | {status} | \"{uri}\"",
                method = req.method(),
                uri = req.uri(),
                duration = formatted,
                status = res.status().code,
            );

            res.set_header(Header::new("X-Request-Duration", formatted));
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let micros = duration.subsec_micros() - millis * 1000;
    let nanos = duration.subsec_nanos() - micros * 1000;
    if secs > 0 {
        format!("{}.{:0>3} s", secs, millis)
    } else if millis > 0 {
        format!("{}.{:0>3} ms", millis, micros)
    } else {
        format!("{}.{:0>3} μs", micros, nanos)
    }
}
