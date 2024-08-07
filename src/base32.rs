use std::sync::OnceLock;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use tokio::sync::Mutex;

pub const CHARS: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

static RNG: OnceLock<Mutex<ChaCha8Rng>> = OnceLock::new();

pub async fn random_secure(len: usize) -> String {
	let rng =
		RNG.get_or_init(|| Mutex::new(ChaCha8Rng::from_seed(rand::random())));
	let mut rng = rng.lock().await;
	let mut buffer = String::with_capacity(len);
	for _ in 0..len {
		let i = rng.gen_range(0..CHARS.len());
		buffer.push(CHARS[i] as char);
	}

	buffer
}
