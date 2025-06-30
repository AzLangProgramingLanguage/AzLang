use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]

pub struct TranspileContext {
    pub imports: HashSet<String>,
    pub current_struct: Option<String>,
    pub needs_allocator: bool,
    pub uses_stdout: bool,
    pub enum_defs: HashMap<String, Vec<String>>,
    pub used_input_fn: bool,
    pub cleanup_statements: Vec<String>,
    pub used_sum_fn: bool,
    pub used_split_n_fn: bool,
    pub used_split_auto_fn: bool,
    pub used_split_alloc_fn: bool,
}

impl TranspileContext {
    pub fn new() -> Self {
        Self {
            imports: HashSet::new(),
            current_struct: None,
            needs_allocator: false,
            uses_stdout: false,
            used_input_fn: false,
            used_split_alloc_fn: false,
            enum_defs: HashMap::new(),
            cleanup_statements: Vec::new(),
            used_sum_fn: false,
            used_split_n_fn: false,
            used_split_auto_fn: false,
        }
    }

    pub fn add_import(&mut self, import: &str) -> Option<String> {
        if self.imports.contains(import) {
            None
        } else {
            self.imports.insert(import.to_string());
            Some(import.to_string())
        }
    }
}
