pub use rand;

use rand::{CryptoRng, RngCore};

/// The secure interface for cryptographically-secure random numbers
pub struct HostRng;

impl CryptoRng for HostRng {}

impl RngCore for HostRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        crate::random::random::get_random_u64() as _
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        crate::random::random::get_random_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let n = dest.len();
        if usize::BITS <= u64::BITS || n <= u64::MAX as _ {
            dest.copy_from_slice(&crate::random::random::get_random_bytes(n as _));
        } else {
            let (head, tail) = dest.split_at_mut(u64::MAX as _);
            head.copy_from_slice(&crate::random::random::get_random_bytes(u64::MAX));
            self.fill_bytes(tail);
        }
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

/// The insecure interface for insecure pseudo-random numbers
pub struct HostInsecureRng;

impl RngCore for HostInsecureRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        crate::random::insecure::get_insecure_random_u64() as _
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        crate::random::insecure::get_insecure_random_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let n = dest.len();
        if usize::BITS <= u64::BITS || n <= u64::MAX as _ {
            dest.copy_from_slice(&crate::random::insecure::get_insecure_random_bytes(n as _));
        } else {
            let (head, tail) = dest.split_at_mut(u64::MAX as _);
            head.copy_from_slice(&crate::random::insecure::get_insecure_random_bytes(
                u64::MAX,
            ));
            self.fill_bytes(tail);
        }
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
