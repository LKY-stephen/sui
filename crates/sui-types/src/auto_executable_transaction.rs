use crate::base_types::ObjectID;
use crate::crypto::default_hash;
use crate::digests::TransactionDigest;
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
        let move_obj = obj.data.try_as_move().expect("should be valid");
        if !move_obj.is_auto_tx() {
            return None;
        }
        return Some(AutoTx::from_bcs_bytes(move_obj.contents()).expect("Should be valid object"));
    }

    pub fn trigger_time(&self) -> TimestampMs {
        self.trigger_time
    }
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

    pub fn try_from_obj(auto_tx: &AutoTx, gas: &Object, balance: u64, price: u64) -> Option<Self> {
        let transaction = TransactionData::new_move_call_with_gas_coins(
            auto_tx.caller.into(),
            auto_tx.callee,
            Identifier::from_utf8(auto_tx.module_name.clone()).expect("Invalid module name"),
            Identifier::from_utf8(auto_tx.function_name.clone()).expect("Invalid function name"),
            bcs::from_bytes::<Vec<TypeTag>>(auto_tx.type_inputs.as_slice())
                .expect("Invalid type inputs"),
            vec![(gas.id(), gas.version(), gas.digest())],
            bcs::from_bytes::<Vec<CallArg>>(auto_tx.arguments.as_slice())
                .expect("Invalid arguments"),
            balance,
            price,
        )
        .expect("Transaction creation failed");
        let id: ObjectID = auto_tx.id.object_id().to_owned();
        if gas.get_single_owner().is_some_and(|x| x != id.into()) {
            return None;
        }
        Some(Self::new(id, 0, transaction)?)
    }

    // Returns the primary key for this transaction.
    pub fn key(&self) -> TransactionKey {
        TransactionKey::Digest(self.digest())
    }

    pub fn digest(&self) -> TransactionDigest {
        TransactionDigest::new(default_hash(self))
    }
}
