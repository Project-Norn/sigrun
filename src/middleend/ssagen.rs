use siderow::ssa;

use crate::common::operator::BinaryOperator;
use crate::frontend::ast;

pub fn translate(module: ast::Module) -> ssa::Module {
    SsaGen::new().translate(module)
}

struct SsaGen {}

impl SsaGen {
    fn new() -> Self {
        Self {}
    }

    fn translate(&mut self, module: ast::Module) -> ssa::Module {
        let mut ssa_module = ssa::Module::new();

        for function in module.functions {
            let ssa_function = self.trans_function(function);
            ssa_module.add_function(ssa_function);
        }

        ssa_module
    }

    fn trans_function(&mut self, func: ast::Function) -> ssa::Function {
        let mut function = ssa::Function::new(&func.name, ssa::Type::I32, vec![]);
        let mut builder = ssa::FunctionBuilder::new(&mut function);
        let entry_block = builder.new_block();
        builder.set_block(entry_block);

        if let Some(body) = func.body {
            self.trans_stmt(body, &mut builder);
        }

        function
    }

    fn trans_stmt(&mut self, stmt: ast::Statement, builder: &mut ssa::FunctionBuilder) {
        match stmt.kind {
            ast::StatementKind::Block { stmts } => {
                for stmt in stmts {
                    self.trans_stmt(stmt, builder);
                }
            }
            ast::StatementKind::Return { value } => {
                self.trans_return_stmt(value.map(|v| *v), builder)
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_return_stmt(
        &mut self,
        value: Option<ast::Expression>,
        builder: &mut ssa::FunctionBuilder,
    ) {
        match value {
            None => builder.ret_void(),
            Some(value) => {
                let value = self.trans_expr(value, builder);
                builder.ret(value);
            }
        }
    }

    fn trans_expr(
        &mut self,
        expr: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        match expr.kind {
            ast::ExpressionKind::Integer { value } => ssa::Value::new_i32(value),

            ast::ExpressionKind::BinaryOp { op, lhs, rhs } => {
                self.trans_binop(op, *lhs, *rhs, builder)
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_binop(
        &mut self,
        op: BinaryOperator,
        lhs: ast::Expression,
        rhs: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        use BinaryOperator::*;

        let lhs = self.trans_expr(lhs, builder);
        let rhs = self.trans_expr(rhs, builder);

        match op {
            Add => builder.add(lhs, rhs),
            Sub => builder.sub(lhs, rhs),
            Mul => builder.mul(lhs, rhs),
            Div => builder.div(lhs, rhs),
            Mod => builder.rem(lhs, rhs),
            And => builder.and(lhs, rhs),
            Or => builder.or(lhs, rhs),
            Xor => builder.xor(lhs, rhs),

            x => unimplemented!("{:?}", x),
        }
    }
}
