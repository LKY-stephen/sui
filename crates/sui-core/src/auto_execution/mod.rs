use std::path::Path;
use std::sync::Arc;
use sui_types::auto_executable_transaction::AutoExecutableTransaction;
use sui_types::error::SuiResult;
use typed_store::rocks::{default_db_options, DBMap, DBOptions, MetricConf};
use typed_store::traits::{TableSummary, TypedStoreDebug};
use typed_store::{DBMapUtils, Map};
pub type StateTimestampMs = u64;

#[derive(DBMapUtils)]
pub struct AutoExecutionStore {
    #[default_options_override_fn = "auto_execution_table_default_config"]
    table: DBMap<StateTimestampMs, Vec<AutoExecutableTransaction>>,
}

fn auto_execution_table_default_config() -> DBOptions {
    default_db_options().optimize_for_point_lookup(64)
}

impl AutoExecutionStore {
    pub fn new(path: &Path) -> Arc<Self> {
        Arc::new(Self::open_tables_read_write(
            path.to_path_buf(),
            MetricConf::new("Auto_Execution_Transactions"),
            None,
            None,
        ))
    }

    #[cfg(test)]
    pub fn new_for_test() -> Arc<Self> {
        use tempfile::tempdir;

        let tempdir = tempdir().unwrap();
        AutoExecutionStore::new(tempdir.path())
    }

    pub fn insert_transactions(
        &self,
        timestamp: StateTimestampMs,
        transactions: Vec<AutoExecutableTransaction>,
    ) {
        // Check if the database contains the key
        if let Some(mut existing_transactions) = self.table.get(&timestamp).unwrap() {
            existing_transactions.extend(transactions);
            self.table
                .insert(&timestamp, &existing_transactions)
                .unwrap();
            return;
        }

        // Insert the new transactions into the cache and database
        self.table.insert(&timestamp, &transactions).unwrap();
    }

    /// Returns the transactions triggered at (t1,t2]
    pub fn query_transactions(
        &self,
        t1: StateTimestampMs,
        t2: StateTimestampMs,
    ) -> SuiResult<Vec<AutoExecutableTransaction>> {
        let mut result = Vec::new();

        self.table.safe_range_iter(t1 + 1..=t2).for_each(|res| {
            if let Ok((_, txs)) = res {
                result.extend(txs);
            }
        });

        Ok(result)
    }

    pub fn remove_transactions(&self, t1: StateTimestampMs, t2: StateTimestampMs) -> SuiResult {
        // Check the database for any keys that might not be in the cache
        self.table.multi_remove(t1 + 1..=t2)?;

        Ok(())
    }
}
