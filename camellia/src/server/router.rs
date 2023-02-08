use std::{collections::HashMap, fmt, future::Future};

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
        S: Service<ServerContext, Request<B>, Response = Response<Body>, Error = Error>
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
    type Response = Response<Body>;
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

enum Fallback<B> {
    Default(Endpoint<B>),
    Service(Endpoint<B>),
}

impl<B> Clone for Fallback<B> {
    fn clone(&self) -> Self {
        match self {
            Self::Default(endpoint) => Self::Default(endpoint.clone()),
            Self::Service(endpoint) => Self::Service(endpoint.clone()),
        }
    }
}

impl<B> fmt::Debug for Fallback<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default(endpoint) => f.debug_tuple("Default").field(endpoint).finish(),
            Self::Service(endpoint) => f.debug_tuple("Service").field(endpoint).finish(),
        }
    }
}

pub struct Endpoint<B = Body>(BoxCloneService<ServerContext, Request<B>, Response<Body>, Error>);

impl<B> Endpoint<B>
where
    B: 'static,
{
    pub(crate) fn new<S>(service: S) -> Self
    where
        S: Service<ServerContext, Request<B>, Response = Response<Body>, Error = Error>
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

#[cfg(test)]
mod tests {
    use std::future::Future;

    use motore::Service;

    use crate::{body::Body, context::ServerContext, request::Request, error::Error, response::Response};

    use super::Router;

    #[derive(Debug, Clone)]
    struct AMockService {}

    impl Service<ServerContext, Request<Body>> for AMockService {
        type Response = Response<Body>;
        type Error = Error;
        type Future<'cx> = impl Future<Output = Result<Self::Response, Self::Error>> + 'cx
        where
            Self: 'cx;

        fn call<'cx, 's>(&'s self, cx: &'cx mut ServerContext, req: Request<Body>) -> Self::Future<'cx>
        where
            's: 'cx,
        {
            println!("Call A mock service..");
            async { Ok(Response::new(Body::Null)) }
        }
    }

    #[derive(Debug, Clone)]
    struct BMockService {}

    impl Service<ServerContext, Request<Body>> for BMockService {
        type Response = Response<Body>;
        type Error = Error;
        type Future<'cx> = impl Future<Output = Result<Self::Response, Self::Error>> + 'cx
        where
            Self: 'cx;

        fn call<'cx, 's>(&'s self, cx: &'cx mut ServerContext, req: Request<Body>) -> Self::Future<'cx>
        where
            's: 'cx,
        {
            println!("Call B mock service..");
            async { Ok(Response::new(Body::Null)) }
        }
    }

    #[test]
    fn test_router_service() {
        let service_a = AMockService {};

        let service_b = BMockService {};

        let mut service_a_cx = ServerContext{
            service: "A".to_string(),
            method: "call".to_string(),
        };

        let mut service_b_cx = ServerContext{
            service: "B".to_string(),
            method: "call".to_string(),
        };

        let service_a_req = Request::new(Body::Null);

        let service_b_req = Request::new(Body::Null);

        let app = Router::new()
            .handle("/A/call", service_a)
            .handle("/B/call", service_b);

        let _ = app.call(&mut service_a_cx, service_a_req);

        let _ = app.call(&mut service_b_cx, service_b_req);
    }
}
