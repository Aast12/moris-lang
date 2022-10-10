pub mod expressions;
pub mod node;
pub mod quadruples;
pub mod types;
pub mod statements;
pub mod functions;

use std::fmt::{Debug, Error, Formatter};

use self::{expressions::{Expression, Index, id::Access}, statements::Block};


#[derive(Debug)]
pub struct Dimension<'m>(pub i8, pub Vec<Box<Expression<'m>>>); // dimensions number, dimension sizes

#[derive(Debug)]
pub enum TypeConst<'m> {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vec<Box<Expression<'m>>>),
}

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
