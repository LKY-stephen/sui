// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::base_types::ObjectID;
use crate::signature_verification::VerifiedDigestCache;
use crate::{
    base_types::{EpochId, SuiAddress},
    digests::ZKLoginInputsDigest,
    error::{SuiError, SuiResult},
    signature::{AuthenticatorTrait, VerifyParams},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentMessage;
use std::hash::Hash;
use std::sync::Arc;

/// An passkey authenticator with parsed fields. See field defition below. Can be initialized from [struct RawPasskeyAuthenticator].
#[derive(
    Eq, PartialEq, Clone, Copy, Debug, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
pub struct ObjectAuthenticator {
    id: ObjectID,
}

impl ObjectAuthenticator {
    pub fn new(id: ObjectID) -> Self {
        Self { id }
    }

    /// Returns the public key of the passkey authenticator.
    pub fn get_address(&self) -> SuiAddress {
        SuiAddress::from(self.id)
    }
}

impl AuthenticatorTrait for ObjectAuthenticator {
    fn verify_user_authenticator_epoch(
        &self,
        _epoch: EpochId,
        _max_epoch_upper_bound_delta: Option<u64>,
    ) -> SuiResult {
        Ok(())
    }

    fn verify_claims<T>(
        &self,
        _intent_msg: &IntentMessage<T>,
        _author: SuiAddress,
        _aux_verify_data: &VerifyParams,
        _zklogin_inputs_cache: Arc<VerifiedDigestCache<ZKLoginInputsDigest>>,
    ) -> SuiResult
    where
        T: Serialize,
    {
        // Check if author is derived from the public key.
        return Err(SuiError::InvalidSignature {
            error: "Object authenticator should not be checked".to_string(),
        });
    }
}

impl AsRef<[u8]> for ObjectAuthenticator {
    fn as_ref(&self) -> &[u8] {
        self.id.as_ref()
    }
}
