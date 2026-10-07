use parser::ast::Operation;
use std::collections::{HashMap, HashSet};
use validator::ast::{Ast, Else, Expr, Function, IF, Program};

const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

#[derive(Debug, Default)]
pub struct ModuleOutput {
    pub code: String,
    pub data: String,
    pub called: HashSet<String>,
}

#[derive(Debug, Default)]
pub struct Transpiler {
    prefix: String,
    label_id: u64,
    slots: HashMap<String, usize>,
    constvars: HashMap<String, Expr>,
    data: Vec<String>,
    string_labels: HashMap<String, String>,
    called: HashSet<String>,
    externals: HashSet<String>,
    breaks: Vec<String>,
    continues: Vec<String>,
    in_function: bool,
}

fn frame_bytes(slots: usize) -> usize {
    (slots * 8).div_ceil(16) * 16
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('0') => out.push('\0'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn fasm_string(s: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut run = String::new();
    for &b in s.as_bytes() {
        if (0x20..0x7f).contains(&b) {
            if b == b'"' {
                run.push_str("\"\"");
            } else {
                run.push(b as char);
            }
        } else {
            if !run.is_empty() {
                parts.push(format!("\"{run}\""));
                run.clear();
            }
            parts.push(b.to_string());
        }
    }
    if !run.is_empty() {
        parts.push(format!("\"{run}\""));
    }
    parts.push(String::from("0"));
    format!("db {}", parts.join(","))
}

impl Transpiler {
    fn fresh(&mut self, tag: &str) -> String {
        self.label_id += 1;
        format!(".{tag}{}", self.label_id)
    }

    fn slot(&mut self, name: &str) -> usize {
        if let Some(&index) = self.slots.get(name) {
            return index;
        }
        let index = self.slots.len();
        self.slots.insert(name.to_string(), index);
        index
    }

    fn intern_string(&mut self, s: &str) -> String {
        if let Some(label) = self.string_labels.get(s) {
            return label.clone();
        }
        let label = format!("{}_str{}", self.prefix, self.data.len());
        self.data.push(format!("{label} {}", fasm_string(&unescape(s))));
        self.string_labels.insert(s.to_string(), label.clone());
        label
    }

    fn reset_frame(&mut self) {
        self.slots.clear();
        self.breaks.clear();
        self.continues.clear();
    }

    fn expr(&mut self, out: &mut String, e: &Expr) {
        match e {
            Expr::Void => {}
            Expr::Number(num) => out.push_str(&format!("    mov rax, {num}\n")),
            Expr::Bool(b) => out.push_str(&format!("    mov rax, {}\n", i32::from(*b))),
            Expr::Char(c) => out.push_str(&format!("    mov rax, {}\n", *c as u32)),
            Expr::String(s) => {
                let label = self.intern_string(s);
                out.push_str(&format!("    lea rax, [{label}]\n"));
            }
            Expr::VariableRef { name, .. } => {
                if let Some(&index) = self.slots.get(name) {
                    out.push_str(&format!("    mov rax, [rbp-{}]\n", (index + 1) * 8));
                } else if let Some(value) = self.constvars.get(name).cloned() {
                    self.expr(out, &value);
                } else {
                    panic!(
                        "variable `{name}` is not accessible here; \
                         global variables cannot be referenced inside functions"
                    );
                }
            }
            Expr::BinaryOp { left, right, op, .. } => self.binary(out, left, right, *op),
            Expr::Call { name, args, .. } => self.call(out, name, args),
            Expr::Return(inner) => {
                if !self.in_function {
                    panic!("`return` outside of a function");
                }
                self.expr(out, inner);
                out.push_str("    leave\n    ret\n");
            }
            Expr::Break => {
                let target = self
                    .breaks
                    .last()
                    .cloned()
                    .unwrap_or_else(|| panic!("`break` outside of a loop"));
                out.push_str(&format!("    jmp {target}\n"));
            }
            Expr::Continue => {
                let target = self
                    .continues
                    .last()
                    .cloned()
                    .unwrap_or_else(|| panic!("`continue` outside of a loop"));
                out.push_str(&format!("    jmp {target}\n"));
            }
            Expr::Float(_) => todo!("float is not implemented yet"),
            Expr::TemplateString(_) => todo!("template string is not implemented yet"),
            Expr::List(_) => todo!("list is not implemented yet"),
        }
    }

    fn binary(&mut self, out: &mut String, left: &Expr, right: &Expr, op: Operation) {
        if matches!(left, Expr::Void) {
            match op {
                Operation::Not => {
                    self.expr(out, right);
                    out.push_str("    test rax, rax\n    sete al\n    movzx eax, al\n");
                }
                Operation::Subtract => {
                    self.expr(out, right);
                    out.push_str("    neg rax\n");
                }
                other => todo!("unary operator {other:?} is not implemented yet"),
            }
            return;
        }
        self.expr(out, left);
        out.push_str("    push rax\n");
        self.expr(out, right);
        out.push_str("    mov rcx, rax\n    pop rax\n");
        match op {
            Operation::Add => out.push_str("    add rax, rcx\n"),
            Operation::Subtract => out.push_str("    sub rax, rcx\n"),
            Operation::Multiply => out.push_str("    imul rax, rcx\n"),
            Operation::Divide => out.push_str("    cqo\n    idiv rcx\n"),
            Operation::Modulo => out.push_str("    cqo\n    idiv rcx\n    mov rax, rdx\n"),
            Operation::Equal => setcc(out, "sete"),
            Operation::NotEqual => setcc(out, "setne"),
            Operation::Greater => setcc(out, "setg"),
            Operation::GreaterEqual => setcc(out, "setge"),
            Operation::Less => setcc(out, "setl"),
            Operation::LessEqual => setcc(out, "setle"),
            Operation::And => out.push_str("    and rax, rcx\n"),
            Operation::Or => out.push_str("    or rax, rcx\n"),
            Operation::Not => todo!("unary not cannot be applied here"),
        }
    }

    fn call(&mut self, out: &mut String, name: &Expr, args: &[Expr]) {
        let func = match name {
            Expr::VariableRef { name, .. } => name.clone(),
            other => panic!("cannot call {other:?}"),
        };
        self.called.insert(func.clone());
        let target = if self.externals.contains(&func) {
            func
        } else {
            format!("azfn_{func}")
        };

        let count = args.len();
        let stack_args = count.saturating_sub(6);
        let pad = stack_args & 1;

        if pad == 1 {
            out.push_str("    push rax\n");
        }
        if stack_args == 0 {
            for arg in args {
                self.expr(out, arg);
                out.push_str("    push rax\n");
            }
            for i in (0..count).rev() {
                out.push_str(&format!("    pop {}\n", ARG_REGS[i]));
            }
        } else {
            for arg in args.iter().rev() {
                self.expr(out, arg);
                out.push_str("    push rax\n");
            }
            for reg in ARG_REGS {
                out.push_str(&format!("    pop {reg}\n"));
            }
        }
        out.push_str(&format!("    call {target}\n"));
        if stack_args > 0 {
            out.push_str(&format!("    add rsp, {}\n", (stack_args + pad) * 8));
        }
    }

    fn condition(&mut self, out: &mut String, main: IF, elif: Vec<IF>, other: Option<Else>) {
        let end = self.fresh("end");
        let mut next = self.fresh("br");

        self.expr(out, &main.condition);
        out.push_str(&format!("    test rax, rax\n    jz {next}\n"));
        self.transpile_body(out, main.body);
        out.push_str(&format!("    jmp {end}\n{next}:\n"));

        for branch in elif {
            next = self.fresh("br");
            self.expr(out, &branch.condition);
            out.push_str(&format!("    test rax, rax\n    jz {next}\n"));
            self.transpile_body(out, branch.body);
            out.push_str(&format!("    jmp {end}\n{next}:\n"));
        }

        if let Some(els) = other {
            self.transpile_body(out, els.body);
        }
        out.push_str(&format!("{end}:\n"));
    }

    pub fn transpile_body(&mut self, out: &mut String, body: Vec<Ast>) {
        for ast in body {
            match ast {
                Ast::Expr(expr) => self.expr(out, &expr),
                Ast::Decl { name, value, .. } => {
                    self.expr(out, &value);
                    let index = self.slot(&name);
                    out.push_str(&format!("    mov [rbp-{}], rax\n", (index + 1) * 8));
                }
                Ast::Assignment { name, value } => {
                    self.expr(out, &value);
                    let index = match self.slots.get(&name) {
                        Some(&index) => index,
                        None => panic!(
                            "variable `{name}` is not accessible here; \
                             global variables cannot be referenced inside functions"
                        ),
                    };
                    out.push_str(&format!("    mov [rbp-{}], rax\n", (index + 1) * 8));
                }
                Ast::Condition { main, elif, other } => self.condition(out, main, elif, other),
                Ast::While { condition, body } => {
                    let top = self.fresh("w");
                    let end = self.fresh("we");
                    self.breaks.push(end.clone());
                    self.continues.push(top.clone());
                    out.push_str(&format!("{top}:\n"));
                    self.expr(out, &condition);
                    out.push_str(&format!("    test rax, rax\n    jz {end}\n"));
                    self.transpile_body(out, body);
                    out.push_str(&format!("    jmp {top}\n{end}:\n"));
                    self.breaks.pop();
                    self.continues.pop();
                }
                Ast::Loop { body } => {
                    let top = self.fresh("l");
                    let end = self.fresh("le");
                    self.breaks.push(end.clone());
                    self.continues.push(top.clone());
                    out.push_str(&format!("{top}:\n"));
                    self.transpile_body(out, body);
                    out.push_str(&format!("    jmp {top}\n{end}:\n"));
                    self.breaks.pop();
                    self.continues.pop();
                }
                Ast::Exit(expr) => {
                    self.expr(out, &expr);
                    out.push_str("    mov edi, eax\n    mov eax, 60\n    syscall\n");
                }
                Ast::Enum { .. } => {}
            }
        }
    }

    fn function(&mut self, out: &mut String, function: Function) {
        self.reset_frame();
        self.in_function = true;

        let mut params = String::new();
        for (index, param) in function.params.iter().enumerate() {
            let slot = (self.slot(param.name.as_ref()) + 1) * 8;
            if index < ARG_REGS.len() {
                params.push_str(&format!("    mov [rbp-{slot}], {}\n", ARG_REGS[index]));
            } else {
                params.push_str(&format!(
                    "    mov rax, [rbp+{}]\n    mov [rbp-{slot}], rax\n",
                    16 + (index - ARG_REGS.len()) * 8
                ));
            }
        }

        let mut body = String::new();
        self.transpile_body(&mut body, function.body);
        let frame = frame_bytes(self.slots.len());

        out.push_str(&format!(
            "azfn_{}:\n    push rbp\n    mov rbp, rsp\n",
            function.name
        ));
        if frame > 0 {
            out.push_str(&format!("    sub rsp, {frame}\n"));
        }
        out.push_str(&params);
        out.push_str(&body);
        if !body.ends_with("    ret\n") {
            out.push_str("    leave\n    ret\n");
        }

        self.in_function = false;
    }

    pub fn transpile(
        &mut self,
        prefix: &str,
        program: Program,
        is_start: bool,
        externals: HashSet<String>,
    ) -> ModuleOutput {
        self.prefix = prefix.to_string();
        self.constvars = program.variables;
        self.externals = externals;
        let mut code = String::new();

        for function in program.functions {
            self.function(&mut code, function);
        }

        if is_start {
            self.reset_frame();
            self.in_function = false;
            let mut body = String::new();
            self.transpile_body(&mut body, program.expressions);
            let frame = frame_bytes(self.slots.len()) + 8;
            code.push_str("_start:\n    push rbp\n    mov rbp, rsp\n");
            code.push_str(&format!("    sub rsp, {frame}\n"));
            code.push_str(&body);
            code.push_str("    xor edi, edi\n    mov eax, 60\n    syscall\n");
        }

        ModuleOutput {
            code,
            data: self.data.join("\n"),
            called: std::mem::take(&mut self.called),
        }
    }
}

fn setcc(out: &mut String, cc: &str) {
    out.push_str(&format!(
        "    cmp rax, rcx\n    {cc} al\n    movzx eax, al\n"
    ));
}
