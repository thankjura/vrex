pub const PICO4: &str = "PICO4";
pub const QUEST2: &str = "Quest_2";

#[derive(Debug, Clone)]
pub enum DevType {
    Quest2,
    Pico4,
}

#[derive(Debug, Clone)]
pub struct Device {
    id: String,
    dev_type: DevType,
    online: bool,
}

impl Device {
    pub fn new(id: &str, dev_type: DevType) -> Self {
        Self {
            id: String::from(id),
            dev_type,
            online: true,
        }
    }

    pub fn set_online(&mut self, online: bool) {
        self.online = online;
    }
    
    pub fn is_online(&self) -> bool {
        self.online
    }

    pub fn dev_type(&self) -> &DevType {
        &self.dev_type
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq_ignore_ascii_case(&other.id)
    }
}
