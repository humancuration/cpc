use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use std::collections::HashMap;

/// Unique wallet identifier
pub type WalletId = String;

/// Wallet entity with cryptographic keys
pub struct Wallet {
    pub id: WalletId,
    pub public_key: PublicKey,
    secret_key: SecretKey, // Keep private for security
}

impl Wallet {
    /// Generate a new wallet with random keypair
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);
        
        Wallet {
            id: hex::encode(keypair.public.as_bytes()),
            public_key: keypair.public,
            secret_key: keypair.secret,
        }
    }

    /// Sign a transaction payload
    pub fn sign(&self, payload: &[u8]) -> Signature {
        let keypair = Keypair {
            public: self.public_key,
            secret: self.secret_key.clone(),
        };
        keypair.sign(payload)
    }

    /// Verify a signature for given payload
    pub fn verify(&self, payload: &[u8], signature: &Signature) -> bool {
        self.public_key.verify(payload, signature).is_ok()
    }
}

/// Wallet service for managing multiple wallets
pub struct WalletService {
    wallets: HashMap<WalletId, Wallet>,
}

impl WalletService {
    pub fn new() -> Self {
        WalletService {
            wallets: HashMap::new(),
        }
    }

    /// Create and store a new wallet
    pub fn create_wallet(&mut self) -> WalletId {
        let wallet = Wallet::new();
        let id = wallet.id.clone();
        self.wallets.insert(id.clone(), wallet);
        id
    }

    /// Get wallet by ID
    pub fn get_wallet(&self, id: &WalletId) -> Option<&Wallet> {
        self.wallets.get(id)
    }

    /// Get wallet mutable reference by ID
    pub fn get_wallet_mut(&mut self, id: &WalletId) -> Option<&mut Wallet> {
        self.wallets.get_mut(id)
    }

    /// Get wallet balance by querying transaction ledger
    pub fn get_balance(&self, wallet_id: &WalletId) -> rust_decimal::Decimal {
        // In real implementation, this would query the transaction ledger
        rust_decimal::Decimal::ZERO
    }
}