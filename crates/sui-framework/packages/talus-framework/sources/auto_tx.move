module talus::auto_tx;

use std::string::{Self, String};
use sui::bcs;
use sui::coin::Coin;
use sui::sui::SUI;
use sui::transfer::public_transfer;

#[error]
const ArgMismatch: vector<u8> = b"Input types and arguments mismatch";

#[error]
const InvalidCaller: vector<u8> = b"Invalid Caller";

#[error]
const InvalidGas: vector<u8> = b"Need to reclaim the exact gas object";

#[error]
const InvalidDelete: vector<u8> = b"Cannot Delete Not Triggered AutoTx";

public enum TypeTag has drop, store {
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

public fun create_type(bits: u8): TypeTag {
  match (bits) {
    1 => TypeTag::Bool,
    3 => TypeTag::U8,
    4 => TypeTag::U16,
    5 => TypeTag::U32,
    6 => TypeTag::U64,
    7 => TypeTag::U128,
    8 => TypeTag::U256,
    _ => TypeTag::Address,
  }
}

public fun minimum_gas(): u64 {
  10_000_000
}

public struct ObjectRef has drop, store {
  id: address,
  mutable: bool,
  receiving: bool,
}

public enum CallArg has drop, store {
  // contains no structs or objects
  Pure(vector<u8>),
  // an object
  Object(ObjectRef),
}

public fun create_pure_Arg(data: vector<u8>): CallArg {
  CallArg::Pure(data)
}

public fun create_object_Arg(id: address, mutable: bool, receiving: bool): CallArg {
  CallArg::Object(ObjectRef {
    id,
    mutable,
    receiving,
  })
}

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

/// function to create an instance of the `AutoTx` struct
public fun create_auto_tx(
  trigger_time: u64,
  caller: address,
  callee: address,
  module_name: String,
  function_name: String,
  type_inputs: vector<TypeTag>,
  gas: Coin<SUI>,
  arguments: vector<CallArg>,
  ctx: &mut TxContext,
) {
  // Transfer the gas (coin) to the package address
  let gas_id = object::id_to_address(&object::id(&gas));
  public_transfer(gas, caller);
  // Ensure type_inputs and arguments have a one-to-one correspondence
  let len = vector::length(&type_inputs);

  // Ensure type_inputs and arguments have a one-to-one correspondence
  assert!(len== vector::length(&arguments), ArgMismatch);
  let mut i = 0;
  while (i < len) {
    let input = vector::borrow(&type_inputs, i);
    let arg = vector::borrow(&arguments, i);
    match (arg) {
      CallArg::Pure(_) => {
        match (input) {
          TypeTag::Vector_not_supported => { assert!(false, ArgMismatch); },
          TypeTag::Struct_not_supported => { assert!(false, ArgMismatch); },
          TypeTag::Signer_not_supported => { assert!(false, ArgMismatch); },
          _ => {},
        }
      },
      CallArg::Object(_) => {
        match (input) {
          TypeTag::Address => {},
          _ => { assert!(false, ArgMismatch); },
        }
      },
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
  public_transfer(auto_tx, @talus);
}

/// function to create an instance of the `AutoTx` struct
public fun drop_auto_tx(obj: AutoTx, remain: Coin<SUI>, ctx: &mut TxContext): () {
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

#[test_only]
public fun create_auto_tx_for_test(
  trigger_time: u64,
  caller: address,
  callee: address,
  module_name: vector<u8>,
  function_name: vector<u8>,
  type_inputs: vector<u8>,
  gas_id: address,
  arguments: vector<u8>,
  ctx: &mut TxContext,
): AutoTx {
  AutoTx {
    id: object::new(ctx),
    trigger_time,
    caller,
    callee,
    module_name,
    function_name,
    type_inputs,
    gas_id,
    arguments,
  }
}
