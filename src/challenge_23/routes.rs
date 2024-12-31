use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use html_escape::encode_text;

const COLORS: [&str; 3] = ["red", "blue", "purple"];

fn escape_quotes(input: &str) -> String {
    input.replace('"', "&quot;")
}

pub(crate) async fn star() -> Html<&'static str> {
    Html("<div id=\"star\" class=\"lit\"></div>")
}

pub(crate) async fn get_present(Path(color): Path<String>) -> Result<Html<String>, StatusCode> {
    let color = escape_quotes(&*encode_text(&color));
    if let Some(current_index) = COLORS.iter().position(|&c| c == color) {
        let next_color = COLORS[(current_index + 1) % COLORS.len()];
        let next_color = encode_text(next_color);
        let html_response = format!(
            "<div class=\"present {}\" hx-get=\"/23/present/{}\" hx-swap=\"outerHTML\">\n                <div class=\"ribbon\"></div>\n                <div class=\"ribbon\"></div>\n                <div class=\"ribbon\"></div>\n                <div class=\"ribbon\"></div>\n            </div>",
            color, next_color
        );

        Ok(Html(html_response))
    } else {
        Err(StatusCode::IM_A_TEAPOT)
    }
}

pub(crate) async fn get_ornament(Path((state, n)): Path<(String, String)>) -> impl IntoResponse {
    let state = escape_quotes(&*encode_text(&state));
    let n = escape_quotes(&*encode_text(&n));
    let next_state = match state.to_string().as_str() {
        "on" => "off",
        "off" => "on",
        _ => return StatusCode::IM_A_TEAPOT.into_response(),
    };

    let mut css_class = String::new();
    if state == "on" {
        css_class.push_str(" on");
    }

    let response = format!(
        r#"<div class="ornament{css_class}" id="ornament{n}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{valid_state}/{n}" hx-swap="outerHTML"></div>"#,
        css_class = css_class,
        n = n,
        valid_state = next_state,
    );
    tracing::info!("{}", response);

    Html(response).into_response()
}

