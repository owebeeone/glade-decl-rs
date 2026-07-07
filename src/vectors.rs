// GENERATED conformance vectors + byte-parity test (tautc corpus) — do not edit.
// Requires the crate root to re-export its taut types + `Cbor`/`encode`/`decode`,
// e.g. `pub use generated::*; pub use cbor::{Cbor, encode, decode};`.
#![allow(dead_code)]

#[rustfmt::skip]
pub static VECTORS: &[(&str, &str, &str)] = &[
    ("AdvertisementRecord", "AdvertisementRecord", "a301a701a10162733102020301046273340502060007a201010219012c0262733203627333"),
    ("BindingDecl", "BindingDecl", "a701a10162733102020301046273340502060007a201010219012c"),
    ("ChangeEvent", "ChangeEvent", "a601a1016273310202030104182a05a2016273310219012c0643060102"),
    ("GladeId", "GladeId", "a101627331"),
    ("GladeIdManifest", "GladeIdManifest", "a3016273310262733203a101627331"),
    ("OriginMeta", "OriginMeta", "a2016273310219012c"),
    ("Retention", "Retention", "a201010219012c"),
    ("edge/advert", "AdvertisementRecord", "a301a701a10168646f632e626f64790201030004f60501060007a2010102f6026a6772617a656c2d617070036b646f633a34322f626f6479"),
    ("edge/binding-account-commons", "BindingDecl", "a701a1016d616363742e73657474696e67730200030004f60500060007a2010002f6"),
    ("edge/binding-deployment-window", "BindingDecl", "a701a1016a7379732e6e6f746963650205030004f60502060007a2010002f6"),
    ("edge/binding-doc-commons-log", "BindingDecl", "a701a10168646f632e626f64790201030004f60501060007a2010102f6"),
    ("edge/binding-exchange", "BindingDecl", "a701a1016a6772617a656c2e72756e0204030004f60501060007a2010002f6"),
    ("edge/binding-external-private-ttl", "BindingDecl", "a701a1016a646f632e637572736f720203030104656d6574656f0501060107a2010202191388"),
    ("edge/binding-message-private", "BindingDecl", "a701a10168646f632e636861740202030004f60501060107a2010102f6"),
    ("edge/change-delta", "ChangeEvent", "a601a10168646f632e626f64790201030104182905a201666e6f64652d61020c0648617070656e646564"),
    ("edge/change-min", "ChangeEvent", "a601a1016d616363742e73657474696e67730200030004f605f60640"),
    ("edge/change-refresh", "ChangeEvent", "a601a10168646f632e626f64790201030004f605a201666e6f64652d61020c0643010203"),
    ("edge/gladeid", "GladeId", "a101716772617a656c2e776f726b737061636573"),
    ("edge/gladeid-unicode", "GladeId", "a10171636166c3a92e6e6f74c495732ef09f9880"),
    ("edge/manifest", "GladeIdManifest", "a3016a6772617a656c2d617070026b646f633a34322f626f647903a10168646f632e626f6479"),
    ("edge/origin-meta", "OriginMeta", "a201666e6f64652d61020c"),
    ("edge/retention-cursor", "Retention", "a2010102f6"),
    ("edge/retention-latest", "Retention", "a2010002f6"),
    ("edge/retention-ttl", "Retention", "a2010202197530"),
];

/// Decode->re-encode dispatch by message name, over this crate's generated types.
pub fn reencode(message: &str, c: &crate::Cbor) -> crate::Cbor {
    match message {
        "AdvertisementRecord" => crate::AdvertisementRecord::from_cbor(c).to_cbor(),
        "BindingDecl" => crate::BindingDecl::from_cbor(c).to_cbor(),
        "ChangeEvent" => crate::ChangeEvent::from_cbor(c).to_cbor(),
        "GladeId" => crate::GladeId::from_cbor(c).to_cbor(),
        "GladeIdManifest" => crate::GladeIdManifest::from_cbor(c).to_cbor(),
        "OriginMeta" => crate::OriginMeta::from_cbor(c).to_cbor(),
        "Retention" => crate::Retention::from_cbor(c).to_cbor(),
        other => panic!("reencode: unknown message {other}"),
    }
}

#[cfg(test)]
mod conformance {
    use super::reencode;
    fn unhex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }
    fn hexof(b: &[u8]) -> String {
        use std::fmt::Write as _;
        b.iter().fold(String::new(), |mut s, x| { let _ = write!(s, "{x:02x}"); s })
    }
    /// Parity == correctness: every golden vector (bytes from taut's Python codec)
    /// must decode and re-encode to the identical bytes via this crate's codec.
    #[test]
    fn corpus_byte_parity() {
        assert!(!super::VECTORS.is_empty(), "empty corpus");
        for (name, message, golden) in super::VECTORS {
            let out = hexof(&crate::encode(&reencode(message, &crate::decode(&unhex(golden)))));
            assert_eq!(&out, golden, "byte mismatch for {name} ({message})");
        }
    }
}
