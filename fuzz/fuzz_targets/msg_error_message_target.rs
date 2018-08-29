extern crate lightning;

use lightning::ln::msgs;
use lightning::util::reset_rng_state;

use lightning::ln::msgs::{MsgEncodable, MsgDecodable};

#[inline]
pub fn do_test(data: &[u8]) {
	reset_rng_state();
	if let Ok(msg) = msgs::ErrorMessage::decode(data){
		let enc = msg.encode();
		assert_eq!(&data[0..32], &enc[0..32]);
		assert_eq!(&data[34..enc.len()], &enc[34..]);
	}
}

#[cfg(feature = "afl")]
#[macro_use] extern crate afl;
#[cfg(feature = "afl")]
fn main() {
	fuzz!(|data| {
		do_test(data);
	});
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
	loop {
		fuzz!(|data| {
			do_test(data);
		});
	}
}

extern crate hex;
#[cfg(test)]
mod tests {
	#[test]
	fn duplicate_crash() {
		super::do_test(&::hex::decode("00").unwrap());
	}
}