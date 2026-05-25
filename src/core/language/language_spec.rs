use crate::core::ir::{Field, Kind};

pub trait LanguageSpec {
    fn matches_kind(&self, kind: &Kind, node_kind: &str) -> bool;
    fn field_names(&self, kind: &Kind, field: &Field) -> &'static [&'static str];
}
