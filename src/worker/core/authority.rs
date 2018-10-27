#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum Authority {
    Authoritative(),
    AuthorityLossImminent(),
    NotAuthoritative()
}

impl Authority {
    pub fn has_authority(&self) -> bool {
        self != &Authority::NotAuthoritative()
    }
}

impl From<u8> for Authority {
    fn from(auth: u8) -> Self {
        match auth {
            0 => Authority::NotAuthoritative(),
            1 => Authority::Authoritative(),
            2 => Authority::AuthorityLossImminent(),
            _ => panic!("Unknown authority state: {}", auth)
        }
    }
}
