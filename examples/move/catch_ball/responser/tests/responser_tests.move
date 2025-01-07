module responser::tests;

use challenger::challenger::create_for_test;
use responser::responser::{initiate, response, merge, redeem};
use sui::balance;
use sui::coin;
use sui::sui::SUI;
use sui::test_scenario as ts;
use talus::auto_tx::minimum_gas;

#[test]
fun test_responser() {
  let owner = @0xb111;
  let mut ts = ts::begin(owner);

  let deposit = coin::mint_for_testing<SUI>(10*minimum_gas(), ts.ctx());

  let ctx = ts.ctx();
  initiate(deposit, ctx);

  ts.next_tx(owner);
  let mut responser = ts.take_shared();

  let reward = coin::mint_for_testing<SUI>(2*minimum_gas(), ts.ctx());
  merge(&mut responser, reward, ts.ctx());

  redeem(&mut responser, 1, ts.ctx());

  ts::return_shared(responser);

  ts.end();
  // Add assertions to verify the challenge creation
}

#[test]
fun test_response() {
  let owner = @0xb111;

  let mut ts = ts::begin(owner);
  let deposit = coin::mint_for_testing<SUI>(10*minimum_gas(), ts.ctx());

  let ctx = ts.ctx();
  initiate(deposit, ctx);

  ts.next_tx(owner);
  let mut responser = ts.take_shared();

  let challenge = create_for_test(
    @0xa111,
    @0xb111,
    tx_context::epoch(ts.ctx()),
    balance::create_for_testing(minimum_gas()),
    ts.ctx(),
  );
  let id = object::id_address(&challenge);

  ts.next_tx(id);
  response(&mut responser, &challenge, ts.ctx());
  ts.next_tx(@responser);
  transfer::public_freeze_object(challenge);
  ts::return_shared(responser);
  ts.end();
}
