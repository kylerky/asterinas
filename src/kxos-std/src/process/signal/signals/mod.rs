pub mod fault;
pub mod kernel;
pub mod user;

use core::fmt::Debug;

use super::{c_types::siginfo_t, sig_num::SigNum};

pub trait Signal: Send + Sync + Debug {
    /// Returns the number of the signal.
    fn num(&self) -> SigNum;
    /// Returns the siginfo_t that gives more details about a signal.
    fn to_info(&self) -> siginfo_t;
}
