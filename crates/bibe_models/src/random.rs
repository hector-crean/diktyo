use std::sync::{Arc, Mutex};

use rand_chacha::ChaCha8Rng;

pub type Random = Arc<Mutex<ChaCha8Rng>>;
