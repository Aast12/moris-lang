use std::fmt::{Debug, Error, Formatter};

// #[derive(Debug)]
pub enum TypeConst {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Vector(Vec<Box<Expr>>),
}

// #[derive(Debug)]
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

impl Debug for TypeConst {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::TypeConst::*;
        match &*self {
            Bool(value) => write!(fmt, "{:?}", value),
            Int(value) => write!(fmt, "{:?}", value),
            Float(value) => write!(fmt, "{:?}", value),
            String(value) => write!(fmt, "{}", value),
            Vector(value) => write!(fmt, "{:?}", value),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Const(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "{:?} {:?} {:?}", l, op, r),
            Id(s) => write!(fmt, "{}", s),
            ParenthOp(op) => write!(fmt, "({:?})", op),
            Error => write!(fmt, "error"),
        }
    }
}

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
