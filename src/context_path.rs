use std::{
    sync::Arc,
    task::{Context, Poll},
};

use http::{Request, Uri};
use tower::Layer;
use tower_service::Service;

#[derive(Debug, Clone)]
pub struct StripPrefixLayer {
    prefix: Arc<str>,
}

impl StripPrefixLayer {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl<S> Layer<S> for StripPrefixLayer {
    type Service = StripPrefix<S>;

    fn layer(&self, inner: S) -> Self::Service {
        StripPrefix {
            inner,
            prefix: Arc::clone(&self.prefix),
        }
    }
}

#[derive(Clone)]
pub struct StripPrefix<S> {
    inner: S,
    prefix: Arc<str>,
}

impl<S, B> Service<Request<B>> for StripPrefix<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        if let Some(new_uri) = strip_prefix(req.uri(), &self.prefix) {
            *req.uri_mut() = new_uri;
        }
        self.inner.call(req)
    }
}

fn strip_prefix(uri: &Uri, prefix: &str) -> Option<Uri> {
    let path = uri.path_and_query()?;
    if path.path().starts_with(prefix) {
        let new_path = &path.path()[prefix.len()..];
        let new_path_and_query = match (new_path.starts_with('/'), path.query()) {
            (true, None) => new_path.parse().unwrap(),
            (true, Some(query)) => format!("{new_path}?{query}").parse().unwrap(),
            (false, None) => format!("/{new_path}").parse().unwrap(),
            (false, Some(query)) => format!("/{new_path}?{query}").parse().unwrap(),
        };
        let mut parts = uri.clone().into_parts();
        parts.path_and_query = Some(new_path_and_query);
        Some(Uri::from_parts(parts).unwrap())
    } else {
        None
    }
}
