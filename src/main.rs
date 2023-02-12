#[macro_use] extern crate lalrpop_util;
extern crate llvm_sys;

use ast::Expr;
use ast::Opcode;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::{LLVMBuilder, LLVMIntPredicate, LLVMModule};
use std::ptr;
use std::process::Command;
use std::ffi::{CStr, CString};
use std::os::raw::{c_uint, c_ulonglong};

lalrpop_mod!(pub calculator); // synthesized by LALRPOP
mod ast;
use std::fs;

macro_rules! c_str {
    ($s:expr) => (
        concat!($s, "\0").as_ptr() as *const i8
    );
}

const LLVM_FALSE: LLVMBool = 0;
const LLVM_TRUE: LLVMBool = 1;

// Types 
/// Convert this integer to LLVM's representation of a constant
/// integer.
unsafe fn int8(val: c_ulonglong) -> LLVMValueRef {
    LLVMConstInt(LLVMInt8Type(), val, LLVM_FALSE)
}
/// Convert this integer to LLVM's representation of a constant
/// integer.
// TODO: this should be a machine word size rather than hard-coding 32-bits.
fn int32(val: c_ulonglong) -> LLVMValueRef {
    unsafe { LLVMConstInt(LLVMInt32Type(), val, LLVM_FALSE) }
}

fn int1_type() -> LLVMTypeRef {
    unsafe { LLVMInt1Type() }
}

fn int8_type() -> LLVMTypeRef {
    unsafe { LLVMInt8Type() }
}

fn int32_type() -> LLVMTypeRef {
    unsafe { LLVMInt32Type() }
}

fn int8_ptr_type() -> LLVMTypeRef {
    unsafe { LLVMPointerType(LLVMInt8Type(), 0) }
}

fn compile(expr: Expr) {
    unsafe {
        // setup
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("main"));
        let builder = LLVMCreateBuilderInContext(context);

        // START
        // common void type
        let void_type = LLVMVoidTypeInContext(context);
    
        

        // our "main" function which will be the entry point when we run the executable
        let main_func_type = LLVMFunctionType(void_type, ptr::null_mut(), 0, 0);
        let main_func = LLVMAddFunction(module, c_str!("main"), main_func_type);
        let main_block = LLVMAppendBasicBlockInContext(context, main_func, c_str!("main"));
        LLVMPositionBuilderAtEnd(builder, main_block);

        // START MATCH
        match_expr(expr);
        // END MATCH
        // main's function body

        //EXAMPLE
        let value_index_ptr = LLVMBuildAlloca(builder, int32_type(), c_str!("value"));
        let value_ptr_init = int32(32);
        LLVMBuildStore(builder, value_ptr_init, value_index_ptr);

        // create string vairables and then function
        let world_str = LLVMBuildGlobalStringPtr(builder, c_str!("world!"), c_str!(""));
        let value_is_str = LLVMBuildGlobalStringPtr(builder, c_str!("Value is %ld\n"), c_str!(""));
        let print_func_type = LLVMFunctionType(void_type, [int8_ptr_type()].as_mut_ptr(), 1, 1);
        let print_func = LLVMAddFunction(module, c_str!("printf"), print_func_type);
        let print_args = [value_is_str, value_index_ptr].as_mut_ptr();
        // calling `printf("Hello %s!", "world")`
        LLVMBuildCall(builder, print_func, print_args, 2, c_str!(""));



        LLVMBuildRetVoid(builder);

        // write our bitcode file to arm64
        LLVMSetTarget(module, c_str!("arm64"));
        LLVMWriteBitcodeToFile(module, c_str!("bin/main.bc"));

        // clean up
        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }

    // Run clang
    Command::new("clang")
    .arg("bin/main.bc")
    .arg("-o")
    .arg("bin/main")
    .output()
    .expect("Failed to execute clang with main.bc file");

    println!("main executable generated, run with ./main")
    
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

fn match_op(lhs: Expr, value: Opcode, rhs: Expr) {
    // only set if lhs is not a number as this will be set in second lhs
    match rhs {
        Expr::Number(rhs) => {
            match lhs {
                Expr::Number(_) => {},
                _ => {println!("Setting x {:?} {:?}, where x is {:?}", value, rhs, lhs)
            },
            }
        },
        _ => {},
    }
    match lhs {
        Expr::Number(lhs) => {set_value(ast::Expr::Number(lhs), value, rhs)},
        Expr::Op(lhs, op , rhs) => {match_op(unbox(lhs), op, unbox(rhs))}
        Expr::Error => {},
    }
}

fn set_value(value: Expr, opcode: Opcode, lhs: Expr) {
    println!("Setting lhs ({:?} {:?} {:?})", value, opcode, lhs)
}

fn match_expr(value: Expr) {
    match value {
        // If code is just one value and not an expression
        Expr::Number(_) => {println!("setting two {:?}", value)}
        Expr::Op(lhs, op , rhs) => {match_op(unbox(lhs), op, unbox(rhs))}
        Expr::Error => {},
    }
}

fn parse(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let expr = calculator::ExprParser::new()
        .parse(&contents);

    match expr {
        Ok(expr) => {compile(unbox(expr))}
        Err(_) => {}
    }
}

fn main() {
    let path = std::env::args().skip(1).next();

    match path {
        None => println!("interpreter"),
        Some(path) => parse(path),
    }
}