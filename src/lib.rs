#![feature(plugin_registrar, box_syntax)]
#![feature(rustc_private, collections)]

#![allow(unused_imports)]

#[macro_use]
extern crate syntax;
#[macro_use]
extern crate rustc;

// Only for the compile time checking of paths
extern crate collections;

use rustc::plugin::Registry;
use rustc::lint::LintPassObject;

pub mod types;
pub mod misc;
pub mod eq_op;
pub mod bit_mask;
pub mod ptr_arg;
pub mod needless_bool;
pub mod approx_const;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box types::TypePass as LintPassObject);
    reg.register_lint_pass(box misc::MiscPass as LintPassObject);
    reg.register_lint_pass(box misc::StrToStringPass as LintPassObject);
    reg.register_lint_pass(box misc::TopLevelRefPass as LintPassObject);
    reg.register_lint_pass(box misc::CmpNan as LintPassObject);
    reg.register_lint_pass(box eq_op::EqOp as LintPassObject);
    reg.register_lint_pass(box bit_mask::BitMask as LintPassObject);
    reg.register_lint_pass(box ptr_arg::PtrArg as LintPassObject);
    reg.register_lint_pass(box needless_bool::NeedlessBool as LintPassObject);
    reg.register_lint_pass(box approx_const::ApproxConstant as LintPassObject);
    
    reg.register_lint_group("clippy", vec![types::BOX_VEC, types::LINKEDLIST,
                                           misc::SINGLE_MATCH, misc::STR_TO_STRING,
                                           misc::TOPLEVEL_REF_ARG, eq_op::EQ_OP,
                                           bit_mask::BAD_BIT_MASK, ptr_arg::PTR_ARG,
                                           needless_bool::NEEDLESS_BOOL,
                                           approx_const::APPROX_CONSTANT,
                                           misc::CMP_NAN
                                           ]);
}
