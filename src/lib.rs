// #![feature(unboxed_closures)]
// #![feature(fn_traits)]
#![feature(test)]

pub mod caller;
pub mod r_static;
mod pointer_;
mod types_;
mod macro_;
mod errors_;
mod sized_;
mod thread_;
mod format_;
mod enum_;
pub mod doc_;
mod closure_;
mod copy_;
mod env_;
mod spawn_async;
mod block_on_;
