use uuid::Uuid;

pub struct Account{
    account_id: Uuid,
    account_name: String,
    account_kind: AccountKind,
}

pub enum AccountKind{
    TFSA,
    RRSP,
    FHSA,
    Unregistered,
    Chequing,
    Saving,
}