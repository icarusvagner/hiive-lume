pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use rand::RngCore;

fn main() -> Result<()> {
	let mut key = [0u8; 64];
	rand::rng().fill_bytes(&mut key);
	println!("\nGenerated key from rand::rng()\n{key:?}");

	let b64u = lib_utils::b64::b64u_encode(key);
	println!("\nGenerated key from Base64\n{b64u:?}");

	Ok(())
}
