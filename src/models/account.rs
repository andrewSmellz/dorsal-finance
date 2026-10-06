use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Account{
    pub id: Uuid,
    pub name: String,
    pub kind: AccountKind,
}
#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum AccountKind{
    TFSA,
    RRSP,
    FHSA,
    Unregistered,
    Chequing,
    Saving,
}

impl Account{
    pub fn new(name: String, kind: AccountKind) -> Self{
        Account{
            id: Uuid::new_v4(),
            name,
            kind,
        }
    }
}