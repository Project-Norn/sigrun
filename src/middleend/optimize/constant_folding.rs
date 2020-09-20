use crate::{
    common::operator::{BinaryOperator, UnaryOperator},
    frontend::parser::ast::*,
};

pub fn optimize(mut program: Program) -> Program {
    let mut functions = Vec::new();
    for function in program.functions {
        functions.push(Function {
            name: function.name,
            ret_typ: function.ret_typ,
            body: match opt_statement(function.body) {
                Some(stmt) => stmt,
                None => AstStatement::Block { stmts: Vec::new() },
            },
        });
    }
    program.functions = functions;
    program
}

fn opt_statement(statement: AstStatement) -> Option<AstStatement> {
    match statement {
        AstStatement::Block { stmts } => {
            let mut new_stmts = Vec::new();
            for stmt in stmts {
                if let Some(new_stmt) = opt_statement(stmt) {
                    new_stmts.push(new_stmt);
                }
            }
            Some(AstStatement::Block { stmts: new_stmts })
        }
        AstStatement::Declare { name, typ, value } => Some(AstStatement::Declare {
            name,
            typ,
            value: Box::new(opt_expression(*value)),
        }),
        AstStatement::Assign { name, value } => Some(AstStatement::Assign {
            name,
            value: Box::new(opt_expression(*value)),
        }),
        AstStatement::Return { value } => Some(AstStatement::Return {
            value: Box::new(opt_expression(*value)),
        }),
        AstStatement::If { cond, then, els } => {
            let cond = opt_expression(*cond);
            if let AstExpression::Bool { value } = cond {
                match value {
                    true => opt_statement(*then),
                    false => match els {
                        Some(els) => Some(opt_statement(*els)?),
                        None => None,
                    },
                }
            } else {
                Some(AstStatement::If {
                    cond: Box::new(cond),
                    then,
                    els,
                })
            }
        }
        AstStatement::While { cond, body } => Some(AstStatement::While {
            cond: Box::new(opt_expression(*cond)),
            body: Box::new(opt_statement(*body)?),
        }),
    }
}

fn opt_expression(expression: AstExpression) -> AstExpression {
    match expression {
        AstExpression::Integer { .. } => expression,
        AstExpression::Bool { .. } => expression,
        AstExpression::Ident { .. } => expression,
        AstExpression::UnaryOp { op, expr } => match *expr {
            AstExpression::Integer { value } => opt_unop_int(op, value),
            AstExpression::Bool { value } => opt_unop_bool(op, value),
            expr => expr,
        },
        AstExpression::BinaryOp { op, lhs, rhs } => {
            let lhs = opt_expression(*lhs);
            let rhs = opt_expression(*rhs);
            match (lhs, rhs) {
                (
                    AstExpression::Integer { value: left_value },
                    AstExpression::Integer { value: right_value },
                ) => {
                    return opt_binop_int(op, left_value, right_value);
                }
                (lhs, rhs) => AstExpression::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            }
        }
    }
}

fn opt_unop_int(op: UnaryOperator, value: i32) -> AstExpression {
    match op {
        UnaryOperator::Not => AstExpression::Integer { value: !value },
    }
}

fn opt_unop_bool(op: UnaryOperator, value: bool) -> AstExpression {
    match op {
        UnaryOperator::Not => AstExpression::Bool { value: !value },
    }
}

fn opt_binop_int(op: BinaryOperator, left_value: i32, right_value: i32) -> AstExpression {
    match op {
        BinaryOperator::Add => AstExpression::Integer {
            value: left_value + right_value,
        },
        BinaryOperator::Sub => AstExpression::Integer {
            value: left_value - right_value,
        },
        BinaryOperator::Mul => AstExpression::Integer {
            value: left_value * right_value,
        },
        BinaryOperator::Div => AstExpression::Integer {
            value: left_value / right_value,
        },
        BinaryOperator::And => AstExpression::Integer {
            value: left_value & right_value,
        },
        BinaryOperator::Or => AstExpression::Integer {
            value: left_value | right_value,
        },
        BinaryOperator::Xor => AstExpression::Integer {
            value: left_value ^ right_value,
        },

        BinaryOperator::Equal => AstExpression::Bool {
            value: left_value == right_value,
        },
        BinaryOperator::NotEqual => AstExpression::Bool {
            value: left_value != right_value,
        },
        BinaryOperator::Lt => AstExpression::Bool {
            value: left_value < right_value,
        },
        BinaryOperator::Lte => AstExpression::Bool {
            value: left_value <= right_value,
        },
        BinaryOperator::Gt => AstExpression::Bool {
            value: left_value > right_value,
        },
        BinaryOperator::Gte => AstExpression::Bool {
            value: left_value >= right_value,
        },
    }
}