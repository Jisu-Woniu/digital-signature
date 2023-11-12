use pgp::{SignedPublicKey, SignedSecretKey};

pub(crate) struct KeyPair {
    secret_key: SignedSecretKey,
    public_key: SignedPublicKey,
}

impl KeyPair {
    pub(crate) fn from_keys(secret_key: SignedSecretKey, public_key: SignedPublicKey) -> Self {
        Self {
            secret_key,
            public_key,
        }
    }
    pub(crate) fn secret_key(&self) -> &SignedSecretKey {
        &self.secret_key
    }

    pub(crate) fn public_key(&self) -> &SignedPublicKey {
        &self.public_key
    }
}
