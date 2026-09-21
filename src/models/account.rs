struct Account{
    account_id: Uuid,
    account_name: String,
    account_kind: AcountKind,
}

enum AccountKind{
    TFSA,
    RRSP,
    FHSA,
    Unregistered,
    Chequing,
    Saving,
}