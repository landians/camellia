pub struct Request<T> {
    builder: Builder,
    body: T,
}

impl<T> Request<T> {
    #[inline]
    pub fn new(body: T) -> Self {
        Self {
            builder: Builder::new(),
            body: body,
        }
    }

    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

/// A RPC request builder.
#[derive(Default, Debug, Clone)]
pub struct Builder {
    /// RPC request service.
    service: String,

    /// RPC request method.
    method: String,
}

impl Builder {
    #[inline]
    pub fn new() -> Builder {
        Default::default()
    }

    #[inline]
    pub fn service(&mut self, service: &str) -> &mut Self {
        self.service = service.to_string();
        self
    }

    #[inline]
    pub fn method(&mut self, method: &str) -> &mut Self {
        self.method = method.to_string();
        self
    }

    #[inline]
    pub fn body<T>(self, body: T) -> Request<T> {
        Request {
            builder: self,
            body: body,
        }
    }
}
