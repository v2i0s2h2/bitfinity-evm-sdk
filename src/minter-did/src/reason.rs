/// reason.rs: Is file mein Icrc2Burn struct hai. Ye ICRC-2 tokens (jo IC par ek token standard hai) ko burn karke ERC721 mint karne ke liye use hota hai. Isme ye information hoti hai:



use candid::{CandidType, Principal};
use did::{H160, U256};
use ic_exports::icrc_types::icrc1::account::Subaccount;
use serde::{Deserialize, Serialize};

/// Information to perform burn operation for ICRC-2 token and create a mint order.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Icrc2Burn {
    /// Principal from which tokens should be sent.
    pub sender: Principal,

    /// Amount to burn;
    pub amount: U256,

    /// Principal of ICRC-2 token to burn.
    pub icrc2_token_principal: Principal,

    /// Subaccount of the ICRC-2 token from which amount will be burned.
    pub from_subaccount: Option<Subaccount>,

    /// Address of the Wrapped token recipient.
    pub recipient_address: H160,

    /// Address from which fee should be charged for mint transaction
    /// performed by minter canister.
    /// If None, mint transaction will not be sent and user can send it by himself.
    pub fee_payer: Option<H160>,
}

// sender: Kon burn kar raha hai
// amount: Kitna burn karna hai
// icrc2_token_principal: canister id of icrc2 ledger
// recipient_address: Naya ERC721 kisko milega
// fee_payer: Mint ka fee kon dega