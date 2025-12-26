//! PCA and PoC types for PIC Protocol
pub mod pca;

pub use pca::*;

pub fn pca_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
