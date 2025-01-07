use crate::base_types::ObjectID;
use crate::crypto::default_hash;
use crate::digests::TransactionDigest;
use crate::programmable_transaction_builder::ProgrammableTransactionBuilder;
use crate::transaction::{
    CallArg, TransactionData, TransactionDataAPI, TransactionKey, TransactionKind,
};
use crate::TALUS_FRAMEWORK_ADDRESS;
use crate::{id::UID, object::Object};
use move_core_types::annotated_value::{MoveFieldLayout, MoveStructLayout, MoveTypeLayout};
use move_core_types::ident_str;
use move_core_types::identifier::{IdentStr, Identifier};
use move_core_types::language_storage::{StructTag, TypeTag};
use serde::{Deserialize, Serialize};

pub type TimestampMs = u64;
pub const AUTO_EXECUTION_MODULE_NAME: &IdentStr = ident_str!("auto_tx");
pub const AUTO_EXECUTION_STRUCT_NAME: &IdentStr = ident_str!("AutoTx");

pub const TRIGGER_TIME: &IdentStr = ident_str!("trigger_time");
pub const CALLER: &IdentStr = ident_str!("caller");
pub const CALLEE: &IdentStr = ident_str!("callee");
pub const MODULE_NAME: &IdentStr = ident_str!("module_name");
pub const FUNCTION_NAME: &IdentStr = ident_str!("function_name");
pub const TYPE_INPUTS: &IdentStr = ident_str!("type_inputs");

pub const GAS_ID: &IdentStr = ident_str!("gas_id");
pub const GAS_SEQUENCE: &IdentStr = ident_str!("gas_sequence");
pub const GAS_DIGEST: &IdentStr = ident_str!("gas_digest");
pub const ARGUMENTS: &IdentStr = ident_str!("arguments");
pub const GAS_BUDGET: &IdentStr = ident_str!("gas_budget");
pub const GAS_PRICE: &IdentStr = ident_str!("gas_price");

pub const MINMUM_GAS: u64 = 1_000_000;

use once_cell::sync::Lazy;
pub static AUTO_EXECUTION_TYPE_PARAMS: Lazy<Vec<TypeTag>> = Lazy::new(|| {
    vec![
        // Trigger Time
        TypeTag::U64,
        // Owner, Object ID
        TypeTag::Address,
        // Pure Inputs
        TypeTag::Vector(Box::new(TypeTag::U8)),
        // Object Inputs, Ids
        TypeTag::Vector(Box::new(TypeTag::Address)),
        // Object Inputs, Sequence Numbers
        TypeTag::Vector(Box::new(TypeTag::U64)),
        // Move Call Command, Object ID
        TypeTag::Address,
        // Move Call Command, Module Name
        TypeTag::Vector(Box::new(TypeTag::U8)),
        // Move Call Command, Function Name
        TypeTag::Vector(Box::new(TypeTag::U8)),
        // Move Call Command, Type inputs
        TypeTag::Vector(Box::new(TypeTag::U8)),
        // Gas Id
        TypeTag::Address,
        // Move Call Command, Arguments
        TypeTag::Vector(Box::new(TypeTag::U8)),
    ]
});

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AutoTx {
    pub id: UID,
    pub trigger_time: TimestampMs,
    pub caller: ObjectID,
    pub callee: ObjectID,
    pub module_name: Vec<u8>,
    pub function_name: Vec<u8>,
    pub type_inputs: Vec<u8>,
    pub gas_id: ObjectID,
    pub arguments: Vec<u8>,
}

impl AutoTx {
    pub fn struct_tag() -> StructTag {
        StructTag {
            address: TALUS_FRAMEWORK_ADDRESS,
            module: AUTO_EXECUTION_MODULE_NAME.to_owned(),
            name: AUTO_EXECUTION_STRUCT_NAME.to_owned(),
            type_params: AUTO_EXECUTION_TYPE_PARAMS.clone(),
        }
    }

    pub fn is_auto_tx(s: &StructTag) -> bool {
        s.address == TALUS_FRAMEWORK_ADDRESS
            && s.module == AUTO_EXECUTION_MODULE_NAME.to_owned()
            && s.name == AUTO_EXECUTION_STRUCT_NAME.to_owned()
            && s.type_params
                .iter()
                .zip(AUTO_EXECUTION_TYPE_PARAMS.iter())
                .all(|(a, b)| a == b)
    }
    pub fn layout() -> MoveStructLayout {
        MoveStructLayout {
            type_: Self::struct_tag(),
            fields: Box::new(vec![
                MoveFieldLayout::new(TRIGGER_TIME.to_owned(), MoveTypeLayout::U64),
                MoveFieldLayout::new(CALLER.to_owned(), MoveTypeLayout::Address),
                MoveFieldLayout::new(CALLEE.to_owned(), MoveTypeLayout::Address),
                MoveFieldLayout::new(
                    MODULE_NAME.to_owned(),
                    MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
                ),
                MoveFieldLayout::new(
                    FUNCTION_NAME.to_owned(),
                    MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
                ),
                MoveFieldLayout::new(
                    TYPE_INPUTS.to_owned(),
                    MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
                ),
                MoveFieldLayout::new(GAS_ID.to_owned(), MoveTypeLayout::Address),
                MoveFieldLayout::new(
                    ARGUMENTS.to_owned(),
                    MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
                ),
            ]),
        }
    }

    /// Create a coin from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn try_from_object(obj: &Object) -> Option<Self> {
        if let Some(move_obj) = obj.data.try_as_move() {
            if move_obj.is_auto_tx() {
                return Some(
                    AutoTx::from_bcs_bytes(move_obj.contents()).expect("Should be valid object"),
                );
            }
        }

