use cranelift::prelude::*;
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};

pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),
    WhileLoop(Box<Expr>, Vec<Expr>),
    Call(String, Vec<Expr>),
    GlobalDataAddr(String),
}

fn main() {
    // let builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
    // let module = Module::new(builder);

    /// The basic JIT class.
    pub struct JIT {
        /// The function builder context, which is reused across multiple
        /// FunctionBuilder instances.
        builder_context: FunctionBuilderContext,

        /// The main Cranelift context, which holds the state for codegen. Cranelift
        /// separates this from `Module` to allow for parallel compilation, with a
        /// context per thread, though this isn't in the simple demo here.
        ctx: codegen::Context,

        /// The data context, which is to data objects what `ctx` is to functions.
        data_ctx: DataContext,

        /// The module, with the simplejit backend, which manages the JIT'd
        /// functions.
        module: Module<SimpleJITBackend>,
    }
    println!("Hello from Susie!");
}
