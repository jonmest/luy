#![allow(unused)]

/**
*
* QueryEngine is the interface to implement for taking a query in LIR
* and returning a set of matches.
*
* Eventually, I would like a more intelligent engine that runs two passes:
*   1. Compile a Treesitter query from the LIR query. This would be partial
*      since luy will need to support queries that Treesitter cannot handle. Applying
*      the query returns a set of candidates.
*
*   2. Walk through the candidates and raw dow the luy specific queries.
*
* However, it is not what I'm starting out with. Why?
*
* Frankly it's just more complex and I need to figure out the partial Treesitter
* compilation. In the meantime, we're rolling a dumb engine that just walks through
* the tree.
*
*/
use tree_sitter::{Node, Tree};

use crate::ir::{Filter, Kind, Pattern, Predicate, Query};

#[derive(Debug)]
pub struct Match {
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

pub trait QueryEngine {
    fn run(&self, query: &Query, tree: &Tree, source: &str) -> Vec<Match>;
}
