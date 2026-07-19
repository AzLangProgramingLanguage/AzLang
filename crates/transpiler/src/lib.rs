use crate::{
    definition::external_functions_def::transpile_external_functions,
    function_call::transpile_function_call,
    helper::{is_semicolon_needed, map_typ},
    transpile::transpile_stmt,
};
pub mod definition;

use parser::{ast::FunctionDef, shared_ast::Type};
use validator::ast::{Expr, Program};

mod function_call;
use std::collections::{HashMap, HashSet};
pub mod helper;
mod tests;
pub mod transpile;

use std::fmt::Write;

pub fn transpile_expr(expr: Expr, ctx: &mut TranspileContext, buf: &mut String) {
    match expr {
        Expr::String(s) => {
            write!(buf, "\"{s}\"").unwrap();
        }
        Expr::Float(f) => {
            write!(buf, "{f}").unwrap();
        }
        Expr::Number(num) => {
            write!(buf, "{num}").unwrap();
        }
        Expr::Bool(b) => {
            if b {
                buf.push_str("true");
            } else {
                buf.push_str("false");
            }
        }
        Expr::Call { name, args, .. } => {
            transpile_function_call(buf, ctx, *name, args);
        }
        Expr::VariableRef { name, .. } => {
            buf.push_str(&name);
        }
        Expr::List(exprs) => {
            write!(buf, "[{}]{} {{", exprs.len(), map_typ(&Type::Natural)).unwrap();

            for (i, expr) in exprs.into_iter().enumerate() {
                if i > 0 {
                    buf.push_str(", ");
                }
                transpile_expr(expr, ctx, buf);
            }
            buf.push('}');
        }
        Expr::BinaryOp {
            left, right, op, ..
        } => {
            transpile_expr(*left, ctx, buf);
            buf.push_str(op.as_str());
            transpile_expr(*right, ctx, buf);
        }
        other => panic!("Buraya çatmamalıydı. Burası hele hazır deyil {other:?}"),
    }
}

#[derive(Clone, Debug, Default)]
pub struct TranspileContext {
    pub has_external_any: bool,
    pub imports: HashSet<String>,
    pub externalfunctionspath: HashSet<String>,
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
        .to_string() //TODO: Use Format Bro
    }

    pub fn transpile(&mut self, program: Program) -> String {
        let mut body = String::new();
        let mut externalfunctions = String::new();

        for pat in &program.external_functions {
            transpile_external_functions(self, pat, &mut externalfunctions);
        }
        for stmt in program.expressions {
            if is_semicolon_needed(&stmt) {
                body.push_str(&transpile_stmt(stmt, self));
                body.push(';');
            } else {
                body.push_str(&transpile_stmt(stmt, self));
            }
        }
        let ifanytype = match self.has_external_any {
            true => {
                r#"const ValueTag = enum(u8) {
    void = 0,
    int = 1,
    float = 2,
    string = 3,
    bool = 4,
};

const ValueData = extern union {
    int: i64,
    float: f64,
    string: [*:0]const u8,
    bool: u8,
};

const ValueType = extern struct {
    tag: ValueTag,
    data: ValueData,
};"#
            }
            false => "",
        };

        format!(
            "
{ifanytype}
        {externalfunctions}
pub fn main() void {{
{body}
}} "
        )
    }
}
