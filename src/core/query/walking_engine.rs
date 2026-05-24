use tree_sitter::{Node, Tree};

use crate::core::{
    ir::{Field, Filter, Pattern, Predicate, Query},
    language::language_spec::LanguageSpec,
    query::query_engine::{Match, QueryEngine},
};

pub struct WalkingEngine<'a> {
    pub spec: &'a dyn LanguageSpec,
}

impl<'a> QueryEngine for WalkingEngine<'a> {
    fn run(&self, query: &Query, tree: &Tree, source: &str) -> Vec<Match> {
        let mut out = Vec::new();
        let root = tree.root_node();

        collect_matches(query, self.spec, root, source, &mut out);

        out
    }
}

pub struct QueryContext<'a> {
    node: Node<'a>,
    source: &'a str,
}

fn eval_query(query: &Query, spec: &dyn LanguageSpec, node: Node, source: &str) -> bool {
    let ctx = QueryContext { node, source };
    match query {
        Query::Pattern(pattern) => matches_pattern(pattern, spec, &ctx),
        _ => todo!(),
    }
}

fn matches_pattern(pattern: &Pattern, spec: &dyn LanguageSpec, ctx: &QueryContext) -> bool {
    if !spec.matches_kind(&pattern.kind, ctx.node.kind()) {
        return false;
    }
    pattern
        .filters
        .iter()
        .all(|filter| matches_filter(filter, spec, ctx))
}

fn get_node_text(node: &Node, source: &str) -> Option<String> {
    node.utf8_text(source.as_bytes()).ok().map(str::to_string)
}

fn get_field_node<'tree>(node: Node<'tree>, field: &Field) -> Option<Node<'tree>> {
    match field {
        Field::Name => node.child_by_field_name("name"),
        Field::Params => node.child_by_field_name("parameters"),
        Field::Body => node.child_by_field_name("body"),
        Field::Self_ => Some(node),
        _ => None,
    }
}

fn matches_filter(filter: &Filter, spec: &dyn LanguageSpec, ctx: &QueryContext) -> bool {
    let Some(field_node) = get_field_node(ctx.node, &filter.field) else {
        return false;
    };

    matches_predicate(&filter.predicate, spec, field_node, ctx)
}

fn contains_pattern(node: Node, spec: &dyn LanguageSpec, pattern: &Pattern, source: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let child_ctx = QueryContext {
            node: child,
            source,
        };
        if matches_pattern(pattern, spec, &child_ctx) {
            return true;
        }
        if contains_pattern(child, spec, pattern, source) {
            return true;
        }
    }

    false
}

fn contains_text(node: &Node, expected: &str, ctx: &QueryContext) -> bool {
    let Some(text) = get_node_text(node, ctx.source) else {
        return false;
    };
    text.contains(expected)
}

fn matches_predicate(
    predicate: &Predicate,
    spec: &dyn LanguageSpec,
    node: Node,
    ctx: &QueryContext,
) -> bool {
    let Some(text) = get_node_text(&node, ctx.source) else {
        return false;
    };

    match predicate {
        Predicate::Eq(value) => &text == value,
        Predicate::ContainsText(value) => contains_text(&node, &value.text, ctx),
        Predicate::ContainsPattern(pattern) => contains_pattern(node, spec, pattern, ctx.source),
        Predicate::Matches(_) => todo!("regex crate"),
        Predicate::All(inner) => {
            // later
            matches_predicate(inner, spec, node, ctx)
        }
    }
}

fn collect_matches(
    query: &Query,
    spec: &dyn LanguageSpec,
    node: Node,
    source: &str,
    out: &mut Vec<Match>,
) {
    if eval_query(query, spec, node, source) {
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
        collect_matches(query, spec, child, source, out);
    }
}
