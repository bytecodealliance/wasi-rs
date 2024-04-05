impl core::fmt::Display for crate::io::error::Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_debug_string())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for crate::io::error::Error {}
