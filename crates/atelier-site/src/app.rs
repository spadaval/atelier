//! The web app in `web/`, embedded in the binary (atelier-yn69).
//!
//! `build.rs` builds it into `web/dist` for a release build; a debug build
//! embeds `web/dist` only if it exists. A path naming an embedded file gets
//! that file and its content type, a missing path under `assets/` is a 404,
//! and every other path gets `index.html`, so the app's own router serves
//! deep links.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/dist"]
#[allow_missing = true]
struct Dist;

/// The page served in place of the app when this binary was built without
/// it, which only a debug build can be.
const NOT_BUILT: &str = "<!doctype html>\n<meta charset=\"utf-8\">\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
<title>Atelier: app not built</title>\n\
<p>This <code>atelier</code> was built without its web app; the API under \
<a href=\"/api/repos\"><code>/api/</code></a> works. Build the app with \
<code>pnpm install --frozen-lockfile &amp;&amp; pnpm build</code> in \
<code>crates/atelier-site/web</code>, then build <code>atelier</code> again.</p>\n";

/// Where the app's files come from: the embedded build, or a fixed set in
/// tests.
#[derive(Clone)]
pub struct Assets(Arc<dyn Fn(&str) -> Option<Cow<'static, [u8]>> + Send + Sync>);

impl Assets {
    /// The app embedded in this binary.
    pub fn embedded() -> Self {
        Self(Arc::new(|path| Dist::get(path).map(|file| file.data)))
    }

    /// `files`, keyed by path without a leading `/`.
    pub fn from_files(files: HashMap<String, Vec<u8>>) -> Self {
        Self(Arc::new(move |path| {
            files.get(path).map(|data| Cow::Owned(data.clone()))
        }))
    }

    fn get(&self, path: &str) -> Option<Cow<'static, [u8]>> {
        (self.0)(path)
    }

    /// The answer to `GET path` for any path outside `/api/`.
    pub fn respond(&self, path: &str) -> Response {
        let path = path.trim_start_matches('/');
        if !path.is_empty() {
            if let Some(data) = self.get(path) {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                // Vite names every file under assets/ by its content hash.
                let cache = if path.starts_with("assets/") {
                    "public, max-age=31536000, immutable"
                } else {
                    "no-cache"
                };
                return reply(StatusCode::OK, mime.as_ref(), cache, data);
            }
            if path.starts_with("assets/") {
                return reply(
                    StatusCode::NOT_FOUND,
                    "text/plain; charset=utf-8",
                    "no-cache",
                    Cow::Owned(format!("no asset /{path}\n").into_bytes()),
                );
            }
        }
        let index = self
            .get("index.html")
            .unwrap_or(Cow::Borrowed(NOT_BUILT.as_bytes()));
        reply(
            StatusCode::OK,
            "text/html; charset=utf-8",
            "no-cache",
            index,
        )
    }
}

fn reply(
    status: StatusCode,
    mime: &str,
    cache: &'static str,
    data: Cow<'static, [u8]>,
) -> Response {
    let mut response = Response::new(Body::from(data.into_owned()));
    *response.status_mut() = status;
    let headers = response.headers_mut();
    if let Ok(value) = HeaderValue::from_str(mime) {
        headers.insert(header::CONTENT_TYPE, value);
    }
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static(cache));
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake() -> Assets {
        Assets::from_files(HashMap::from([
            (
                "index.html".to_owned(),
                b"<script src=\"/assets/index-abc.js\"></script>".to_vec(),
            ),
            ("assets/index-abc.js".to_owned(), b"console.log(1)".to_vec()),
            ("assets/index-abc.css".to_owned(), b"body{}".to_vec()),
            ("favicon.svg".to_owned(), b"<svg/>".to_vec()),
        ]))
    }

    fn content_type(response: &Response) -> &str {
        response.headers()[header::CONTENT_TYPE].to_str().unwrap()
    }

    #[test]
    fn an_embedded_file_is_served_with_its_content_type() {
        let assets = fake();
        for (path, mime) in [
            ("/assets/index-abc.js", "text/javascript"),
            ("/assets/index-abc.css", "text/css"),
            ("/favicon.svg", "image/svg+xml"),
        ] {
            let response = assets.respond(path);
            assert_eq!(response.status(), StatusCode::OK, "{path}");
            assert_eq!(content_type(&response), mime, "{path}");
        }
    }

    #[test]
    fn a_missing_asset_is_a_404_not_the_index() {
        let response = fake().respond("/assets/index-gone.js");
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(content_type(&response), "text/plain; charset=utf-8");
    }

    #[test]
    fn any_other_path_gets_the_index() {
        for path in ["/", "/alpha/mission/alpha-m1/brief", "/nothing.png"] {
            let response = fake().respond(path);
            assert_eq!(response.status(), StatusCode::OK, "{path}");
            assert_eq!(content_type(&response), "text/html; charset=utf-8");
        }
    }

    #[test]
    fn without_the_app_the_index_says_how_to_build_it() {
        let response = Assets::from_files(HashMap::new()).respond("/");
        assert_eq!(response.status(), StatusCode::OK);
        let body = futures_body(response);
        assert!(body.contains("pnpm build"), "{body}");
    }

    fn futures_body(response: Response) -> String {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let bytes = runtime
            .block_on(axum::body::to_bytes(response.into_body(), usize::MAX))
            .unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}
