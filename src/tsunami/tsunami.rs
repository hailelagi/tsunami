use anyhow;

use runtime::Term;

pub mod bag;
pub mod duplicate_bag;
pub mod ordered_set;
pub mod set;

// public :ets
pub trait Table: Send + Sync {
    fn insert(&self, process: value: Term, key_clash_fail: bool) -> Result<()>

    fn get_element(&self, process: &RcProcess, key: Term, index: usize) -> Result<Term>;

    fn member(&self, key: Term) -> bool;

    fn remove(&self, key: Term) -> Result<Term>;

    fn remove_object(&mut self, object: Term) -> Result<Term>;

    fn slot(&self, slot: Term) -> Result<Term>;
}
