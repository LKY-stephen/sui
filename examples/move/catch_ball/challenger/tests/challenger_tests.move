module challenger::tests;

use challenger::challenger::{create_challenge, settle, create_for_test};
use sui::balance;
use sui::coin;
use sui::random::{Self, Random};
use sui::sui::SUI;
use sui::test_scenario as ts;
use talus::auto_tx::minimum_gas;

#[test]
fun test_create_challenge() {
  let mut ts = ts::begin(@0xb111);

  let package = @0x1;
  let target = @0xa1111;
  let module_name = b"challenger".to_string();
  let entry_name = b"test".to_string();
  let gas = coin::mint_for_testing<SUI>(minimum_gas(), ts.ctx());
  let bet = coin::mint_for_testing<SUI>(minimum_gas(), ts.ctx());

  create_challenge(package, target, module_name, entry_name, gas, bet, ts.ctx());

  ts.end();
  // Add assertions to verify the challenge creation
}

#[test]
fun test_settle_function() {
  let mut ts = ts::begin(@0x0);
  random::create_for_testing(ts.ctx());

  ts.next_tx(@0xb111);
  let random_state: Random = ts.take_shared();

  let challenge = create_for_test(
    @0xa111,
    @0xb111,
    tx_context::epoch(ts.ctx()),
    balance::create_for_testing(minimum_gas()),
    ts.ctx(),
  );
  let answer = true;
  let bet2 = coin::mint_for_testing<SUI>(minimum_gas(), ts.ctx());

  settle(&random_state, challenge, answer, bet2, ts.ctx());

  ts::return_shared(random_state);
  ts.end();
}
