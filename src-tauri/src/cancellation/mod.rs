use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct CancellationTokens(pub Mutex<HashMap<String, Arc<AtomicBool>>>);

impl CancellationTokens {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    pub fn insert(&self, request_id: String) -> Arc<AtomicBool> {
        let token = Arc::new(AtomicBool::new(false));
        let mut map = self.0.lock().unwrap();
        map.insert(request_id, token.clone());
        token
    }

    pub fn cancel(&self, request_id: &str) -> bool {
        let map = self.0.lock().unwrap();
        if let Some(token) = map.get(request_id) {
            token.store(true, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    pub fn remove(&self, request_id: &str) {
        let mut map = self.0.lock().unwrap();
        map.remove(request_id);
    }
}

pub fn is_cancelled(token: &Arc<AtomicBool>) -> bool {
    token.load(Ordering::Relaxed)
}
