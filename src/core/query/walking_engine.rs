use tree_sitter::{Node, Tree};

use crate::core::{
    ir::{Field, Filter, Pattern, Predicate, Query},
    language::language_spec::LanguageSpec,
    query::query_engine::{Match, QueryEngine},
};

pub struct Context<'a> {
    node: Node<'a>,
    source: &'a str,
}

pub struct WalkingEngine<'a, 'b> {
    pub spec: &'a dyn LanguageSpec,
    pub query: &'b Query,
}

impl<'a, 'b> QueryEngine<'a, 'b> for WalkingEngine<'a, 'b> {
    fn new(spec: &'a dyn LanguageSpec, query: &'b Query) -> Self {
        WalkingEngine { spec, query }
    }

    fn run(&self, tree: &Tree, source: &str) -> Vec<Match> {
        let mut out = Vec::new();
        let root = tree.root_node();
        self.collect_matches(self.query, root, source, &mut out);
        out
    }
}

impl<'a, 'b> WalkingEngine<'a, 'b> {
    fn collect_matches(&self, query: &Query, node: Node, source: &str, out: &mut Vec<Match>) {
        if self.eval_query(query, node, source) {
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
            self.collect_matches(query, child, source, out);
        }
    }

    fn eval_query(&self, query: &Query, node: Node, source: &str) -> bool {
        let ctx = Context { node, source };
        match query {
            Query::Pattern(pattern) => self.matches_pattern(pattern, &ctx),
            _ => todo!(),
        }
    }

    /*
     *
     * Contains
     *
     */

    fn contains_pattern(&self, pattern: &Pattern, ctx: &Context) -> bool {
        let mut cursor = ctx.node.walk();
        for child in ctx.node.children(&mut cursor) {
            let child_ctx = Context {
                node: child,
                source: ctx.source,
            };
            if self.matches_pattern(pattern, &child_ctx) {
                return true;
            }
            if self.contains_pattern(pattern, &child_ctx) {
                return true;
            }
        }

        false
    }

    fn contains_text(&self, expected: &str, ctx: &Context) -> bool {
        let Some(text) = self.get_node_text(ctx) else {
            return false;
        };
        text.contains(expected)
    }

    /*
     *
     *   Matches
     *
     */

    fn matches_pattern(&self, pattern: &Pattern, ctx: &Context) -> bool {
        if !self.spec.matches_kind(&pattern.kind, ctx.node.kind()) {
            return false;
        }
        pattern
            .filters
            .iter()
            .all(|filter| self.matches_filter(filter, ctx))
    }

    fn matches_filter(&self, filter: &Filter, ctx: &Context) -> bool {
        let Some(field_node) = self.get_field_node(ctx.node, &filter.field) else {
            return false;
        };

        self.matches_predicate(
            &filter.predicate,
            &Context {
                node: field_node,
                source: ctx.source,
            },
        )
    }

    fn matches_predicate(&self, predicate: &Predicate, ctx: &Context) -> bool {
        let Some(text) = self.get_node_text(ctx) else {
            return false;
        };

        match predicate {
            Predicate::Eq(value) => &text == value,
            Predicate::ContainsText(value) => self.contains_text(&value.text, ctx),
            Predicate::ContainsPattern(pattern) => self.contains_pattern(pattern, ctx),
            Predicate::Matches(_) => todo!("regex crate"),
            Predicate::All(inner) => {
                // later
                self.matches_predicate(inner, ctx)
            }
        }
    }

    /*
     *
     * Helpers
     *
     */

    fn get_field_node<'tree>(&self, node: Node<'tree>, field: &Field) -> Option<Node<'tree>> {
        match field {
            Field::Name => node.child_by_field_name("name"),
            Field::Params => node.child_by_field_name("parameters"),
            Field::Body => node.child_by_field_name("body"),
            Field::Self_ => Some(node),
            _ => None,
        }
    }

    fn get_node_text(&self, ctx: &Context) -> Option<String> {
        ctx.node
            .utf8_text(ctx.source.as_bytes())
            .ok()
            .map(str::to_string)
    }
}
