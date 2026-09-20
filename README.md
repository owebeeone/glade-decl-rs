# glade-decl (Rust)

The Rust **rendering** of the [`glade-decl`](https://github.com/owebeeone/glade-decl)
contract: generated native types + a deterministic-CBOR codec for the glade
declaration surface (`GladeId`, `Shape`, `Authority`, `BindingDecl`,
`AdvertisementRecord`, `ChangeEvent`, …). A leaf crate — no dependencies beyond
its own vendored CBOR runtime — so grip-core / glial can type the declaration
surface without importing glade or glial.

Generated files (`src/api.rs`, `src/cbor.rs`, `src/ext.rs`, `src/vectors.rs`)
carry a `do not edit` header. Regenerate from the contract; never hand-edit.

## Corpus gate

`src/vectors.rs` embeds the contract's golden corpus and, for every vector,
decodes then re-encodes through this crate's codec and asserts the bytes match
(`parity == correctness`). It is pinned to a contract commit
(`CONTRACT_VERSION` in `src/lib.rs`).

```sh
cargo test        # runs vectors::conformance::corpus_byte_parity
```

## Regenerate

From the `glade-decl` contract repo (a gwz sibling):

```sh
PYTHONPATH=../taut/src python3 -m taut.cli gen ir/glade_decl.taut.py \
    -o /tmp/g -l rust --api-only --with-runtime      # -> api.rs, cbor.rs, ext.rs
cp /tmp/g/rust/*.rs ../glade-decl-rs/src/
python3 corpus/build.py                              # rewrites ../glade-decl-rs/src/vectors.rs
cd ../glade-decl-rs && cargo fmt                     # api.rs: tautc does not format its output
```

Generated Rust is committed **formatted**. `build.py` runs `rustfmt` over
`src/vectors.rs` itself — on write and on `--check` alike, so `rustfmt` is
required by that gate — but `src/api.rs` comes from the taut compiler, so
`cargo fmt` is the step that brings it to the same convention. `cargo fmt
--check` must be clean here.

Design: `glade-decl/dev-docs/DeclSurface.md`.
