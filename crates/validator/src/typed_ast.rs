use parser::ast::{Else, Expr, IF};


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
enum Value {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
}

pub enum Type {
    Integer,
    Float,
    Bool,
    String,
    Char,
    Void,
    Any,
    Function,
    Struct(String),
    Union(String),
    Enum(String),
}

pub enum Statement {
    VariableDecl {
        name: String,
        typ: Type,
        is_mutable: bool,
        value: Option<Value>,
    },
    Condition {
        main: IF,
        elif: Vec<IF>,
        other: Option<Else>,
    },
    Loop {
        var_name: String,
        iterable: Expr,
        body: Vec<Statement>,
    },
    Assignment {
        name: String,
        value: Value,
    },
    FunctionDef {
        name: String,
        params: Vec<(String, Type)>,
        body: Vec<Statement>,
        return_type: Option<Type>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, Type)>,
    },
    UnionDef {
        name: String,
        fields: Vec<(String, Type)>,
    },
}


/* 
#[derive(Debug, Clone)]
pub enum Expr {
    DynamicString(Rc<String>),
    Void,
    Return(Box<Expr>),
    Time(std::time::Instant),
    String(String),
    Bool(bool),
    Number(i64),
    Char(char),
    EnumDecl {
        name: String,
        variants: Vec<String>,
    },
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
    Loop {
        var_name: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    Float(f64),
    Decl {
        name: String,
        typ: Rc<Type>,
        is_mutable: bool,
        value: Box<Expr>,
    },
    VariableRef {
        name: String,
        symbol: Option<Symbol>,
    },
    TemplateString(Vec<TemplateChunk>),
    Condition {
        main: IF,
        elif: Vec<IF>,
        other: Option<Else>,
    },
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Expr>,
        return_type: Type,
    },
    Call {
        target: Option<Box<Expr>>,
        name: Box<Expr>,
        args: Vec<Expr>,
        returned_type: Option<Type>,
    },
    FunctionDef {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Expr>,
        return_type: Option<Type>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, Type, Option<Expr>)>,
        methods: Vec<MethodType>,
    },

    UnionType {
        name: String,
        fields: Vec<(String, Type)>,
        methods: Vec<MethodType>,
    },
    StructInit {
        name: String,
        args: Vec<(String, Expr)>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
        symbol: Option<Symbol>,
    },
    BinaryOp {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Operation,
        return_type: Type,
    },
    Break,
    Continue,
    Match {
        target: Box<Expr>,
        arms: Vec<(Expr, Vec<Expr>)>,
    },
}

impl Expr {
    pub fn as_number(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            _ => 0,
        }
    }
    pub fn as_float(&self) -> f64 {
        match self {
            Expr::Float(f) => *f,
            _ => 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub typ: Type,
    pub is_mutable: bool,
    pub is_pointer: bool,
    pub is_used: bool,
    pub is_changed: bool,
}

#[derive(Debug, Clone)]
pub enum TemplateChunk {
    Literal(String),
    Expr(Box<Expr>),
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
    pub is_mutable: bool,
    pub is_pointer: bool,
}
 */