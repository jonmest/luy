use tree_sitter::{Node, Tree};

use crate::{
    ir::{Filter, Kind, Pattern, Predicate, Query},
    query::query_engine::{Match, QueryEngine},
};

pub struct WalkingEngine {}

impl QueryEngine for WalkingEngine {
    fn run(&self, query: &Query, tree: &Tree, source: &str) -> Vec<Match> {
        let mut out = Vec::new();
        let root = tree.root_node();

        collect_matches(query, root, source, &mut out);

        out
    }
}

fn eval_query(query: &Query, node: Node, source: &str) -> bool {
    match query {
        Query::Pattern(pattern) => matches_pattern(pattern, node, source),
        _ => todo!(),
    }
}

fn matches_pattern(pattern: &Pattern, node: Node, source: &str) -> bool {
    if !matches_kind(&pattern.kind, node) {
        return false;
    }
    pattern
        .filters
        .iter()
        .all(|filter| matches_filter(filter, node, source))
}

fn matches_kind(kind: &Kind, node: Node) -> bool {
    match kind {
        Kind::Function => {
            node.kind() == "function_item"
                || node.kind() == "function_declaration"
                || node.kind() == "function_definition"
        }
        Kind::Var => node.kind() == "let_declaration" || node.kind() == "variable_declarator",
        Kind::Comment => node.kind() == "comment",
    }
}

fn get_field_text(node: Node, field: &str, source: &str) -> Option<String> {
    let child = match field {
        "name" => node.child_by_field_name("name"),
        "params" => node.child_by_field_name("parameters"),
        "body" => node.child_by_field_name("body"),
        _ => None,
    }?;

    child.utf8_text(source.as_bytes()).ok().map(str::to_string)
}

fn matches_filter(filter: &Filter, node: Node, source: &str) -> bool {
    let Some(text) = get_field_text(node, &filter.field, source) else {
        return false;
    };

    matches_predicate(&filter.predicate, &text)
}

fn matches_predicate(predicate: &Predicate, text: &str) -> bool {
    match predicate {
        Predicate::Eq(value) => text == value,
        Predicate::Like(value) => text.contains(value),
        Predicate::Grep(value) => text.contains(value),
        Predicate::Matches(_) => todo!("regex crate"),
        Predicate::Regex(_) => todo!("regex crate"),
        Predicate::Any(inner) => {
            // later: apply inner to children/items inside a field
            matches_predicate(inner, text)
        }
        Predicate::All(inner) => {
            // later
            matches_predicate(inner, text)
        }
    }
}

fn collect_matches(query: &Query, node: Node, source: &str, out: &mut Vec<Match>) {
    if eval_query(query, node, source) {
        out.push(Match {
            kind: node.kind().to_string(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_matches(query, child, source, out);
    }
}
