use crate::{
    helper::{is_semicolon_needed, map_typ},
    transpile::transpile_stmt,
};
use parser::{
    ast::{Expr, FunctionDef, Symbol},
    shared_ast::Type,
};
use validator::ast::Program;

use std::collections::{HashMap, HashSet};
pub mod helper;
mod tests;
pub mod transpile;
pub fn transpile_expr(expr: Expr, ctx: &mut TranspileContext) -> String {
    match expr {
        Expr::String(s) => format!("\"{s}\""),
        Expr::Float(f) => format!("{f}"),
        Expr::Number(num) => num.to_string(),
        Expr::Bool(bool) => {
            if bool {
                return String::from("true");
            }
            String::from("false")
        }
        Expr::List(exprs) => {
            let mut result = format!("[{}]{} {{", exprs.len(), map_typ(&Type::Natural));
            for expr in exprs {
                result.push_str(&transpile_expr(expr, ctx));
                result.push(',');
            }
            result.pop();
            result.push('}');
            result
        }
        other => panic!("Buraya çatmamalıydı. Burası hele hazır deyil {other:?}"),
    }
}

#[derive(Clone, Debug, Default)]
pub struct TranspileContext {
    pub imports: HashSet<String>,
    pub functions: HashMap<String, FunctionDef>,
}
impl TranspileContext {
    pub fn add_import(&mut self, import: &str) -> Option<String> {
        if self.imports.contains(import) {
            None
        } else {
            self.imports.insert(import.to_string());
            Some(import.to_string())
        }
    }
    pub fn transpile_build(&self) -> String {
        r#"const std = @import("std");
pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const mod = b.addModule("bin", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });
    const exe = b.addExecutable(.{
        .name = "bin",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "bin", .module = mod },
            },
        }),
    });
    mod.addLibraryPath(.{ .cwd_relative = "." });
    mod.linkSystemLibrary("input", .{});
    mod.link_libc = true;
    b.installArtifact(exe);
    const run_step = b.step("run", "Run the app");
    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const mod_tests = b.addTest(.{
        .root_module = mod,
    });
    const run_mod_tests = b.addRunArtifact(mod_tests);
    const exe_tests = b.addTest(.{
        .root_module = exe.root_module,
    });
    const run_exe_tests = b.addRunArtifact(exe_tests);
    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
    test_step.dependOn(&run_exe_tests.step);
}"#
        .to_string()
    }

    pub fn transpile(&mut self, program: Program) -> String {
        let body = String::new();

        format!(
            "pub fn main() void {{
{body}
}} "
        )
        // let mut body = String::new();
        //
        // for stmt in program.expressions {
        //     if is_semicolon_needed(&stmt) {
        //         body.push_str(&transpile_stmt(stmt, self));
        //         body.push(';');
        //     } else {
        //         body.push_str(&transpile_stmt(stmt, self));
        //     }
        // }
        // let mut imports = self
        //     .imports
        //     .iter()
        //     .map(|s| s.as_str())
        //     .collect::<Vec<_>>()
        //     .join(";");
        // if !self.imports.is_empty() {
        //     imports.push(';');
        // }
        // format!("{imports} pub fn main() !void {{{body}}}")
    }
}
