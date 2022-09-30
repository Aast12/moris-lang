use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Operator, Box<Expr>),
    Error,
}

#[derive(Clone, Copy)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
    Pipe,
    And,
    Or,
    LessThan,
    GreaterThan,
    NotEq,
    Eq,
    Assign,
}

pub enum TypeConst {
    BoolConst,
    IntConst,
    FloatConst,
    StringConst,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Pipe => write!(fmt, "|>"),
            And => write!(fmt, "&&"),
            Or => write!(fmt, "||"),
            LessThan => write!(fmt, "<"),
            GreaterThan => write!(fmt, ">"),
            NotEq => write!(fmt, "!="),
            Eq => write!(fmt, "=="),
            Assign => write!(fmt, "="),
        }
    }
}
