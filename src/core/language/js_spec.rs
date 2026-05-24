use crate::core::{
    ir::{Field, Kind},
    language::language_spec::LanguageSpec,
};

pub struct JavaScriptSpec;

impl LanguageSpec for JavaScriptSpec {
    fn matches_kind(&self, kind: &Kind, node_kind: &str) -> bool {
        match kind {
            Kind::Function => {
                node_kind == "function_declaration"
                    || node_kind == "function"
                    || node_kind == "arrow_function"
            }
            Kind::Var => node_kind == "variable_declarator",
            Kind::Comment => node_kind == "comment",
            Kind::Call => node_kind == "call_expression",
            Kind::Param => node_kind == "formal_parameter",
            Kind::String => node_kind == "string",
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
