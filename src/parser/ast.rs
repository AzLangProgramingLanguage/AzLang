use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInFunction {
    Print,
    Input,
    Len,
    Number,
    Sum,
    Range,
    LastWord,
    Sqrt,
    Timer,
    Max,
    Mod,
    Min,
    Round,
    Floor,
    Ceil,
}

impl BuiltInFunction {
    pub fn expected_arg_count(&self) -> Option<usize> {
        match self {
            BuiltInFunction::Print
            | BuiltInFunction::Len
            | BuiltInFunction::Sum
            | BuiltInFunction::Sqrt
            | BuiltInFunction::Round
            | BuiltInFunction::Floor
            | BuiltInFunction::Ceil
            | BuiltInFunction::Mod
            | BuiltInFunction::Number
            | BuiltInFunction::LastWord => Some(1),

            BuiltInFunction::Range => Some(2),

            BuiltInFunction::Timer => Some(0),

            BuiltInFunction::Input => None, // Special case

            BuiltInFunction::Max | BuiltInFunction::Min => None, // Flexible

                                                                 //_ => None, (əgər başqa funksiyalar varsa da qeyd edə bilərsən)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    Metn,
    Siyahi(Box<Type<'a>>),
    Istifadeci(Cow<'a, str>),
    Integer,
    BigInteger,
    LowInteger,
    Bool,
    Char,
    Void,
    Any,
    Float,
}
#[derive(Debug)]
pub struct EnumDecl<'a> {
    pub name: Cow<'a, str>,
    pub variants: Vec<&'a str>,
}

#[derive(Debug)]
pub enum Expr<'a> {
    String(&'a str),
    Bool(bool),
    Number(i64),
    EnumDecl(EnumDecl<'a>),
    Return(Box<Expr<'a>>),
    List(Vec<Expr<'a>>),
    Index {
        target: Box<Expr<'a>>,
        index: Box<Expr<'a>>,
    },
    Loop {
        var_name: &'a str,
        iterable: Box<Expr<'a>>,
        body: Vec<Expr<'a>>,
    },
    Float(f64),
    Decl {
        name: Cow<'a, str>,
        typ: Option<Type<'a>>,
        is_mutable: bool,
        value: Box<Expr<'a>>,
    },
    VariableRef {
        name: Cow<'a, str>,
        symbol: Option<Symbol<'a>>,
    },
    TemplateString(Vec<TemplateChunk<'a>>),
    If {
        condition: Box<Expr<'a>>,
        then_branch: Vec<Expr<'a>>,
        else_branch: Vec<Expr<'a>>,
    },
    ElseIf {
        condition: Box<Expr<'a>>,
        then_branch: Vec<Expr<'a>>,
    },

    Else {
        then_branch: Vec<Expr<'a>>,
    },
    BuiltInCall {
        function: BuiltInFunction,
        args: Vec<Expr<'a>>,
        return_type: Type<'a>,
    },
    Call {
        target: Option<Box<Expr<'a>>>,
        name: &'a str,
        args: Vec<Expr<'a>>,
        returned_type: Option<Type<'a>>,
    },
    StructDef {
        name: &'a str,
        fields: Vec<(&'a str, Type<'a>)>,
        methods: Vec<(&'a str, Vec<Parameter<'a>>, Vec<Expr<'a>>, Option<Type<'a>>)>,
    },
    FunctionDef {
        name: &'a str,
        params: Vec<Parameter<'a>>,
        body: Vec<Expr<'a>>,
        return_type: Option<Type<'a>>,
    },
    StructInit {
        name: String,
        args: Vec<Expr<'a>>,
    },
    Assignment {
        name: Cow<'a, str>,
        value: Box<Expr<'a>>,
        symbol: Option<Symbol<'a>>,
    },
    BinaryOp {
        left: Box<Expr<'a>>,
        op: &'a str,
        right: Box<Expr<'a>>,
    },
}

#[derive(Debug)]
pub struct Program<'a> {
    pub expressions: Vec<Expr<'a>>,
    pub return_type: Option<Type<'a>>,
}

#[derive(Clone, Debug)]
pub struct Symbol<'a> {
    pub typ: Type<'a>,
    pub is_mutable: bool,
    pub is_used: bool,
    pub is_pointer: bool,
    //pub source_location: Option<Location>,
}

#[derive(Debug)]
pub enum TemplateChunk<'a> {
    Literal(&'a str),
    Expr(Box<Expr<'a>>),
}

#[derive(Clone, Debug)]
pub struct Parameter<'a> {
    pub name: String,
    pub typ: Type<'a>,
    pub is_mutable: bool,
    pub is_pointer: bool,
}
