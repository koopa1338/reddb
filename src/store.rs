use super::document::{Document, Leches};
use super::error;
use super::json;
use super::status::Status;
use crate::json_store::JsonStore;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Lines};
use std::marker::Sized;
use std::result;

use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

pub type Result<T> = result::Result<T, error::RedDbError>;
pub type RedDbHashMap = HashMap<Uuid, Mutex<Document>>;
pub type WriteGuard<'a, T> = RwLockWriteGuard<'a, T>;
pub type ReadGuard<'a, T> = RwLockReadGuard<'a, T>;

#[derive(Debug)]
pub struct Store {
    pub store: RwLock<HashMap<Uuid, Mutex<Document>>>,
}

impl Store {
    pub fn new(buf: Lines<&[u8]>) -> Result<Self> {
        println!("[RedDb] Parsing database into memory");
        let mut map_store: RedDbHashMap = HashMap::new();
        for (_index, line) in buf.enumerate() {
            let content = &line?;
            let mut json_doc = json::from_str(&content)?;
            let _id = match &json_doc["_id"].as_str() {
                Some(_id) => Uuid::parse_str(_id)?,
                None => panic!("ERR: Wrong Uuid format!"),
            };
            json_doc["data"]["_id"] = Value::String(_id.to_string());
            let doc = Document {
                data: json_doc["data"].clone(),
                status: Status::Saved,
            };
            map_store.insert(_id, Mutex::new(doc));
        }

        Ok(Self {
            store: RwLock::new(map_store),
        })
    }

    pub fn to_read(&self) -> Result<ReadGuard<RedDbHashMap>> {
        Ok(self.store.read()?)
    }

    fn to_write(&self) -> Result<WriteGuard<RedDbHashMap>> {
        Ok(self.store.write()?)
    }

    fn get_id<'a>(&self, query: &'a Value) -> Result<&'a str> {
        //Fixme
        let _id = match query.get("_id").unwrap().as_str() {
            Some(_id) => _id,
            None => "",
        };
        Ok(_id)
    }

    fn get_uuid(&self, query: &Value) -> Result<Uuid> {
        let _id = self.get_id(query)?;
        let uuid = Uuid::parse_str(_id)?;
        Ok(uuid)
    }

    pub fn find_id<'a, T>(
        &self,
        store: RwLockReadGuard<HashMap<Uuid, Mutex<T>>>,
        id: &'a Value,
    ) -> &T {
        let uuid = self.get_uuid(&id).unwrap();
        let doc = store
            .get(&uuid)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Not found"))
            .unwrap();

        let guard = doc.lock().unwrap();
        let data = &*guard;
        &data
    }

    // pub fn flush_store(&self) -> Result<()> {
    //     let store = self.to_read::<Document>().unwrap();
    //     for (_key, doc) in store.iter() {
    //         println!("STORE RECORD {:?}", doc);
    //     }
    //     Ok(())
    // }

    /*
    pub fn format_jsondocs(&self) -> Vec<u8> {
        let store = self.to_read().unwrap();
        println!("STORE DATA{:?}", &store);
        let formated_docs: Vec<u8> = store
            .iter()
            .map(|(_k, v)| (_k, v.lock().unwrap()))
            .filter(|(_k, v)| v.status == Status::NotSaved)
            .flat_map(|doc| {
                let mut doc_vector = json::serialize(&doc).unwrap();
                doc_vector.extend("\n".as_bytes());
                doc_vector
            })
            .collect();
        formated_docs
    }

    pub fn format_operation(&self, documents: &Vec<(&Uuid, &mut Document)>) -> Vec<u8> {
        let formated_docs: Vec<u8> = documents
            .iter()
            .filter(|(_id, doc)| doc.status != Status::Saved)
            .map(|(_id, doc)| json::to_jsonlog(&_id, &doc).unwrap())
            .flat_map(|doc| {
                let mut doc_vector = json::serialize(&doc).unwrap();
                doc_vector.extend("\n".as_bytes());
                doc_vector
            })
            .collect();
        formated_docs
    }
    */
}
