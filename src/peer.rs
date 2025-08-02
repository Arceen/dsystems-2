use std::sync::{Arc, Mutex};

pub type shared_peer_type = Arc<Mutex<Vec<String>>>;
