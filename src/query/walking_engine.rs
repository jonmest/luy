use tree_sitter::{Node, Tree, TreeCursor};

use crate::{
    ir::{Field, Filter, Kind, Pattern, Predicate, Query},
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

pub struct QueryContext<'a> {
    node: &'a Node<'a>,
    source: &'a str,
}

fn eval_query(query: &Query, node: Node, source: &str) -> bool {
    let ctx = QueryContext {
        node: &node,
        source,
    };
    match query {
        Query::Pattern(pattern) => matches_pattern(pattern, &ctx),
        _ => todo!(),
    }
}

fn matches_pattern(pattern: &Pattern, ctx: &QueryContext) -> bool {
    if !matches_kind(&pattern.kind, *ctx.node) {
        return false;
    }
    pattern
        .filters
        .iter()
        .all(|filter| matches_filter(filter, ctx))
}

fn matches_kind(kind: &Kind, node: Node) -> bool {
    match kind {
        Kind::Function => {
            node.kind() == "function_item"
                || node.kind() == "function"
                || node.kind() == "function_declaration"
                || node.kind() == "function_definition"
        }
        Kind::Var => node.kind() == "let_declaration" || node.kind() == "variable_declarator",
        Kind::Comment => node.kind() == "comment",
    }
}

fn get_field_text(node: Node, field: &Field, source: &str) -> Option<String> {
    let child = match field {
        Field::Name => node.child_by_field_name("name"),
        Field::Param => node.child_by_field_name("parameters"),
        Field::Body => node.child_by_field_name("body"),
        _ => None,
    }?;

    child.utf8_text(source.as_bytes()).ok().map(str::to_string)
}

fn matches_filter(filter: &Filter, ctx: &QueryContext) -> bool {
    let Some(text) = get_field_text(*ctx.node, &filter.field, ctx.source) else {
        return false;
    };

    matches_predicate(&filter.predicate, &text, ctx)
}

fn matches_predicate(predicate: &Predicate, text: &str, ctx: &QueryContext) -> bool {
    match predicate {
        Predicate::Eq(value) => text == value,
        Predicate::ContainsText(value) => {
            println!("Actual {} expcted {}", text, value.text);
            text.contains(&value.text)
        }
        Predicate::ContainsPattern(pattern) => {
            let mut cursor = ctx.node.walk();
            ctx.node.children(&mut cursor).any(|c| {
                /* println!(
                    "Node {:?},\n pattern {:?}\n {:?}\n\n",
                    &ctx.source[c.start_byte()..c.end_byte()],
                    pattern,
                    c.kind()
                ); */
                let sub_ctx = QueryContext {
                    node: &c,
                    source: ctx.source,
                };
                matches_pattern(pattern, &sub_ctx)
            })
        }
        Predicate::Matches(_) => todo!("regex crate"),
        Predicate::Any(inner) => {
            // later: apply inner to children/items inside a field
            matches_predicate(inner, text, ctx)
        }
        Predicate::All(inner) => {
            // later
            matches_predicate(inner, text, ctx)
        }
    }
}

fn collect_matches(query: &Query, node: Node, source: &str, out: &mut Vec<Match>) {
    if eval_query(query, node, source) {
        out.push(Match {
            kind: node.kind().to_string(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            start_row: node.start_position().row,
            start_col: node.start_position().column,
            text: source[node.start_byte()..node.end_byte()].to_string(),
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_matches(query, child, source, out);
    }
}
