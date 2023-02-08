use std::{collections::HashMap, fmt, future::Future};

use bytes::Bytes;
use motore::{BoxCloneService, Service};

use crate::{
    body::Body, context::ServerContext, error::Error, request::Request, response::Response,
};

#[derive(Default)]
pub struct Router<B = Body> {
    routes: HashMap<String, Endpoint<B>>,
}

impl<B> Clone for Router<B> {
    fn clone(&self) -> Self {
        Self {
            routes: self.routes.clone(),
        }
    }
}

impl<B> fmt::Debug for Router<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("routes", &self.routes)
            .finish()
    }
}

impl<B> Router<B>
where
    B: 'static,
{
    pub fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    pub fn handle<S>(mut self, path: &str, service: S) -> Self
    where
        S: Service<ServerContext, Request<B>, Response = Response<Bytes>, Error = Error>
            + Clone
            + Send
            + Sync
            + 'static,
    {
        if path.is_empty() {
            panic!("[CAMELLIA] Paths must start with a `/`. Use \"/\" for root routes");
        } else if !path.starts_with('/') {
            panic!("[CAMELLIA] Paths must start with a `/`");
        }

        self.routes.insert(path.to_string(), Endpoint::new(service));

        self
    }
}

impl<B> Service<ServerContext, Request<B>> for Router<B>
where
    B: Send,
{
    type Response = Response<Bytes>;
    type Error = Error;
    type Future<'cx> = impl Future<Output = Result<Self::Response, Self::Error>> + 'cx
    where
        Self: 'cx;

    fn call<'cx, 's>(&'s self, cx: &'cx mut ServerContext, req: Request<B>) -> Self::Future<'cx>
    where
        's: 'cx,
    {
        async move {
            let path = cx.service_path();

            match self.routes.get(&path) {
                Some(s) => {
                    let route = s.0.clone();
                    route.call(cx, req).await
                }
                None => Err(Error::ServiceNotFound(path)),
            }
        }
    }
}

pub struct Endpoint<B = Body>(BoxCloneService<ServerContext, Request<B>, Response<Bytes>, Error>);

impl<B> Endpoint<B>
where
    B: 'static,
{
    pub(crate) fn new<S>(service: S) -> Self
    where
        S: Service<ServerContext, Request<B>, Response = Response<Bytes>, Error = Error>
            + Clone
            + Send
            + Sync
            + 'static,
    {
        Self(BoxCloneService::new(service))
    }
}

impl<B> Clone for Endpoint<B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<B> fmt::Debug for Endpoint<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Endpoint")
            .field("endpoint", &self.0)
            .finish()
    }
}
