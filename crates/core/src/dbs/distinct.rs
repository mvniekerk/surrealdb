use crate::ctx::Context;
use crate::dbs::Processed;
use crate::kvs::Key;
use radix_trie::Trie;
use std::default::Default;
#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
use std::sync::Arc;
#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
use tokio::sync::Mutex;

// TODO: This is currently processed in memory. In the future is should be on disk (mmap?)
type Distinct = Trie<Key, bool>;

#[derive(Default)]
pub(crate) struct SyncDistinct {
	processed: Distinct,
}

impl SyncDistinct {
	pub(super) fn new(ctx: &Context) -> Option<Self> {
		if let Some(pla) = ctx.get_query_planner() {
			if pla.requires_distinct() {
				return Some(Self::default());
			}
		}
		None
	}

	pub(super) fn check_already_processed(&mut self, pro: &Processed) -> bool {
		if let Some(key) = pro.rid.as_ref().map(|r| r.as_ref().into()) {
			if self.processed.get(&key).is_some() {
				true
			} else {
				self.processed.insert(key, true);
				false
			}
		} else {
			false
		}
	}
}

#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
#[derive(Default, Clone)]
pub(crate) struct AsyncDistinct {
	processed: Arc<Mutex<SyncDistinct>>,
}

#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
impl AsyncDistinct {
	pub(super) fn new(ctx: &Context) -> Option<Self> {
		if let Some(pla) = ctx.get_query_planner() {
			if pla.requires_distinct() {
				return Some(Self::default());
			}
		}
		None
	}

	pub(super) async fn check_already_processed(&self, pro: &Processed) -> bool {
		self.processed.lock().await.check_already_processed(pro)
	}
}
