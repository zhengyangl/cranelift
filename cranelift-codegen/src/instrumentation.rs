use crate::cursor::{Cursor, FuncCursor};
use crate::ir::{
    condcodes::{CondCode, IntCC},
    dfg::ValueDef,
    immediates,
    instructions::{Opcode, ValueList},
    types::{I16, I32, I64, I8},
    DataFlowGraph, Ebb, Function, Inst, InstBuilder, InstructionData, Type, LibCall, ExternalName, Signature, ExtFuncData
};
use crate::isa::{CallConv, TargetIsa};
use crate::flowgraph::ControlFlowGraph;
use alloc::vec::Vec;

/// The main pre-opt pass.
pub fn do_instrumentation(func: &mut Function, cfg: &mut ControlFlowGraph, isa: &dyn TargetIsa) {
    // get ready for the callsite
    let call_conv = CallConv::for_libcall(isa);
    let mut sig = Signature::new(call_conv);
    let sigref = func.import_signature(sig);
    let callee = func.import_function(ExtFuncData {
        name: ExternalName::LibCall(LibCall::PrintText),
        signature: sigref,
        colocated: isa.flags().colocated_libcalls(),
    });


    let mut pos = FuncCursor::new(func);

    // For each instruction in each Basic block
    while let Some(ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst()
        {
            let mut args = Vec::new();
            pos.ins().call(callee, &args);
        }
    }
}
