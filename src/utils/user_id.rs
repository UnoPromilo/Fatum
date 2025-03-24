use sqlx::types::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        UserId(uuid)
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}