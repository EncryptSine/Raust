use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub id: Uuid,
    pub service: String,
    pub username: String,
    pub password: String,
}

impl PasswordEntry {
    pub fn new(service: String, username: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            service,
            username,
            password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Vault {
    pub entries: Vec<PasswordEntry>,
}

impl Vault {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: PasswordEntry) {
        self.entries.push(entry);
    }

    pub fn delete_at(&mut self, index: usize) -> bool {
        if index < self.entries.len() {
            self.entries.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_at(&mut self, index: usize, s: String, u: String, p: String) -> bool {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.service = s;
            entry.username = u;
            entry.password = p;
            true
        } else {
            false
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}