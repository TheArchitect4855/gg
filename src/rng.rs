use std::sync::OnceLock;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use tokio::sync::{Mutex, MutexGuard};

const BASE32_CHARS: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

static SECURE_GEN: OnceLock<Mutex<ChaCha20Rng>> = OnceLock::new();

pub async fn gen_base32_secure(len: usize) -> String {
	let mut buf = String::with_capacity(len);
	let mut rng = get_rng_secure().await;
	for _ in 0..len {
		let i = rng.gen_range(0..BASE32_CHARS.len());
		buf.push(BASE32_CHARS[i] as char);
	}

	buf
}

async fn get_rng_secure() -> MutexGuard<'static, impl Rng> {
	SECURE_GEN
		.get_or_init(|| Mutex::new(ChaCha20Rng::from_entropy()))
		.lock()
		.await
}
