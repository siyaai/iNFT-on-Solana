use anchor_lang::prelude::*;
use mpl_token_metadata::types::{Creator, DataV2, Collection};

// create nft metadata helper function
pub fn create_nft_metadata_data(
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    pda_creator: Pubkey,
    royalty_receiver: Pubkey,
    collection_mint: Pubkey,
) -> DataV2 {
    DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points,
        creators: Some(vec![
            Creator {
                address: pda_creator,
                verified: true,
                share: 0,  // PDA creator does not participate in royalty sharing
            },
            Creator {
                address: royalty_receiver,
                verified: false,
                share: 100,  // Royalty receiver gets all royalties
            }
        ]),
        collection: Some(Collection {
            verified: false,
            key: collection_mint,
        }),
        uses: None,
    }
}

// create collection metadata helper function
pub fn create_collection_metadata_data(
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
    creator: Pubkey,
) -> DataV2 {
    DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points,
        creators: Some(vec![Creator {
            address: creator,
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    }
}
