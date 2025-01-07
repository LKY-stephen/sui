module challenger::challenger;

use std::string::String;
use sui::balance::Balance;
use sui::coin::Coin;
use sui::random::{Self, Random};
use sui::sui::SUI;
use talus::auto_tx;

#[error]
const InvalidResponser: vector<u8> = b"Invalid Responser";

#[error]
const InsufficientGas: vector<u8> = b"Insufficient Gas";

public struct Challenge has key, store {
  id: UID,
  owner: address,
  responser: address,
  epoch: u64,
  bet: Balance<SUI>,
}

public fun get_bet_value(self: &Challenge): u64 {
  self.bet.value()
}

entry fun settle(
  r: &Random,
  challenge: Challenge,
  answer: bool,
  bet2: Coin<SUI>,
  ctx: &mut TxContext,
) {
  assert!(ctx.sender() == challenge.responser, InvalidResponser);

  let mut generator = random::new_generator(r, ctx);

  let Challenge { id, owner, responser, epoch, bet } = challenge;
  let reward = bet.into_coin(ctx);
  if (epoch == tx_context::epoch(ctx)&&answer== generator.generate_bool()) {
    transfer::public_transfer(reward, responser);
    transfer::public_transfer(bet2, responser);
  } else {
    transfer::public_transfer(reward, owner);
    transfer::public_transfer(bet2, owner);
  };
  object::delete(id);
}

entry fun create_challenge(
  package: address,
  target: address,
  module_name: String,
  entry_name: String,
  gas: Coin<SUI>,
  bet: Coin<SUI>,
  ctx: &mut TxContext,
) {
  let challenge = Challenge {
    id: object::new(ctx),
    owner: tx_context::sender(ctx),
    responser: target,
    epoch: tx_context::epoch(ctx),
    bet: bet.into_balance(),
  };

  assert!(gas.balance().value() >=auto_tx::minimum_gas(), InsufficientGas);

  let challenge_id = object::id_to_address(&object::id(&challenge));
  transfer::share_object(challenge);

  let mut inputs = vector::empty();
  let mut arguments = vector::empty();
  inputs.push_back(auto_tx::create_type(0));
  inputs.push_back(auto_tx::create_type(0));
  arguments.push_back(auto_tx::create_object_Arg(target, true, false));
  arguments.push_back(auto_tx::create_object_Arg(challenge_id, false, false));
  auto_tx::create_auto_tx(
    tx_context::epoch_timestamp_ms(ctx),
    challenge_id,
    package,
    module_name,
    entry_name,
    inputs,
    gas,
    arguments,
    ctx,
  );
}

#[test_only]
public fun create_for_test(
  owner: address,
  responser: address,
  epoch: u64,
  bet: Balance<SUI>,
  ctx: &mut TxContext,
): Challenge {
  Challenge {
    id: object::new(ctx),
    owner,
    responser,
    epoch,
    bet,
  }
}
