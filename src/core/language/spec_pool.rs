use std::collections::HashMap;

use crate::core::language::{
    lang::Lang, language_spec::LanguageSpec, rust_spec::RustSpec, ts_spec::TypeScriptSpec,
};

/*
*
* Store and reuse language-speicific language specs.
*
*/

pub struct LanguageSpecPool<'a> {
    spec: HashMap<Lang, &'a dyn LanguageSpec>,
}

impl<'a> LanguageSpecPool<'a> {
    pub fn new() -> Self {
        LanguageSpecPool {
            spec: HashMap::new(),
        }
    }
    pub fn spec_for(&mut self, lang: Lang) -> anyhow::Result<&'a dyn LanguageSpec> {
        if let std::collections::hash_map::Entry::Vacant(e) = self.spec.entry(lang) {
            let spec: &'a dyn LanguageSpec = match lang {
                Lang::Rust => &RustSpec {},
                Lang::TypeScript => &TypeScriptSpec {},
                _ => todo!(),
            };
            e.insert(spec);
        }
        Ok(*self.spec.get(&lang).unwrap())
    }
}
