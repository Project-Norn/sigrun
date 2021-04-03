use siderow::ssa;

use crate::common::{
    operator::BinaryOperator,
    symtab::{NodeId, SymbolTable},
    types::Type,
};
use crate::frontend::ast;

pub fn translate(module: ast::Module, symtab: &mut SymbolTable) -> ssa::Module {
    SsaGen::new(symtab).translate(module)
}

struct SsaGen<'a> {
    module: ssa::Module,

    symtab: &'a mut SymbolTable,
    scopes: Vec<NodeId>,
}

impl<'a> SsaGen<'a> {
    fn new(symtab: &'a mut SymbolTable) -> Self {
        Self {
            module: ssa::Module::new(),
            symtab,
            scopes: Vec::new(),
        }
    }

    fn translate(mut self, module: ast::Module) -> ssa::Module {
        self.push(module.id);
        for function in module.functions {
            let ssa_function = self.trans_function(function);
            self.module.add_function(ssa_function);
        }
        self.pop();

        self.module
    }

    fn trans_function(&mut self, func: ast::Function) -> ssa::Function {
        let mut function = ssa::Function::new(&func.name, ssa::Type::I32, vec![]);
        let mut builder = ssa::FunctionBuilder::new(&mut function);
        let entry_block = builder.new_block();
        builder.set_block(entry_block);

        self.push(func.id);
        if let Some(body) = func.body {
            self.trans_stmt(body, &mut builder);
        }
        self.pop();

        function
    }

    fn trans_stmt(&mut self, stmt: ast::Statement, builder: &mut ssa::FunctionBuilder) {
        match stmt.kind {
            ast::StatementKind::Block { stmts } => {
                self.push(stmt.id);
                for stmt in stmts {
                    self.trans_stmt(stmt, builder);
                }
                self.pop();
            }
            ast::StatementKind::Var { name, typ, value }
            | ast::StatementKind::Val { name, typ, value } => {
                self.trans_var(name, typ, value.map(|v| *v), builder)
            }
            ast::StatementKind::Assign { dst, value } => self.trans_assign(*dst, *value, builder),
            ast::StatementKind::Return { value } => {
                self.trans_return_stmt(value.map(|v| *v), builder)
            }
            ast::StatementKind::If { cond, then, els } => {
                self.trans_if_stmt(*cond, *then, els.map(|v| *v), builder)
            }
            ast::StatementKind::While { cond, body } => {
                self.trans_while_stmt(*cond, *body, builder)
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_var(
        &mut self,
        name: String,
        typ: Type,
        value: Option<ast::Expression>,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let typ = self.trans_type(typ);
        let dst = builder.alloc(typ);

        if let Some(value) = value {
            let src = self.trans_expr(value, builder);
            builder.store(dst, src);
        }

        self.symtab.set_local(self.cur_scope(), name, dst);
    }

    fn trans_assign(
        &mut self,
        dst: ast::Expression,
        value: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let dst = self.trans_lvalue(dst);
        let src = self.trans_expr(value, builder);
        builder.store(dst, src);
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

    fn trans_if_stmt(
        &mut self,
        cond: ast::Expression,
        then: ast::Statement,
        els: Option<ast::Statement>,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let block_then = builder.new_block();
        let block_els = builder.new_block();
        let block_merge = if els.is_some() {
            builder.new_block()
        } else {
            block_els
        };

        let cond = self.trans_expr(cond, builder);
        builder.cond_br(cond, block_then, block_els);

        builder.set_block(block_then);
        self.trans_stmt(then, builder);
        if !builder.is_terminated() {
            builder.br(block_merge);
        }

        builder.set_block(block_els);
        if let Some(els) = els {
            self.trans_stmt(els, builder);
            if !builder.is_terminated() {
                builder.br(block_merge);
            }
        }

        builder.set_block(block_merge);
    }

    fn trans_while_stmt(
        &mut self,
        cond: ast::Expression,
        body: ast::Statement,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let cond_block = builder.new_block();
        let body_block = builder.new_block();
        let exit_block = builder.new_block();

        builder.br(cond_block);
        builder.set_block(cond_block);
        let cond = self.trans_expr(cond, builder);
        builder.cond_br(cond, body_block, exit_block);

        builder.set_block(body_block);
        self.trans_stmt(body, builder);
        builder.br(cond_block);

        builder.set_block(exit_block)
    }

    fn trans_expr(
        &mut self,
        expr: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        match expr.kind {
            ast::ExpressionKind::Integer { value } => ssa::Value::new_i32(value),
            ast::ExpressionKind::Bool { value } => ssa::Value::new_i1(value),

            ast::ExpressionKind::Ident { name } => self.trans_ident(name, builder),
            ast::ExpressionKind::BinaryOp { op, lhs, rhs } => {
                self.trans_binop(op, *lhs, *rhs, builder)
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_ident(&mut self, name: String, builder: &mut ssa::FunctionBuilder) -> ssa::Value {
        let sig = self.symtab.find_variable(self.cur_scope(), &name).unwrap();
        builder.load(&self.module, sig.val.unwrap())
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

            Equal => builder.eq(lhs, rhs),
            NotEqual => builder.neq(lhs, rhs),
            Lt => builder.lt(lhs, rhs),
            Lte => builder.lte(lhs, rhs),
            Gt => builder.gt(lhs, rhs),
            Gte => builder.gte(lhs, rhs),
        }
    }

    fn trans_lvalue(&mut self, expr: ast::Expression) -> ssa::Value {
        match expr.kind {
            ast::ExpressionKind::Ident { name } => {
                let sig = self.symtab.find_variable(self.cur_scope(), &name).unwrap();
                sig.val.unwrap()
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_type(&self, typ: Type) -> ssa::Type {
        match typ {
            Type::Int => ssa::Type::I32,
            Type::Bool => ssa::Type::I1,

            x => unimplemented!("{:?}", x),
        }
    }

    fn push(&mut self, node: NodeId) {
        self.scopes.push(node);
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }

    fn cur_scope(&self) -> NodeId {
        *self.scopes.last().unwrap()
    }
}
