#![cfg_attr(not(feature = "std"), no_std, no_main)]

use pop_api::nfts;
use pop_api::nfts::*;
use enumflags2::BitFlags;

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
	InvalidCollection,
	NftsError(nfts::Error),
}

impl From<nfts::Error> for ContractError {
	fn from(value: nfts::Error) -> Self {
		ContractError::NftsError(value)
	}
}

#[ink::contract]
mod my_nft {
    use super::*;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MyNft {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl MyNft {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
		pub fn create_nft_collection( &self ) -> Result<(), ContractError>{
			// create collection api
			// pop_api::nfts::mint(collection_id, item_id, receiver)?;
            // let the admin be the caller
            let admin = Self::env().caller();
            // create a MintSetting with mint_type, price, start_block, end_block, default_item_settings
            use pop_api::nfts::ItemSetting;

            // Return an ItemSettings with an ItemSetting with the Transferable flag
            let item_settings = ItemSettings(BitFlags::from(ItemSetting::Transferable));

            let mint_settings = MintSettings {
                mint_type: MintType::Issuer,
                price: Some(0),
                start_block: Some(0),
                end_block: Some(0),
                default_item_settings: item_settings,
            };

            // create a CollectionConfig with everything enabled
            let config = CollectionConfig {
                settings: CollectionSettings(BitFlags::from(CollectionSetting::TransferableItems)),  
                max_supply: None,
                mint_settings,
            };
            pop_api::nfts::create(admin, config)?;
			ink::env::debug_println!("Contract::create_nft_collection: collection created successfully");
            Ok(())
		}
    }
}
