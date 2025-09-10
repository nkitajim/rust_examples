pub fn i32_count_bit(num: i32) -> i32 {
	(0..32)
		.map(|i| (num >> i) & 0x0001)
		.sum()
}

#[inline]
pub fn inline_i32_count_bit(num: i32) -> i32 {
	(0..32)
		.map(|i| (num >> i) & 0x0001)
		.sum()
}

pub fn f32_count_bit(f: f32) -> i32 {
	let b: u32 = f.to_bits();
	(0..32)
		.map(|i| (b >> i) & 0x0001)
		.map(|i| i as i32)
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_i32_count_bit() {
		assert_eq!(i32_count_bit(1), 1);
		assert_eq!(i32_count_bit(3), 2);
		assert_eq!(i32_count_bit(7), 3);
		assert_eq!(i32_count_bit(15), 4);
		assert_eq!(i32_count_bit(31), 5);
		assert_eq!(i32_count_bit(63), 6);
		assert_eq!(i32_count_bit(127), 7);
		assert_eq!(i32_count_bit(255), 8);
	}

	#[test]
	fn test_inline_i32_count_bit() {
		assert_eq!(inline_i32_count_bit(1), 1);
		assert_eq!(inline_i32_count_bit(3), 2);
		assert_eq!(inline_i32_count_bit(7), 3);
		assert_eq!(inline_i32_count_bit(15), 4);
		assert_eq!(inline_i32_count_bit(31), 5);
		assert_eq!(inline_i32_count_bit(63), 6);
		assert_eq!(inline_i32_count_bit(127), 7);
		assert_eq!(inline_i32_count_bit(255), 8);
	}

	#[test]
	fn test_f32_count_bit() {
		assert_eq!(f32_count_bit(1.0), 1.0_f32.to_bits().count_ones().try_into().unwrap());
		assert_eq!(f32_count_bit(3.0), 3.0_f32.to_bits().count_ones().try_into().unwrap());
		assert_eq!(f32_count_bit(7.0), 7.0_f32.to_bits().count_ones().try_into().unwrap());
		assert_eq!(f32_count_bit(15.0), 15.0_f32.to_bits().count_ones().try_into().unwrap());
		assert_eq!(f32_count_bit(31.0), 31.0_f32.to_bits().count_ones().try_into().unwrap());
		assert_eq!(f32_count_bit(63.0), 63.0_f32.to_bits().count_ones().try_into().unwrap());
		assert_eq!(f32_count_bit(127.0), 127.0_f32.to_bits().count_ones().try_into().unwrap());
		assert_eq!(f32_count_bit(255.0), 255.0_f32.to_bits().count_ones().try_into().unwrap());
	}
}
