use crate::base_types::ObjectID;
use crate::crypto::default_hash;
use crate::digests::TransactionDigest;
use crate::error::{SuiError, SuiResult};
use crate::event::EventID;
use crate::transaction::{TransactionData, TransactionDataAPI, TransactionKey};
use serde::{Deserialize, Serialize};

pub type TimestampMs = u64;

#[derive(Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AutoExecutableTransaction {
    event_id: EventID,
    trigger_time: TimestampMs,
    caller: ObjectID,
    transaction: TransactionData,
}

impl AutoExecutableTransaction {
    pub fn event_id(&self) -> &EventID {
        &self.event_id
    }

    pub fn trigger_time(&self) -> TimestampMs {
        self.trigger_time
    }

    pub fn caller(&self) -> &ObjectID {
        &self.caller
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
            event_id: EventID,
            trigger_time: TimestampMs,
            caller: ObjectID,
            transaction: TransactionData,
        }

        let helper = AutoExecutableTransactionHelper::deserialize(deserializer)?;
        let signer = helper.transaction.signers();
        if signer.len() != 1 || signer[0] != helper.caller.into() {
            return Err(serde::de::Error::custom(
                "AutoExecutableTransaction should have only one signer as caller",
            ));
        }

        Ok(AutoExecutableTransaction {
            event_id: helper.event_id,
            trigger_time: helper.trigger_time,
            caller: helper.caller,
            transaction: helper.transaction,
        })
    }
}

impl AutoExecutableTransaction {
    pub fn new(
        event_id: EventID,
        trigger_time: TimestampMs,
        caller: ObjectID,
        transaction: TransactionData,
    ) -> SuiResult<Self> {
        let signer = transaction.signers();
        if signer.len() != 1 || signer[0] != caller.into() {
            return Err(SuiError::InvalidSignature {
                error: "AutoExecutableTransaction should have only one signer as caller"
                    .to_string(),
            });
        }

        Ok(AutoExecutableTransaction {
            event_id,
            trigger_time,
            caller,
            transaction,
        })
    }

    // Returns the primary key for this transaction.
    pub fn key(&self) -> TransactionKey {
        TransactionKey::Digest(self.digest())
    }

    pub fn digest(&self) -> TransactionDigest {
        TransactionDigest::new(default_hash(self))
    }
}
