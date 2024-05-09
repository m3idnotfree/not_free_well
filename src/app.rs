use axum::{extract::FromRef, routing::get, Router};
use tower_http::trace::TraceLayer;
use twitch_oauth_token::TwitchOauth;

use crate::{
    routes::{health_check, index, oauth},
    settings,
};

#[derive(Clone)]
struct AppState {
    twitch_oauth: TwitchOauth,
}

impl FromRef<AppState> for TwitchOauth {
    fn from_ref(input: &AppState) -> Self {
        input.twitch_oauth.clone()
    }
}

pub fn app() -> Router {
    let settings = settings().unwrap();

    let twitch = TwitchOauth::new(
        &settings.twitch.client_id,
        &settings.twitch.client_secret,
        "http://localhost:8874",
    );
    let app_state = AppState {
        twitch_oauth: twitch,
    };

    Router::new()
        .route("/", get(index))
        .route("/oauth", get(oauth))
        .route("/health_check", get(health_check))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
