#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod nft_module;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[elrond_wasm::contract]
pub trait NftMinter: nft_module::NftModule {
    #[init]
    fn init(&self) {
    }
    
    #[only_owner]
    #[endpoint(setManager)]
    fn set_manager(&self,manager_address: ManagedAddress) -> SCResult<()> {
    	self.manager_address().set(&manager_address);
    	Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    #[payable("EGLD")]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        royalties: BigUint,
        uri: ManagedBuffer,
        selling_price: BigUint,
        #[payment] payment: BigUint,
        #[var_args] opt_token_used_as_payment: OptionalArg<TokenIdentifier>,
        #[var_args] opt_token_used_as_payment_nonce: OptionalArg<u64>,
    ) -> SCResult<u64> {
        let token_used_as_payment = opt_token_used_as_payment
            .into_option()
            .unwrap_or_else(|| self.types().token_identifier_egld());

        let token_used_as_payment_nonce = if token_used_as_payment.is_egld() {
            0
        } else {
            opt_token_used_as_payment_nonce
                .into_option()
                .unwrap_or_default()
        };

        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        
        let owner = self.blockchain().get_owner_address();
        let fee = payment/(BigUint::from(10 as u32));
        self.send().direct_egld(&owner, &fee, &[]);
        
        let manager_fee = self.blockchain().get_sc_balance(&self.types().token_identifier_egld(), 0);
        let manager_address = self.manager_address().get();
        self.send().direct_egld(&manager_address, &manager_fee, &[]);
        
        self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        )
    }
    #[view(get_manager_address)]
    #[storage_mapper("managerAddress")]
    fn manager_address(&self) -> SingleValueMapper<ManagedAddress>;
}
