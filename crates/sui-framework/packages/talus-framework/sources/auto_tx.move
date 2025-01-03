
module talus::auto_tx {

    use sui::address;
    use sui::transfer::{public_transfer};
    use sui::coin::{Coin};

    /// Definition of the `AutoTx` struct
    public struct AutoTx has key, store {
        id: UID,
        trigger_time: u64,
        caller: address,
        callee: address,
        module_name: vector<u8>,
        function_name: vector<u8>,
        type_inputs: vector<u8>,
        gas_id: address,
        arguments: vector<u8>,
    }

    /// Entry function to create an instance of the `AutoTx` struct
    public fun create_auto_tx(
        ctx: &mut TxContext,
        trigger_time: u64,
        caller: address,
        callee: address,
        module_name: vector<u8>,
        function_name: vector<u8>,
        type_inputs: vector<u8>,
        gas: Coin<u64>,
        arguments: vector<u8>
    ): () {
        // Transfer the gas (coin) to the package address
        let gas_id = object::id_to_address(&object::id(&gas));
        public_transfer(gas, address::from_u256(0xa070));

        // Create the AutoTx object
        let auto_tx = AutoTx {
            id: object::new(ctx),
            trigger_time,
            caller,
            callee,
            module_name,
            function_name,
            type_inputs,
            gas_id: gas_id,
            arguments:arguments,
        };

        // Transfer the AutoTx object to the package address
        public_transfer(auto_tx, address::from_u256(0xa070));
    }
}