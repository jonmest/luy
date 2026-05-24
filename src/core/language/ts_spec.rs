use crate::core::{
    ir::{Field, Kind},
    language::language_spec::LanguageSpec,
};

pub struct TypeScriptSpec;

impl LanguageSpec for TypeScriptSpec {
    fn matches_kind(&self, kind: &Kind, node_kind: &str) -> bool {
        match kind {
            Kind::Function => matches!(
                node_kind,
                "function_declaration"
                    | "function"
                    | "arrow_function"
                    | "method_definition"
                    | "generator_function"
                    | "generator_function_declaration"
            ),

            Kind::Var => matches!(
                node_kind,
                "variable_declarator" | "lexical_declaration" | "variable_declaration"
            ),

            Kind::Comment => node_kind == "comment",

            Kind::Call => node_kind == "call_expression",

            Kind::Param => matches!(
                node_kind,
                "required_parameter" | "optional_parameter" | "formal_parameter"
            ),

            Kind::Arg => node_kind == "arguments",

            Kind::String => matches!(node_kind, "string" | "template_string"),

            Kind::Import => matches!(
                node_kind,
                "import_statement" | "import_clause" | "import_specifier"
            ),

            Kind::Type => matches!(
                node_kind,
                "type_identifier"
                    | "predefined_type"
                    | "generic_type"
                    | "object_type"
                    | "union_type"
                    | "intersection_type"
                    | "type_alias_declaration"
                    | "interface_declaration"
            ),
        }
    }

    fn field_names(&self, kind: &Kind, field: &Field) -> &'static [&'static str] {
        match (kind, field) {
            (_, Field::Self_) => &[],

            (Kind::Function, Field::Name) => &["name"],
            (Kind::Function, Field::Params) => &["parameters"],
            (Kind::Function, Field::Body) => &["body"],
            (Kind::Function, Field::ReturnType) => &["return_type"],

            (Kind::Call, Field::Name) => &["function"],
            (Kind::Call, Field::Args) => &["arguments"],

            (Kind::Param, Field::Name) => &["pattern", "name"],
            (Kind::Param, Field::Type) => &["type"],

            (Kind::Var, Field::Name) => &["name"],
            (Kind::Var, Field::Value) => &["value"],

            (Kind::Import, Field::Path) => &["source"],
            (Kind::Import, Field::Name) => &["name"],

            _ => &[],
        }
    }
}
