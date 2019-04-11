//! Maintain the structure of account data for the Universal Password Manager version 3.
//!
//! Separating the account structure from database prepares for incorporating other
//! functions within account.
//!

use std::cmp::Ordering;
use std::str;

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
