//! glade-decl — the glade declaration surface, as generated Rust types.
//!
//! This crate is a **rendering** of the `glade-decl` contract (a taut schema):
//! generated native types + a deterministic-CBOR codec, gated byte-for-byte
//! against the contract's golden corpus. It is a LEAF — it depends on nothing
//! but its own vendored CBOR runtime, so grip-core / glial can import the
//! declaration surface without pulling in glade or glial.
//!
//! Everything under `api`/`cbor`/`ext`/`vectors` is GENERATED — do not edit.
//! Regenerate from the contract repo:
//! ```text
//! # glade-decl-ts/-rs/-py are siblings of glade-decl in the gwz workspace
//! PYTHONPATH=../taut/src python3 -m taut.cli gen ../glade-decl/ir/glade_decl.taut.py \
//!     -o <out> -l rust --api-only --with-runtime      # api.rs + cbor.rs + ext.rs
//! (cd ../glade-decl && python3 corpus/build.py)        # regenerates src/vectors.rs
//! ```

extern crate alloc;

pub mod api;
pub mod cbor;
pub mod ext;

// Re-export the generated types + codec at the crate root, the idiom the
// generated conformance harness (`vectors.rs`) expects
// (`crate::GladeId::from_cbor`, `crate::encode`, `crate::Cbor`).
pub use api::*;
pub use cbor::{decode, encode, Cbor};

// The byte-parity gate: decode + re-encode every golden vector through this
// crate's codec and assert the bytes match the contract oracle. Test-only.
#[cfg(test)]
mod vectors;

/// The pinned `glade-decl` contract commit this crate was generated from. CI
/// regenerates from this exact contract and runs the corpus vectors; skew fails
/// the build rather than drifting silently.
pub const CONTRACT_VERSION: &str = "99a04e0b960d03cbe92c0ec17321761eda860845";
