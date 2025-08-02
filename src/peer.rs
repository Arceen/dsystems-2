use std::sync::{Arc, Mutex};

pub type SharedPeerType = Arc<Mutex<Vec<String>>>;
