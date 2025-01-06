use crate::authority::authority_store::TriggerTime;
use crate::execution_cache::ObjectCacheRead;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Bound::{Excluded, Included};
use std::result::Result::Ok;
use std::sync::{Arc, Mutex, RwLock};
use sui_types::auto_executable_transaction::{AutoExecutableTransaction, AutoTx, MoveCallArg};
use sui_types::base_types::ObjectID;
use sui_types::error::SuiResult;
use sui_types::transaction::{CallArg, ObjectArg};
use sui_types::TypeTag;

pub struct AutoExecutionStore {
    clock: Arc<Mutex<TriggerTime>>,
    cache: Arc<dyn ObjectCacheRead>,
    map: Arc<RwLock<BTreeMap<TriggerTime, Vec<ObjectID>>>>,
    reverse: Arc<Mutex<HashMap<ObjectID, TriggerTime>>>,
}

impl AutoExecutionStore {
    pub fn new(current: Vec<(TriggerTime, ObjectID)>, cache: Arc<dyn ObjectCacheRead>) -> Self {
        Self {
            clock: Arc::new(Mutex::new(0)),
            cache,
            map: Arc::new(RwLock::new(BTreeMap::from_iter(
                current
                    .clone()
                    .into_iter()
                    .chunk_by(|(a, _)| *a)
                    .into_iter()
                    .map(|(time, group)| (time, group.map(|(_, id)| id).collect::<Vec<_>>())),
            ))),
            reverse: Arc::new(Mutex::new(HashMap::from_iter(
                current.into_iter().map(|(time, id)| (id, time)),
            ))),
        }
    }

    pub fn get_till(
        &self,
        from: TriggerTime,
        to: TriggerTime,
        gas_price: u64,
    ) -> SuiResult<Vec<AutoExecutableTransaction>> {
        let mut clock = self.clock.lock().expect("cannot lock clock");
        if to > *clock {
            // increase only, to store the fontier of triggered transactions.
            *clock = to;
        }

        let snapshot = self
            .map
            .read()
            .expect("should be able to read current auto execution store");

        return Ok(snapshot
            .range((Excluded(&from), Included(&to)))
            .into_iter()
            .map(|(_, ids)| {
                ids.into_iter()
                    .filter_map(|id| self.get_auto_tx_from_id(id, gas_price))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect());
    }

    pub fn get_auto_tx_from_id(
        &self,
        id: &ObjectID,
        gas_price: u64,
    ) -> Option<AutoExecutableTransaction> {
        // read auto_tx object
        let auto_tx = AutoTx::try_from_object(
            &self
                .cache
                .get_object(id)
                .ok()?
                .expect("Failed to read object"),
        )?;

        // read gas object
        let gas: sui_types::object::Object = self
            .cache
            .get_object(&auto_tx.gas_id)
            .ok()?
            .expect("There must be an object");
        if !gas.is_gas_coin() {
            return None;
        }
        let balance = gas.as_coin_maybe().expect("Should be a coin").value();
        // compute arguments and type_inputs
        let mut type_inputs =
            bcs::from_bytes::<Vec<TypeTag>>(auto_tx.type_inputs.as_slice()).ok()?;

        let arguments = bcs::from_bytes::<Vec<MoveCallArg>>(auto_tx.arguments.as_slice())
            .ok()?
            .into_iter()
            .zip(type_inputs.iter_mut())
            .filter_map(|(arg, input)| match arg {
                MoveCallArg::Pure(s) => Some(CallArg::Pure(s)),
                MoveCallArg::Object(value) => {
                    let read = self.cache.get_object(&value.id);
                    if read.is_err() {
                        return None;
                    }
                    let obj = read
                        .expect("Should be able to read")
                        .expect("There must be an object");

                    // modify the type tag.
                    if let Ok(tag) = obj.get_move_template_type() {
                        *input = tag;
                    } else {
                        return None;
                    }

                    if obj.is_shared() {
                        Some(CallArg::Object(ObjectArg::SharedObject {
                            id: value.id,
                            initial_shared_version: value.version,
                            mutable: value.mutable,
                        }))
                    } else if value.receiving {
                        Some(CallArg::Object(ObjectArg::Receiving(
                            obj.compute_object_reference(),
                        )))
                    } else {
                        Some(CallArg::Object(ObjectArg::ImmOrOwnedObject(
                            obj.compute_object_reference(),
                        )))
                    }
                }
            })
            .collect::<Vec<_>>();
        if arguments.len() != auto_tx.arguments.len() {
            // corrupted arguments, skip this transaction.
            return None;
        }
        let tx = AutoExecutableTransaction::try_from_obj(
            &auto_tx,
            &gas,
            balance,
            gas_price,
            type_inputs,
            arguments,
        )?;
        Some(tx)
    }

    pub fn update(
        &self,
        to_add: Vec<(TriggerTime, ObjectID)>,
        to_update: Vec<(TriggerTime, ObjectID)>,
        to_delete: Vec<ObjectID>,
    ) {
        let mut snapshot = self
            .map
            .write()
            .expect("cannot write to current auto execution store");
        let mut reverse = self
            .reverse
            .lock()
            .expect("cannot write to current auto execution store");

        // remove old objects

        to_delete
            .into_iter()
            .chain(to_update.clone().into_iter().map(|(_, id)| id))
            .map(|id| (reverse.remove(&id).expect("should have a time"), id))
            .into_group_map()
            .into_iter()
            .for_each(|(time, ids)| {
                let value = snapshot.get_mut(&time).expect("should have a time");
                if value.len() == ids.len() {
                    // TODO: optimistic way, may be wrong.
                    snapshot.remove(&time);
                } else {
                    let dict: HashSet<ObjectID> = HashSet::from_iter(ids.into_iter());
                    value.retain(|x| !dict.contains(x));
                }
            });

        // add new objects
        let clock = self.clock.lock().expect("cannot lock clock");
        to_add
            .into_iter()
            .chain(to_update.into_iter())
            .map(|(t, id)| {
                if t < *clock {
                    // cannot create a new task for old time
                    (*clock, id)
                } else {
                    (t, id)
                }
            })
            .into_group_map()
            .into_iter()
            .for_each(|(time, ids)| {
                if snapshot.contains_key(&time) {
                    snapshot
                        .get_mut(&time)
                        .expect("should have a time")
                        .extend(ids);
                } else {
                    snapshot.insert(time, ids);
                }
            });
    }
}
