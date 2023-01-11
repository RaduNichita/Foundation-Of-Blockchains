#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait Neversea {
    #[init]
    fn init(&self, initial_registration_fee: BigUint, initial_registration_fee_vip: BigUint) {
        self.registration_fee()
            .set(BigUint::from(initial_registration_fee));
        self.registration_fee()
            .set(BigUint::from(initial_registration_fee_vip));
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    #[payable("EGLD")]
    fn register(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = self.call_value().egld_value();
        require!(
            payment_amount == BigUint::from(self.registration_fee().get()) || payment_amount == BigUint::from(self.registration_fee_vip().get()),
            "Registration fee is incorrect; please check and try again"
        );
        if payment_amount == BigUint::from(self.registration_fee().get()) {
            self.participants().insert(caller);
        } else if payment_amount == BigUint::from(self.registration_fee().get()) {
            self.vip_participants().insert(caller);
        }
    }

    /// Add desired amount to the storage variable.
    #[only_owner] 
    #[endpoint]
    fn update_registration_fees(&self, initial_registration_fee: BigUint, initial_registration_fee_vip: BigUint) {
        self.registration_fee()
            .set(BigUint::from(initial_registration_fee));
        self.registration_fee()
            .set(BigUint::from(initial_registration_fee_vip));
    }

    #[view(getParticipants)]
    #[storage_mapper("participants")]
    fn participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getVipParticipants)]
    #[storage_mapper("vip_participants")]
    fn vip_participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getRegistrationFee)]
    #[storage_mapper("registration_fee")]
    fn registration_fee(&self) -> SingleValueMapper<BigUint>;

    #[view(getRegistrationFeeVip)]
    #[storage_mapper("registration_fee_vip")]
    fn registration_fee_vip(&self) -> SingleValueMapper<BigUint>;
}
