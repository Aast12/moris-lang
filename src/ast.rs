pub mod expressions;
pub mod functions;
pub mod node;
pub mod quadruples;
pub mod statements;
pub mod temp;
pub mod types;

use std::fmt::Debug;

use self::{expressions::Expression, node::Node};

#[derive(Debug)]
pub struct Dimension(pub i8, pub Vec<Box<Expression>>); // dimensions number, dimension sizes

impl<'m> Node<'m> for Dimension {
    fn generate(&mut self) -> () {
        for dim in self.1.iter_mut() {
            dim.generate();
        }
    }
}

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vec<Box<Expression>>),
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
