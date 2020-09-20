#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    Not,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,
    Xor,

    Equal,
    NotEqual,

    Lt,
    Lte,
    Gt,
    Gte,
}
