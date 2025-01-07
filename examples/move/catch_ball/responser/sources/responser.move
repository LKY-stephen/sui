module responser::responser;

use challenger::challenger::Challenge;
use sui::balance::Balance;
use sui::coin::Coin;
use sui::sui::SUI;
use talus::auto_tx::{create_auto_tx, create_object_Arg, create_type, minimum_gas, create_pure_Arg};

#[error]
const InsufficientBalance: vector<u8> = b"Insufficient Balance";

#[error]
const InvalidCaller: vector<u8> = b"Invalid Caller";

public struct Responser has key, store {
  id: UID,
  owner: address,
  balance: Balance<SUI>,
}

entry fun initiate(deposit: Coin<SUI>, ctx: &mut TxContext) {
  assert!(deposit.value() > minimum_gas(), InsufficientBalance);
  let responser = Responser {
    id: object::new(ctx),
    owner: ctx.sender(),
    balance: deposit.into_balance(),
  };

  transfer::share_object(responser);
}

// merge the deposit and reward.
entry fun merge(responser: &mut Responser, coin: Coin<SUI>, _ctx: &mut TxContext) {
  responser.balance.join(coin.into_balance());
}

entry fun response(responser: &mut Responser, challenge: &Challenge, ctx: &mut TxContext) {
  let min_gas = minimum_gas();

  // responser id
  let challenger_id = object::id_to_address(&object::id(challenge));

  assert!(challenger_id == tx_context::sender(ctx), InvalidCaller);
  let bet_value = challenge.get_bet_value();
  assert!(responser.balance.value() >= (bet_value + min_gas), InsufficientBalance);
  let mut bet = responser.balance.split(bet_value+min_gas);
  let gas = bet.split(min_gas);
  let bet_coin = bet.into_coin(ctx);
  let gas_coin = gas.into_coin(ctx);
  let mut inputs = vector::empty();
  let mut arguments = vector::empty();
  // random
  inputs.push_back(create_type(0));
  arguments.push_back(create_object_Arg(@0x8, false, false));

  // challenge
  inputs.push_back(create_type(0));
  arguments.push_back(create_object_Arg(object::id_address(challenge), true, false));
  // answer
  inputs.push_back(create_type(1));
  arguments.push_back(create_pure_Arg(vector::singleton(1)));
  // bet
  inputs.push_back(create_type(0));
  arguments.push_back(create_object_Arg(object::id_address(&bet_coin), true, true));

  // responser id
  let responser_id = object::id_to_address(&object::id(responser));

  transfer::public_transfer(bet_coin, responser_id);

  create_auto_tx(
    tx_context::epoch_timestamp_ms(ctx),
    responser_id,
    @challenger,
    b"challenger".to_string(),
    b"settle".to_string(),
    inputs,
    gas_coin,
    arguments,
    ctx,
  );
}

entry fun redeem(responser: &mut Responser, amount: u64, _ctx: &mut TxContext) {
  assert!(responser.owner == tx_context::sender(_ctx), InvalidCaller);
  assert!(responser.balance.value() >= amount, InsufficientBalance);
  let coin = responser.balance.split(amount).into_coin(_ctx);
  transfer::public_transfer(coin, responser.owner);
}
