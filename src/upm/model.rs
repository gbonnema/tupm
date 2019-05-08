//! structures and traits for representing a model

/// Describe a specific account field.
#[derive(Clone)]
pub struct Field {
    pub name: &'static str,
    pub label: &'static str,
    pub secret: bool,
    pub multiline: bool,
}

pub struct DataItem {
    pub fieldnr: u8,
    pub contents: String,
}

pub struct Data {
    pub key: &'static str,
    pub data_items: Vec<DataItem>,
}

/// Trait for model object
pub trait ModelObject {
    fn fields() -> Vec<&'static Field>
    where
        Self: Sized;
    fn field(fieldname: &str) -> Option<Field>
    where
        Self: Sized;
    fn object_data(&self) -> Data;
}

// Remember to add Iterator trait
pub trait ModelList {
    fn add(&self, key: &str, data: Data) -> Result<(), String>;
    fn rm(&self, key: &str) -> Result<Box<ModelObject>, String>;
    fn update(&self, key: &str, data_after: Data) -> Result<(), String>;
    fn get(&self, key: &str) -> Result<Box<ModelObject>, String>;
}

// Model communication

use std::sync::mpsc;

pub enum Request {
    GetModelList,
    GetModelObject(String),
    UpdateModelObject(Box<ModelObject>),
    Sync,
    Quit,
}

pub enum Notification {
    RefreshData,
}

pub trait ModelMsg {
    fn open_out_ch(&self) -> mpsc::Sender<Request>;
    fn open_in_ch(&self) -> mpsc::Receiver<Notification>;
}
