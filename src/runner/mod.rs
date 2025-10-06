use std::collections::HashMap;
mod builtin;
mod helpers;
mod runner_interpretator;
use crate::parser::ast::{Expr, MethodType, Program, Type};
use bumpalo::Bump;
mod eval;

#[derive(Debug)]
struct Variable<'a> {
    value: Expr<'a>, // artıq Rc deyil
    typ: Type<'a>,   // artıq Rc deyil
    is_mutable: bool,
}

#[derive(Debug)]
struct Method<'a> {
    name: &'a str,
    params: Vec<(String, Type<'a>)>, // bunları da istəsən arena-dan ala bilərsən
    body: Vec<Expr<'a>>,
    return_type: Option<Type<'a>>,
}

#[derive(Debug)]
pub struct StructDef<'a> {
    name: &'a str,
    fields: Vec<(&'a str, Type<'a>, Option<Expr<'a>>)>,
    methods: Vec<Method<'a>>,
}

#[derive(Debug)]
pub struct FunctionDef<'a> {
    params: Vec<(String, Type<'a>)>,
    body: Vec<Expr<'a>>, // arena-dan slice
    return_type: Type<'a>,
}

#[derive(Debug)]
pub struct UnionType<'a> {
    name: &'a str,
    fields: Vec<(&'a str, Type<'a>)>,
    methods: Vec<Method<'a>>,
}

#[derive(Debug)]
pub struct Runner<'a> {
    variables: HashMap<String, Variable<'a>>,
    structdefs: HashMap<String, StructDef<'a>>,
    functions: HashMap<String, FunctionDef<'a>>,
    uniontypes: HashMap<String, UnionType<'a>>,
    arena: &'a Bump,
}

impl<'a> Runner<'a> {
    pub fn new(arena: &'a Bump) -> Self {
        Self {
            variables: HashMap::new(),
            structdefs: HashMap::new(),
            functions: HashMap::new(),
            uniontypes: HashMap::new(),
            arena,
        }
    }

    /// Arena-da Expr yarat
    pub fn alloc_expr(&self, expr: Expr<'a>) -> &'a Expr<'a> {
        self.arena.alloc(expr)
    }

    /// Arena-da Type yarat
    pub fn alloc_type(&self, typ: Type<'a>) -> &'a Type<'a> {
        self.arena.alloc(typ)
    }

    pub fn alloc_expr_vec(&self, body: Vec<Expr<'a>>) -> &'a [Expr<'a>] {
        self.arena.alloc_slice_fill_iter(body.into_iter())
    }

    pub fn run(&mut self, program: Program<'a>) {
        for expr in program.expressions.into_iter() {
            runner_interpretator::runner_interpretator(self, expr); /* lifetime may not live long enough
            requirement occurs because of a mutable reference to `Runner<'_>`
            mutable references are invariant over their type parameter
            see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variancerustcClick for full compiler diagnostic
            mod.rs(68, 16): let's call the lifetime of this reference `'1`
            mod.rs(44, 6): lifetime `'a` defined here
             */
        }
    }
}
