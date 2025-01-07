module talus::test_auto_tx {
    use sui::address;
    use sui::coin;
    use sui::sui::SUI;
    use sui::bcs;
    use talus::auto_tx::{create_auto_tx, drop_auto_tx, create_pure_Arg,create_type,create_auto_tx_for_test};
    use sui::test_scenario as ts;

    #[test]
    public fun test_create_auto_tx() {
        let mut ts = ts::begin(@0xb111);
        let trigger_time = ts.ctx().epoch_timestamp_ms();
        let caller = address::from_u256(0x1);
        let callee = address::from_u256(0x2);
        let module_name = b"test_module".to_string();
        let function_name = b"test_function".to_string();
        let type_inputs = vector::singleton(create_type(6));
        let gas = coin::mint_for_testing<SUI>(1, ts.ctx());
        
        let arguments = vector::singleton(create_pure_Arg(bcs::to_bytes(&100u64)));

        create_auto_tx(
            trigger_time,
            caller,
            callee,
            module_name,
            function_name,
            type_inputs,
            gas,
            arguments,
            ts.ctx(),
        );

        ts.end();
    }

    #[test]
    public fun test_drop_auto_tx() {
        let mut ts =ts::begin(@0xb111);
        let trigger_time = ts.ctx().epoch_timestamp_ms();
        let caller = address::from_u256(0xb111);
        let callee = address::from_u256(0xa111);
        let type_inputs = vector::singleton(create_type(6));
        let gas = coin::mint_for_testing<SUI>(1, ts.ctx());
        let arguments = vector::singleton(create_pure_Arg(bcs::to_bytes(&100u64)));

        let auto_tx =create_auto_tx_for_test(
            trigger_time,
            caller,
            callee, b"test_module", b"test_function",
            bcs::to_bytes(&type_inputs),object::id_to_address(&object::id(&gas)),bcs::to_bytes(&arguments),ts.ctx());

        ts.later_epoch(10, @0xb111);
        drop_auto_tx(auto_tx, gas, ts.ctx());

        ts.end();
    }
}