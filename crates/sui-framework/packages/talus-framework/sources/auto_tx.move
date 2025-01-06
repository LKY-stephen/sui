
module talus::auto_tx {

    use sui::address;
    use sui::bcs;
    use sui::transfer::{public_transfer};
    use sui::coin::{Coin};
    use std::string::{Self, String};
    use sui::sui::SUI;
    
    #[error]
    const ArgMismatch:vector<u8>  = b"Input types and arguments mismatch";

    #[error]
    const InvalidCaller:vector<u8>  = b"Invalid Caller";

    #[error]
    const InvalidGas:vector<u8>  = b"Need to reclaim the exact gas object";

    #[error]
    const InvalidDelete:vector<u8>  = b"Cannot Delete Not Triggered AutoTx";

    public enum TypeTag has store, drop {
    Bool,
    U8,
    U64,
    U128,
    Address,
    // no signer is allowed
    Signer_not_supported,
    // Add support when recursive types are supported
    Vector_not_supported,
    // use address instead
    Struct_not_supported,
    U16,
    U32,
    U256,
    }

    public struct ObjectRef has store, drop {
        id: address,
        version: u64,
        mutable: bool,
        receiving: bool,
    }

    public fun create_object_ref(id: address, version: u64, mutable: bool, receiving: bool): ObjectRef {
        ObjectRef {
            id,
            version,
            mutable,
            receiving,
        }
    }

    public enum CallArg has store, drop {
        // contains no structs or objects
        Pure(vector<u8>),
        // an object
        Object(ObjectRef),
    }



    /// Definition of the `AutoTx` struct
    public struct AutoTx has key, store  {
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

    /// function to create an instance of the `AutoTx` struct
    public fun create_auto_tx(
        ctx: &mut TxContext,
        trigger_time: u64,
        caller: address,
        callee: address,
        module_name: String,
        function_name: String,
        type_inputs: vector<TypeTag>,
        gas: Coin<SUI>,
        arguments: vector<CallArg>
    ): () {
        // Transfer the gas (coin) to the package address
        let gas_id = object::id_to_address(&object::id(&gas));
        public_transfer(gas, address::from_u256(0xa070));
        // Ensure type_inputs and arguments have a one-to-one correspondence
        let len = vector::length(&type_inputs);

        // Ensure type_inputs and arguments have a one-to-one correspondence
        assert!(len== vector::length(&arguments), ArgMismatch);
        let mut i = 0;
        while (i < len) {
            let input = vector::borrow(&type_inputs, i);
            let arg = vector::borrow(&arguments, i);
            match(arg) {
                CallArg::Pure(_)=> {
                    match(input) {
                        TypeTag::Vector_not_supported => { assert!(false, ArgMismatch); },
                        TypeTag::Struct_not_supported => { assert!(false, ArgMismatch); },
                        TypeTag::Signer_not_supported => { assert!(false, ArgMismatch); },
                        _ => { }
                    }
                },
                CallArg::Object(_) => {
                    match(input) {
                        TypeTag::Address => {},
                        _ => { assert!(false, ArgMismatch); }
                    }
                }
            };
            i = i + 1;
        };
        
        // Create the AutoTx object
        let auto_tx = AutoTx {
            id: object::new(ctx),
            trigger_time,
            caller,
            callee,
            module_name: *string::as_bytes(&module_name),
            function_name: *string::as_bytes(&function_name),
            type_inputs: bcs::to_bytes(&type_inputs),
            gas_id: gas_id,
            arguments: bcs::to_bytes(&arguments),
        };

        // Transfer the AutoTx object to the package address
        public_transfer(auto_tx, address::from_u256(0xa070));

    }

    /// function to create an instance of the `AutoTx` struct
    public fun drop_auto_tx(
        ctx: &mut TxContext,
        obj: AutoTx,
        remain: Coin<SUI>
    ): () {
        // TODO: add cancel logic for deleting not triggered AutoTx, and reclaim gas.
        let epoch_start = ctx.epoch_timestamp_ms();
        assert!(epoch_start>obj.trigger_time, InvalidDelete);
        
        // claim back remaining gas
        let gas_id = object::id_to_address(&object::id(&remain));
        assert!(gas_id == obj.gas_id, InvalidGas);

        // only caller can delete
        assert!(obj.caller == ctx.sender(), InvalidCaller);

        // transfer gas back to caller
        public_transfer(remain, obj.caller);

        
        let AutoTx { id, .. } = obj;
        object::delete(id);

    }
}