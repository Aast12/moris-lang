use std::fmt::{Debug};

#[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub enum Expr {
    Const(TypeConst),
    Op(Box<Expr>, Operator, Box<Expr>),
    ParenthOp(Box<Expr>),
    Id(String),
    Error,
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
    Pipe,
    ForwardPipe,
    And,
    Or,
    LessThan,
    GreaterThan,
    NotEq,
    Eq,
    Assign,
}



// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Expr::*;
//         match &*self {
//             Const(n) => write!(fmt, "{:?}", n),
//             Op(ref l, op, ref r) => write!(fmt, "{:?} {:?} {:?}", l, op, r),
//             Id(s) => write!(fmt, "{}", s),
//             ParenthOp(op) => write!(fmt, "({:?})", op),
//             Error => write!(fmt, "error"),
//         }
//     }
// }

// impl Debug for Operator {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Operator::*;
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
//             ForwardPipe =>  write!(fmt, "|> forward"),
//         }
//     }
// }
