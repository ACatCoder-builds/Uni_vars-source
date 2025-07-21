#![allow(unused)]
use std::collections::HashMap;
use std::sync::Mutex;
use std::any::Any;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_VARS: Mutex<HashMap<String, StoredValue>> =
        Mutex::new(HashMap::new());
}

pub struct StoredValue {
    pub value: Box<dyn Any + Send + Sync>,
    pub typ: String,
}


#[macro_export]
macro_rules! global {
    ($name:expr, $val:expr, $typ:ty) => {{
        let stored = $crate::StoredValue {
            value: Box::new($val),
            typ: std::any::type_name::<$typ>().to_string(),
        };
        $crate::GLOBAL_VARS.lock().unwrap().insert($name.to_string(), stored);
    }};
}

pub fn get<T: 'static + Clone>(name: &str) -> Option<T> {
    let globals = crate::GLOBAL_VARS.lock().unwrap();
    globals.get(name)?.value.downcast_ref::<T>().cloned()
}

#[macro_export]
macro_rules! get {
    ($name:expr, $typ:ty) => {
        $crate::get::<$typ>($name)
    }
}

#[macro_export]
macro_rules! forget {
    ($name:expr) => {
	$crate::GLOBAL_VARS.lock().unwrap().remove($name)
    }
}
