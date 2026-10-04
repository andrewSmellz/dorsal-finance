use uuid::Uuid;

pub struct Account{
    id: Uuid,
    name: String,
    kind: AccountKind,
}

pub enum AccountKind{
    TFSA,
    RRSP,
    FHSA,
    Unregistered,
    Chequing,
    Saving,
}