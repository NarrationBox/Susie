use cranelift::prelude::*;
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use weld::{Data, WeldConf, WeldContext, WeldModule, WeldValue};

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
    #[repr(C)]
    struct MyArgs {
        a: i32,
        b: i32,
    }

    let code = "|a: i32, b: i32| a + b";
    let conf = &WeldConf::new();
    let mut module = WeldModule::compile(code, conf).unwrap();

    // Weld accepts a packed C struct as an argument.
    let args = &MyArgs { a: 9, b: 60 };
    let input = &WeldValue::new_from_data(args as *const _ as Data);

    // A context manages memory.
    let context = &mut WeldContext::new(conf).unwrap();
    // Running a Weld module and reading a value out of it is unsafe!
    unsafe {
        // Run the module, which returns a wrapper `WeldValue`.
        let result = module.run(context, input).unwrap();
        // The data is just a pointer: cast it to the expected type
        let data = result.data() as *const i32;

        let result = (*data).clone();
        assert_eq!(args.a + args.b, result);
        println!("Result from Weld: {output}", output = result);
    }
}
