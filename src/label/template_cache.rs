


use std::{collections::HashMap, sync::RwLock};

use futures::future::BoxFuture;

use crate::cli::config::Config;



struct TemplateCache {
    map: RwLock<HashMap<String, futures::future::Shared<BoxFuture<'static, Option<String>>>>>
}
