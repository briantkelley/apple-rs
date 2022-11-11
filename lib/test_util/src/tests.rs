use core::hash::Hasher;

#[allow(clippy::redundant_pub_crate)]
pub(super) struct AddHasher(pub(super) u64);

impl Hasher for AddHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let value = match bytes.len() {
            0 => 0_u64,
            1 => u64::from(bytes[0]),
            2 => u64::from(u16::from_ne_bytes([bytes[0], bytes[1]])),
            4 => u64::from(u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
            8 => u64::from_ne_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
            _ => bytes
                .iter()
                .fold(0_u64, |sum, byte| sum.wrapping_add(u64::from(*byte))),
        };
        self.0 = self.0.wrapping_add(value);
    }
}
