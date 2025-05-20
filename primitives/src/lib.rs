#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::sr25519::Signature as Sr25519Signature;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature, MultiSigner,
};

/// Account ID type
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance type
pub type Balance = u128;

/// Block number type
pub type BlockNumber = u32;

/// Hash type
pub type Hash = sp_core::H256;

/// Signature type
pub type Signature = MultiSignature;

/// Industry type enumeration
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
pub enum IndustryType {
    /// Software development and engineering
    SoftwareDevelopment,
    /// DevOps and infrastructure engineering  
    DevOpsEngineering,
    /// Blockchain and Web3 development
    BlockchainDevelopment,
    /// Data science and analytics
    DataScience,
    /// Digital art and graphic design
    DigitalCreative,
    /// Music production and audio engineering
    MusicProduction,
    /// Virtual assistance and administrative services
    VirtualAssistance,
    /// Content creation and marketing
    ContentCreation,
    /// Business strategy and operations consulting
    BusinessConsulting,
    /// Financial advisory and analysis
    FinancialAdvisory,
    /// Project management and coordination
    ProjectManagement,
    /// Legal services and compliance
    LegalServices,
}

/// Verification level enumeration
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
pub enum VerificationLevel {
    /// No verification
    None,
    /// Basic on-chain verification
    Basic,
    /// Document-based verification
    Intermediate, 
    /// Authority/expert verification
    Expert,
    /// Institutional verification
    Institutional,
}
