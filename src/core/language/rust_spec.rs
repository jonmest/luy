use crate::core::{
    ir::{Field, Kind},
    language::language_spec::LanguageSpec,
};

pub struct RustSpec;

impl LanguageSpec for RustSpec {
    fn matches_kind(&self, kind: &Kind, node_kind: &str) -> bool {
        match kind {
            Kind::Function => node_kind == "function_item",
            Kind::Var => node_kind == "let_declaration",
            Kind::Comment => node_kind == "line_comment" || node_kind == "block_comment",
            Kind::Call => node_kind == "call_expression",
            Kind::Param => node_kind == "parameter",
            Kind::String => node_kind == "string_literal",
            _ => false,
        }
    }

    fn field_names(&self, kind: &Kind, field: &Field) -> &'static [&'static str] {
        match (kind, field) {
            (Kind::Function, Field::Name) => &["name"],
            (Kind::Function, Field::Params) => &["parameters"],
            (Kind::Function, Field::Body) => &["body"],

            (Kind::Call, Field::Args) => &["arguments"],
            (Kind::Call, Field::Name) => &["function"],

            _ => &[],
        }
    }
}
