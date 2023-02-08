
pub struct Request<T> {
    body: T,
}

impl<T> Request<T> {
    pub fn new(body: T) -> Self {
        Self { body: body }
    }
}