//! Maintain the structure of account data for the Universal Password Manager version 3.
//!
//! Separating the account structure from database prepares for incorporating other
//! functions within account.
//!

use std::cmp::Ordering;
use std::vec::Vec;

// Human-readable field labels
const FIELD_NAME: &'static str = "Account";
const FIELD_USER: &'static str = "Username";
const FIELD_PASSWORD: &'static str = "Password";
const FIELD_URL: &'static str = "URL";
const FIELD_NOTES: &'static str = "Notes";

/// Describe a specific account field.
#[derive(Clone)]
pub struct Field {
    pub name: &'static str,
    pub secret: bool,
    pub multiline: bool,
}

/// Provide a description of each account field.
pub const FIELDS: [Field; 5] = [
    Field {
        name: FIELD_NAME,
        secret: false,
        multiline: false,
    },
    Field {
        name: FIELD_USER,
        secret: false,
        multiline: false,
    },
    Field {
        name: FIELD_PASSWORD,
        secret: true,
        multiline: false,
    },
    Field {
        name: FIELD_URL,
        secret: false,
        multiline: false,
    },
    Field {
        name: FIELD_NOTES,
        secret: false,
        multiline: true,
    },
];

/// This struct represents a single UPM account, and provides an ordering based on the
/// alphanumeric case-insensitive comparison of account names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Account {
    pub name: String,
    pub user: String,
    pub password: String,
    pub url: String,
    pub notes: String,
}

impl Account {
    /// Create a new Account struct.  All fields are initialized to empty strings.
    pub fn new() -> Account {
        Account {
            name: String::new(),
            user: String::new(),
            password: String::new(),
            url: String::new(),
            notes: String::new(),
        }
    }

    // Update this account including the name of the account.
    pub fn update(&mut self, account: Account) {
        self.name = account.name;
        self.user = account.user;
        self.password = account.password;
        self.url = account.url;
        self.notes = account.notes;
    }

    pub fn fields() -> Vec<&'static Field> {
        let mut fields = Vec::new();
        fields.extend(FIELDS.iter().map(|x| x));
        fields
    }

    pub fn field(fieldname: &str) -> Option<Field> {
        for i in 0..FIELDS.len() {
            if FIELDS[i].name == fieldname {
                return Some(FIELDS[i].clone());
            }
        }
        None
    }
}

impl Ord for Account {
    /// Provide an ordering of accounts based on a case-insensitive comparison of account names.
    fn cmp(&self, other: &Account) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Account {
    fn partial_cmp(&self, other: &Account) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
