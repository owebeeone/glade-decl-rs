// GENERATED native Rust types + codec — do not edit.
#![allow(dead_code)]
use crate::cbor::{Cbor, DecodeError};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Shape {
    #[default] Value,
    Log,
    Message,
    Stream,
    Exchange,
    Window,
    Swmr,
}
impl Shape {
    pub fn wire(self) -> i64 { match self {
        Self::Value => 0,
        Self::Log => 1,
        Self::Message => 2,
        Self::Stream => 3,
        Self::Exchange => 4,
        Self::Window => 5,
        Self::Swmr => 6,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Value,
        1 => Self::Log,
        2 => Self::Message,
        3 => Self::Stream,
        4 => Self::Exchange,
        5 => Self::Window,
        6 => Self::Swmr,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "Shape", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Authority {
    #[default] Share,
    External,
}
impl Authority {
    pub fn wire(self) -> i64 { match self {
        Self::Share => 0,
        Self::External => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Share,
        1 => Self::External,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "Authority", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DomainAnchor {
    #[default] Account,
    Document,
    Deployment,
}
impl DomainAnchor {
    pub fn wire(self) -> i64 { match self {
        Self::Account => 0,
        Self::Document => 1,
        Self::Deployment => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Account,
        1 => Self::Document,
        2 => Self::Deployment,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "DomainAnchor", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ZoneKind {
    #[default] Commons,
    Private,
}
impl ZoneKind {
    pub fn wire(self) -> i64 { match self {
        Self::Commons => 0,
        Self::Private => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Commons,
        1 => Self::Private,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ZoneKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum RetentionPolicy {
    #[default] Latest,
    FromCursor,
    Ttl,
}
impl RetentionPolicy {
    pub fn wire(self) -> i64 { match self {
        Self::Latest => 0,
        Self::FromCursor => 1,
        Self::Ttl => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Latest,
        1 => Self::FromCursor,
        2 => Self::Ttl,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "RetentionPolicy", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ChangeKind {
    #[default] Refresh,
    Delta,
}
impl ChangeKind {
    pub fn wire(self) -> i64 { match self {
        Self::Refresh => 0,
        Self::Delta => 1,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        0 => Self::Refresh,
        1 => Self::Delta,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ChangeKind", value: v }),
    }) }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GladeId {
    pub id: String,
}
impl GladeId {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.id.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Retention {
    pub policy: RetentionPolicy,
    pub ttl_ms: Option<i64>,
}
impl Retention {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Int(self.policy.wire())),
            (2, match &self.ttl_ms { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            policy: RetentionPolicy::from_wire(c.try_get(1)?.try_int()?)?,
            ttl_ms: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_int()?) } },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BindingDecl {
    pub glade_id: GladeId,
    pub shape: Shape,
    pub authority: Authority,
    pub source: Option<String>,
    pub domain: DomainAnchor,
    pub zone: ZoneKind,
    pub retention: Retention,
}
impl BindingDecl {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, self.glade_id.to_cbor()),
            (2, Cbor::Int(self.shape.wire())),
            (3, Cbor::Int(self.authority.wire())),
            (4, match &self.source { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (5, Cbor::Int(self.domain.wire())),
            (6, Cbor::Int(self.zone.wire())),
            (7, self.retention.to_cbor()),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            glade_id: GladeId::from_cbor(c.try_get(1)?)?,
            shape: Shape::from_wire(c.try_get(2)?.try_int()?)?,
            authority: Authority::from_wire(c.try_get(3)?.try_int()?)?,
            source: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            domain: DomainAnchor::from_wire(c.try_get(5)?.try_int()?)?,
            zone: ZoneKind::from_wire(c.try_get(6)?.try_int()?)?,
            retention: Retention::from_cbor(c.try_get(7)?)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AdvertisementRecord {
    pub binding: BindingDecl,
    pub package: String,
    pub grip_key: String,
}
impl AdvertisementRecord {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, self.binding.to_cbor()),
            (2, Cbor::Text(self.package.clone())),
            (3, Cbor::Text(self.grip_key.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            binding: BindingDecl::from_cbor(c.try_get(1)?)?,
            package: c.try_get(2)?.try_text()?,
            grip_key: c.try_get(3)?.try_text()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OriginMeta {
    pub origin: String,
    pub seq: i64,
}
impl OriginMeta {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.origin.clone())),
            (2, Cbor::Int(self.seq)),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            origin: c.try_get(1)?.try_text()?,
            seq: c.try_get(2)?.try_int()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ChangeEvent {
    pub glade_id: GladeId,
    pub shape: Shape,
    pub kind: ChangeKind,
    pub base_seq: Option<i64>,
    pub origin_meta: Option<OriginMeta>,
    pub payload: Vec<u8>,
}
impl ChangeEvent {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, self.glade_id.to_cbor()),
            (2, Cbor::Int(self.shape.wire())),
            (3, Cbor::Int(self.kind.wire())),
            (4, match &self.base_seq { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (5, match &self.origin_meta { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (6, Cbor::Bytes(self.payload.clone())),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            glade_id: GladeId::from_cbor(c.try_get(1)?)?,
            shape: Shape::from_wire(c.try_get(2)?.try_int()?)?,
            kind: ChangeKind::from_wire(c.try_get(3)?.try_int()?)?,
            base_seq: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            origin_meta: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(OriginMeta::from_cbor(v)?) } },
            payload: c.try_get(6)?.try_bytes()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GladeIdManifest {
    pub package_id: String,
    pub grip_key: String,
    pub glade_id: GladeId,
}
impl GladeIdManifest {
    pub fn to_cbor(&self) -> Cbor {
        Cbor::Map(vec![
            (1, Cbor::Text(self.package_id.clone())),
            (2, Cbor::Text(self.grip_key.clone())),
            (3, self.glade_id.to_cbor()),
        ])
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            package_id: c.try_get(1)?.try_text()?,
            grip_key: c.try_get(2)?.try_text()?,
            glade_id: GladeId::from_cbor(c.try_get(3)?)?,
        })
    }
}
