
#[derive(Debug)]
pub struct ServerContext {
    pub service: String,
    pub method: String,
}

impl ServerContext {
    pub fn service_path(&self) -> String {
        format!("/{}/{}", &self.service, &self.method)
    }
}