        None
    }

    pub fn trigger_time(&self) -> TimestampMs {
        self.trigger_time
    }
}

#[derive(Serialize, Deserialize)]
pub struct MoveObjectRef {
    pub id: ObjectID,
    pub mutable: bool,
    pub receiving: bool,
}

#[derive(Serialize, Deserialize)]
pub enum MoveCallArg {
    // contains no structs or objects
    Pure(Vec<u8>),
    // an object
    Object(MoveObjectRef),
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AutoExecutableTransaction {
    object_id: ObjectID,
    trigger_time: TimestampMs,
    transaction: TransactionData,
}

impl AutoExecutableTransaction {
    pub fn object_id(&self) -> ObjectID {
        self.object_id
    }
    pub fn trigger_time(&self) -> TimestampMs {
        self.trigger_time
    }
    pub fn transaction(&self) -> &TransactionData {
        &self.transaction
    }
}

impl<'de> Deserialize<'de> for AutoExecutableTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AutoExecutableTransactionHelper {
            object_id: ObjectID,
            trigger_time: TimestampMs,
            transaction: TransactionData,
        }

        let helper = AutoExecutableTransactionHelper::deserialize(deserializer)?;
        let signer = helper.transaction.signers();
        if signer.len() != 1 {
            return Err(serde::de::Error::custom(
                "AutoExecutableTransaction should have only one signer as caller",
            ));
        }

        Ok(AutoExecutableTransaction {
            object_id: helper.object_id,
            trigger_time: helper.trigger_time,
            transaction: helper.transaction,
        })
    }
}

impl AutoExecutableTransaction {
    pub fn new(
        id: ObjectID,
        trigger_time: TimestampMs,
        transaction: TransactionData,
    ) -> Option<Self> {
        match transaction.kind() {
            TransactionKind::AutonomousExecution(_) => {}
            _ => {
                return None;
            }
        }

        Some(AutoExecutableTransaction {
            object_id: id,
            trigger_time,
            transaction,
        })
    }

    pub fn try_from_obj(
        auto_tx: &AutoTx,
        gas: &Object,
        balance: u64,
        price: u64,
        type_inputs: Vec<TypeTag>,
        arguments: Vec<CallArg>,
    ) -> Option<Self> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            builder
                .move_call(
                    auto_tx.callee,
                    Identifier::from_utf8(auto_tx.module_name.clone())
                        .expect("Invalid module name"),
                    Identifier::from_utf8(auto_tx.function_name.clone())
                        .expect("Invalid function name"),
                    type_inputs,
                    arguments,
                )
                .unwrap();
            builder.finish()
        };
        let transaction = TransactionData::new_with_gas_coins(
            TransactionKind::AutonomousExecution(pt),
            auto_tx.caller.into(),
            vec![gas.compute_object_reference()],
            balance,
            price,
        );
        let id: ObjectID = auto_tx.id.object_id().to_owned();
        if gas.get_single_owner().is_some_and(|x| x != id.into()) {
            return None;
        }
        Some(Self::new(id, auto_tx.trigger_time(), transaction)?)
    }

    // Returns the primary key for this transaction.
    pub fn key(&self) -> TransactionKey {
        TransactionKey::Digest(self.digest())
    }

    pub fn digest(&self) -> TransactionDigest {
        TransactionDigest::new(default_hash(self))
    }
}

#[cfg(test)]
mod tests {

    use crate::object::{MoveObject, Owner, OBJECT_START_VERSION};

    use super::*;

    #[test]
    fn test_auto_tx_from_bcs_bytes() {
        let auto_tx = AutoTx {
            id: UID::new(ObjectID::random()),
            trigger_time: 1234567890,
            caller: ObjectID::random(),
            callee: ObjectID::random(),
            module_name: b"module".to_vec(),
            function_name: b"function".to_vec(),
            type_inputs: bcs::to_bytes(&Vec::<TypeTag>::new()).unwrap(),
            gas_id: ObjectID::random(),
            arguments: bcs::to_bytes(&Vec::<CallArg>::new()).unwrap(),
        };
        let bytes = bcs::to_bytes(&auto_tx).unwrap();
        let deserialized_auto_tx = AutoTx::from_bcs_bytes(&bytes).unwrap();
        assert_eq!(auto_tx, deserialized_auto_tx);
    }

    #[test]
    fn test_auto_tx_try_from_object() {
        let owner = Owner::AddressOwner(TALUS_FRAMEWORK_ADDRESS.clone().into());
        let id = UID::new(ObjectID::random());
        let gas = Object::with_owner_for_testing(id.object_id().to_owned().into());
        let auto_tx = AutoTx {
            id,
            trigger_time: 1234567890,
            caller: ObjectID::random(),
            callee: ObjectID::random(),
            module_name: b"module".to_vec(),
            function_name: b"function".to_vec(),
            type_inputs: bcs::to_bytes(&Vec::<TypeTag>::new()).unwrap(),
            gas_id: gas.id(),
            arguments: bcs::to_bytes(&Vec::<CallArg>::new()).unwrap(),
        };
        let bytes = bcs::to_bytes(&auto_tx).unwrap();
        let obj = Object::new_move(
            MoveObject::create_for_test(
                AutoTx::struct_tag().into(),
                false,
                OBJECT_START_VERSION,
                &bytes,
            ),
            owner,
            TransactionDigest::random(),
        );
        let deserialized_auto_tx = AutoTx::try_from_object(&obj).unwrap();
        assert_eq!(auto_tx, deserialized_auto_tx);

        let auto_executable_transaction = AutoExecutableTransaction::try_from_obj(
            &auto_tx,
            &gas,
            gas.as_coin_maybe().unwrap().balance.value(),
            1,
            vec![],
            vec![],
        );

        assert!(auto_executable_transaction.is_some());
    }
}
