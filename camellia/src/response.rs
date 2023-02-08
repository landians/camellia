
pub struct Response<T> {
    body: T,
}

impl<T> Response<T> {
    pub fn new(body: T) -> Self {
        Self { body: body }
    }
}