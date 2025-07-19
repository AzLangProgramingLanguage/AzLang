use std::collections::{HashMap, HashSet};

use crate::parser::ast::Program;
mod builtinfunctions;
mod codegen;
mod helpers;
mod transpile;
#[derive(Clone, Debug, Default)]

pub struct TranspileContext {
    pub imports: HashSet<String>,
    pub current_struct: Option<String>,
    pub needs_allocator: bool,
    pub uses_stdout: bool,
    pub used_min_fn: bool,
    pub used_max_fn: bool,
    pub enum_defs: HashMap<String, Vec<String>>,
    pub used_input_fn: bool,
    pub cleanup_statements: Vec<String>,
    pub used_sum_fn: bool,
    pub used_split_n_fn: bool,
    pub struct_defs: HashMap<String, Vec<String>>,
    pub used_split_auto_fn: bool,
    pub used_split_alloc_fn: bool,
    pub is_find_method: bool,
}

impl TranspileContext {
    pub fn new() -> Self {
        Self {
            imports: HashSet::new(),
            current_struct: None,
            needs_allocator: false,
            uses_stdout: false,
            used_max_fn: false,
            used_min_fn: false,
            used_input_fn: false,
            used_split_alloc_fn: false,
            enum_defs: HashMap::new(),
            cleanup_statements: Vec::new(),
            struct_defs: HashMap::new(),
            used_sum_fn: false,
            used_split_n_fn: false,
            used_split_auto_fn: false,
            is_find_method: false,
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
    pub fn transpile(&mut self, program: &Program) -> String {
        let imports = codegen::prelude::generate_imports(self);
        let defs = codegen::top_level::generate_top_level_defs(program, self);
        let main_body = codegen::main_body::generate_main_body(program, self);
        let utils = codegen::utils_fn::generate_util_functions(self);

        let allocator = if self.needs_allocator {
            "    var gpa = std.heap.GeneralPurposeAllocator(.{}){};\n    const allocator = gpa.allocator();\n"
        } else {
            ""
        };

        let cleanup: String = self
            .cleanup_statements
            .iter()
            .map(|s| format!("    {s}\n"))
            .collect();

        format!(
            r#"{imports}
{defs}
{utils}

pub fn main() !void {{
{allocator}{main_body}{cleanup}}}
"#,
        )
    }
}
