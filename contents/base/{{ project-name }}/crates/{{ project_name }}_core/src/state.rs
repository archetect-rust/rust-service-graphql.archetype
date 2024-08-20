use crate::{Auth, {{ ProjectName }}Core};

pub struct RequestState {
    core: {{ ProjectName }}Core,
    auth: Auth,
}

impl RequestState {
    pub fn new(core: {{ ProjectName }}Core, auth: Auth) -> RequestState {
        RequestState {
            core,
            auth,
        }
    }
    pub fn core(&self) -> &{{ ProjectName }}Core {
        &self.core
    }

    pub fn auth(&self) -> &Auth {
        &self.auth
    }
}