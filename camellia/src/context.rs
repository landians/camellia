
#[derive(Debug)]
pub struct ServerContext {
    service: String,
    method: String,
}

impl ServerContext {
    pub fn service_path(&self) -> String {
        format!("/{}/{}", &self.service, &self.method)
    }
}