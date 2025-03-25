#[derive(Clone)]
pub struct HmacSecret(String);

impl AsRef<[u8]> for HmacSecret {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Into<HmacSecret> for &str {
    fn into(self) -> HmacSecret {
        HmacSecret(self.to_string())
    }
}
