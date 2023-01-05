use std::cmp::Ordering;

#[derive(Debug)]
pub struct App {
    id: String,
    version: String,
    system: bool,
}

pub struct AppBuilder {
    id: Option<String>,
    version: Option<String>,
    system: bool,
}

impl PartialEq<Self> for App {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl PartialOrd for App {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn is_newer_than(&self, other: &App) -> bool {
        self.version > other.version
    }

    pub fn name(&self) -> &str {
        &self.id
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn is_system(&self) -> bool {
        self.system
    }
}

impl AppBuilder {
    fn new() -> Self {
        Self {
            id: None,
            version: None,
            system: false,
        }
    }

    pub fn id(&mut self, id: String) {
        self.id.replace(id);
    }

    pub fn version(&mut self, version: String) {
        self.version.replace(version);
    }

    pub fn system(&mut self, system: bool) {
        self.system = system;
    }

    pub fn build(&self) -> Option<App> {
        if self.id.is_some() && self.version.is_some() {
            return Some(App {
                id: self.id.clone().unwrap(),
                version: self.version.clone().unwrap(),
                system: self.system,
            });
        }
        None
    }
}