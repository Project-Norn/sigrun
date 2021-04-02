use anyhow::Result;
use siderow::arch::x86;
use std::fs;

use crate::{
    common::cli::CompilerConfig,
    frontend::{self, lexer::SourceFile},
    middleend,
};

pub fn compile_to_file(config: CompilerConfig) -> Result<()> {
    let source = SourceFile {
        filename: config.input_file.to_owned(),
        content: fs::read_to_string(&config.input_file)?,
    };
    let output = compile(source, &config)?;
    fs::write(&config.output_file, output)?;
    Ok(())
}

pub fn compile(source: SourceFile, _config: &CompilerConfig) -> Result<String> {
    let tokens = frontend::lexer::tokenize(source)?;
    let module = frontend::parser::parse(tokens)?;
    let mut symtab = frontend::type_check::apply(&module)?;
    frontend::sema_check::apply(&module)?;

    let module = middleend::ssagen::translate(module, &mut symtab);
    let mut asm = x86::instsel::translate(module);
    x86::regalloc::allocate(&mut asm);
    Ok(asm.stringify())

    // let program = middleend::ssa::translate(&program)?;
    // middleend::optimize::apply(&mut program)?;

    // let code = backend::x86::translate(&program)?;
    // backend::x86::regalloc::apply(&mut code)?;
    // backend::x86::optimize::apply(&mut code)?;
    // backend::x86::generator::generate(code)?;
}
