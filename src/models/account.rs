use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub kind: AccountKind,
    pub allow_negative: bool,
}
#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum AccountKind {
    TFSA,
    RRSP,
    FHSA,
    Unregistered,
    Chequing,
    Saving,
    CreditCard,
    Loan,
}

impl Account {
    pub fn new(name: String, kind: AccountKind) -> Self {
        let allow_negative = kind.is_liability();
        Account {
            id: Uuid::new_v4(),
            name,
            kind,
            allow_negative,
        }
    }

    pub fn with_allow_negative(mut self, allow: bool) -> Self {
        self.allow_negative = allow;
        self
    }
}

impl AccountKind {
    pub fn is_liability(&self) -> bool {
        matches!(self, AccountKind::Loan | AccountKind::CreditCard)
    }
}
