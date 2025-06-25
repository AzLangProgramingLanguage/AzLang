use crate::{
    context::{Parameter, Symbol},
    lexer::Token,
};

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct MatchExpr {
    pub target: Box<Expr>,
    pub arms: Vec<(Token, Vec<Expr>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInFunction {
    Print,
    Input,
    Len,
    Number,
    Sum,
    Range,
    LastWord,
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Number(i64),
    Bool(bool),
    Break,
    Continue,
    If {
        condition: Box<Expr>,
        then_branch: Vec<Expr>,
        else_branch: Vec<Expr>, // Else və ElseIf ardıcıllığı daxil olan blok
    },
    ElseIf {
        condition: Box<Expr>,
        then_branch: Vec<Expr>,
    },

    Else {
        then_branch: Vec<Expr>,
    },
    MethodCall {
        target: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, Type)>,
        methods: Vec<(String, Vec<Parameter>, Vec<Expr>, Option<Type>)>,
    },
    EnumDecl(EnumDecl),

    StructInit {
        name: String,
        args: Vec<Expr>,
    },
    Match(Box<MatchExpr>),
    FieldAccess {
        target: Box<Expr>,
        field: String,
    },
    TemplateString(Vec<TemplateChunk>),
    Loop {
        var_name: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
        resolved_params: Vec<Parameter>, // 💡 Validator tərəfindən doldurulur
        return_type: Option<Type>,
    },
    Return(Box<Expr>),
    BuiltInCall {
        func: BuiltInFunction,
        args: Vec<Expr>,
        resolved_type: Option<Type>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
        symbol: Option<Symbol>,
    },
    MutableDecl {
        name: String,
        typ: Option<Type>,
        value: Box<Expr>,
    },
    ConstantDecl {
        name: String,
        typ: Option<Type>,
        value: Box<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    VariableRef {
        name: String,
        symbol: Option<Symbol>,
    },
    List(Vec<Expr>),
    FunctionDef {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Expr>,
        return_type: Option<Type>,
    },
}
#[derive(Debug, Clone)]
pub enum TemplateChunk {
    Literal(String),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Metn,
    Siyahi(Box<Type>),
    Istifadeci(String),
    Integer,
    BigInteger,
    LowInteger,
    Bool,
    Char,
    Void,
    Any,
}

impl Expr {
    pub fn children(&self) -> Vec<&Expr> {
        match self {
            Expr::String(_)
            | Expr::Number(_)
            | Expr::Bool(_)
            | Expr::Break
            | Expr::Continue
            | Expr::VariableRef { .. }
            | Expr::EnumDecl(_) => vec![],

            Expr::Return(expr) => vec![expr],

            Expr::Assignment { value, .. } => vec![value],
            Expr::MutableDecl { value, .. } => vec![value],
            Expr::ConstantDecl { value, .. } => vec![value],

            Expr::BinaryOp { left, right, .. } => vec![left, right],
            Expr::Index { target, index } => vec![target, index],
            Expr::FieldAccess { target, .. } => vec![target],

            Expr::BuiltInCall { args, .. } => args.iter().collect(),

            Expr::MethodCall { target, args, .. } => {
                let mut children = vec![target.as_ref()];
                children.extend(args.iter());
                children
            }

            Expr::FunctionCall { args, .. } => args.iter().collect(),

            Expr::StructInit { args, .. } => args.iter().collect(),

            Expr::List(items) => items.iter().collect(),

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut children = vec![condition.as_ref()];
                children.extend(then_branch.iter());
                children.extend(else_branch.iter());
                children
            }

            Expr::ElseIf {
                condition,
                then_branch,
            } => {
                let mut children = vec![condition.as_ref()];
                children.extend(then_branch.iter());
                children
            }

            Expr::Else { then_branch } => then_branch.iter().collect(),

            Expr::Loop { iterable, body, .. } => {
                let mut children = vec![iterable.as_ref()];
                children.extend(body.iter());
                children
            }

            Expr::TemplateString(chunks) => chunks
                .iter()
                .filter_map(|chunk| {
                    if let TemplateChunk::Expr(expr) = chunk {
                        Some(expr.as_ref())
                    } else {
                        None
                    }
                })
                .collect(),

            Expr::Match(match_expr) => {
                let mut children = vec![match_expr.target.as_ref()];
                for (_pattern, body) in &match_expr.arms {
                    children.extend(body.iter());
                }
                children
            }

            Expr::FunctionDef { body, .. } => body.iter().collect(),

            Expr::StructDef { methods, .. } => {
                // methods: Vec<(name, params, body, return_type)>
                methods
                    .iter()
                    .flat_map(|(_name, _params, body, _ret)| body.iter())
                    .collect()
            }
        }
    }
}
