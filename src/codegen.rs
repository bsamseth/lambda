// #![allow(unused_variables)]
// #![allow(dead_code)]
// use std::collections::HashMap;
//
// use inkwell::builder::Builder;
// use inkwell::context::Context;
// use inkwell::module::Module;
// use inkwell::passes::PassManager;
// use inkwell::values::{FunctionValue, PointerValue};
//
// use super::parser::expr::Expression;
//
// pub type CompileResult = Result<(), &'static str>;
//
// struct Compiler<'a, 'ctx> {
//     context: &'ctx Context,
//     builder: &'a Builder<'ctx>,
//     fpm: &'a PassManager<FunctionValue<'ctx>>,
//     module: &'a Module<'ctx>,
//
//     variables: HashMap<String, PointerValue<'ctx>>,
//     fn_value_opt: Option<FunctionValue<'ctx>>,
// }
//
// pub fn compile(expr: Expression) -> String {
//     let context = Context::create();
//     let module = context.create_module("lambda");
//     let builder = context.create_builder();
//     let fpm = PassManager::create(&module);
//     fpm.add_instruction_combining_pass();
//     fpm.add_reassociate_pass();
//     fpm.add_gvn_pass();
//     fpm.add_cfg_simplification_pass();
//     fpm.add_basic_alias_analysis_pass();
//     fpm.add_promote_memory_to_register_pass();
//     fpm.add_instruction_combining_pass();
//     fpm.add_reassociate_pass();
//     fpm.initialize();
//
//     let compiler = Compiler {
//         context: &context,
//         builder: &builder,
//         fpm: &fpm,
//         module: &module,
//         variables: HashMap::new(),
//         fn_value_opt: None,
//     };
//
//     let i64_type = compiler.context.i64_type();
//     let fn_type = i64_type.fn_type(&[], false);
//     let function = compiler.module.add_function("main", fn_type, None);
//     let basic_block = compiler.context.append_basic_block(function, "entry");
//     compiler.builder.position_at_end(basic_block);
//     let string = unsafe {
//         compiler
//             .builder
//             .build_global_string("Hello, World!", "hello_str")
//     };
//     let return_value = i64_type.const_int(123, false);
//     compiler.builder.build_return(Some(&return_value));
//     return compiler.module.print_to_string().to_string();
// }
