pub mod expressions;
pub mod node;
pub mod quadruples;
pub mod types;

use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum Index {
    Simple(Box<Expr>),
    Range(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Dimension(pub i8, pub Vec<Box<Expr>>);
// dimensions: i8, // 0, 1, 2 dim limit
// shape:

#[derive(Debug)]
pub struct Variable {
    pub id: String,
    pub data_type: types::DataType,
    pub dimension: Dimension,
    pub value: Option<Box<Expr>>,
}

#[derive(Debug)]
pub struct VarRef {
    pub id: String,
    pub indexing: Option<Vec<Index>>,
}

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vec<Box<Expr>>),
}

#[derive(Debug)]
pub enum Expr {
    Const(TypeConst),
    Op(Box<Expr>, types::Operator, Box<Expr>),
    ParenthOp(Box<Expr>),
    Var(VarRef),
    FunctionCall(String, Vec<Box<Expr>>),
    Error,
}

#[derive(Debug)]
pub struct FunctionParam(pub String, pub types::DataType);

#[derive(Debug)]
pub struct FunctionSignature {
    pub id: String,
    pub data_type: types::DataType,
    pub params: Vec<FunctionParam>,
}

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Block,
}

#[derive(Debug)]
pub enum Statement {
    VarDeclaration(Variable),
    VarAssign(VarRef, Box<Expr>),
    Expression(Box<Expr>),
    If {
        condition: Box<Expr>,
        if_block: Block,
        else_block: Option<Block>,
    },
    For {
        iterator_id: String,
        iterable: Box<Expr>,
        block: Block,
    },
    While {
        condition: Box<Expr>,
        block: Block,
    },
    FunctionDeclaration(Function),
    Return(Box<Expr>),
}

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug)]
pub struct Program(pub Vec<Statement>);

// impl Debug for TypeConst {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::TypeConst::*;
//         match &*self {
//             Bool(value) => write!(fmt, "{:?}", value),
//             Int(value) => write!(fmt, "{:?}", value),
//             Float(value) => write!(fmt, "{:?}", value),
//             String(value) => write!(fmt, "{:?}", value),
//             Vector(value) => write!(fmt, "{:?}", value),
//         }
//     }
// }

// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Expr::*;
//         match &*self {
//             Const(n) => write!(fmt, "{:?}", n),
//             Op(ref l, op, ref r) => write!(fmt, "{:?} {:?} {:?}", l, op, r),
//             Var(s) => write!(fmt, "{:?}", s),
//             ParenthOp(op) => write!(fmt, "({:?})", op),
//             Error => write!(fmt, "error"),
//             FunctionCall(id, params) => write!(fmt, "{:?}: {:?}", id, params),
//         }
//     }
// }

// impl Debug for types::Operator {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::types::Operator::*;
//         match *self {
//             Mul => write!(fmt, "*"),
//             Div => write!(fmt, "/"),
//             Add => write!(fmt, "+"),
//             Sub => write!(fmt, "-"),
//             Pipe => write!(fmt, "|>"),
//             And => write!(fmt, "&&"),
//             Or => write!(fmt, "||"),
//             LessThan => write!(fmt, "<"),
//             GreaterThan => write!(fmt, ">"),
//             NotEq => write!(fmt, "!="),
//             Eq => write!(fmt, "=="),
//             Assign => write!(fmt, "="),
//             ForwardPipe => write!(fmt, "|> forward"),
//         }
//     }
// }
