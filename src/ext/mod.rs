#[cfg(feature = "std")]
mod std;

#[cfg(feature = "rand")]
pub mod rand;

impl core::fmt::Display for crate::io::error::Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_debug_string())
    }
}
