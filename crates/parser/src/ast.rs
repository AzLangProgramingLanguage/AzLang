use crate::shared_ast::Type;
use std::{fmt::Display, rc::Rc};
pub use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct MethodType {
    pub name: Atom,
    pub params: Vec<Parameter>,
    pub body: Vec<Expr>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Not,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}
impl Operation {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
            Operation::Modulo => "%",
            Operation::Not => "!",
            Operation::Equal => "==",
            Operation::NotEqual => "!=",
            Operation::Greater => ">",
            Operation::GreaterEqual => ">=",
            Operation::Less => "<",
            Operation::LessEqual => "<=",
            Operation::And => "&&",
            Operation::Or => "||",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IF {
    pub condition: Box<Expr>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Else {
    pub body: Vec<Statement>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub return_type: Option<Type>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    EnumDecl {
        name: Atom,
        variants: Vec<Atom>,
    },
    Decl {
        name: Atom,
        typ: Rc<Type>,
        is_mutable: bool,
        value: Box<Expr>,
    },
    FunctionDef {
        name: Atom,
        return_typ: Type,
        params: Vec<Parameter>,
        body: Vec<Statement>,
    },
    ExternalFunctionDef {
        name: Atom,
        return_typ: Type,
        params: Vec<Parameter>,
        library: Atom,
        symbol: Atom,
        link_name: Option<Atom>,
    },
    StructDef {
        name: Atom,
        fields: Vec<(Atom, Type, Option<Expr>)>,
        methods: Vec<MethodType>,
    },

    UnionType {
        name: Atom,
        fields: Vec<(Atom, Type)>,
        methods: Vec<MethodType>,
    },
    Assignment {
        name: Atom,
        value: Box<Expr>,
    },
    Match {
        target: Box<Expr>,
        arms: Vec<(Expr, Vec<Expr>)>,
    },
    Condition {
        main: IF,
        elif: Vec<IF>,
        other: Option<Else>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Statement>,
    },
    Loop {
        var_name: Atom,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    DynamicString(Rc<String>),
    Void,
    Return(Box<Expr>),
    Time(std::time::Instant),
    String(Atom),
    Bool(bool),
    Number(i64),
    Char(char),
    Comment(String),
    List(Vec<Expr>),
    UnaryOp {
        op: Operation,
        expr: Box<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
        target_type: Type,
    },

    Float(f64),
    VariableRef {
        name: Atom,
        symbol: Option<Symbol>,
    },
    TemplateString(Vec<TemplateChunk>),

    Call {
        target: Option<Box<Expr>>,
        name: Box<Expr>,
        args: Vec<Expr>,
    },

    StructInit {
        name: Atom,
        args: Vec<(Atom, Expr)>,
    },

    BinaryOp {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Operation,
    },
    Break,
    Continue,
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::String(s) => write!(f, "\"{s}\""),
            Expr::Bool(b) => write!(f, "{b}"),
            Expr::Char(c) => write!(f, "'{c}'"),
            other => write!(f, "{other:?}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    pub typ: Type,
    pub is_mutable: bool,
    pub is_used: bool,
    pub is_changed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateChunk {
    Literal(String),
    Expr(Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub name: Atom,
    pub typ: Type,
    pub is_pointer: bool,
}
