use crate::cursor::{Cursor, FuncCursor};
use crate::ir::{
    condcodes::{CondCode, IntCC},
    dfg::ValueDef,
    immediates,
    instructions::{Opcode, ValueList},
    types::{I16, I32, I64, I8},
    DataFlowGraph, Ebb, Function, Inst, InstBuilder, InstructionData, Type, LibCall, ExternalName, Signature, ExtFuncData, AbiParam
};
use crate::isa::{CallConv, TargetIsa};
use crate::flowgraph::ControlFlowGraph;
use alloc::vec::Vec;

/// The main pre-opt pass.
pub fn do_instrumentation(func: &mut Function, cfg: &mut ControlFlowGraph, isa: &dyn TargetIsa) {
    print!("{}\n",func);

    // look at only user defined functions
    if let ExternalName::User { namespace, index } = func.name{

        // get ready for the callsite
        let call_conv = CallConv::for_libcall(isa);
        let mut sig = Signature::new(call_conv);
        sig.params.push(AbiParam::new(I32));
        sig.params.push(AbiParam::new(I32));
        let sigref = func.import_signature(sig);
        let insthook = func.import_function(ExtFuncData {
            name: ExternalName::LibCall(LibCall::InstHook),
            signature: sigref,
            colocated: isa.flags().colocated_libcalls(),
        });

        let mut pos = FuncCursor::new(func);

        // For each instruction in each Basic block
        while let Some(_ebb) = pos.next_ebb() {
            while let Some(_inst) = pos.next_inst()
            {
                let function_ns = pos.ins().iconst(I32, namespace as i64);
                let function_i = pos.ins().iconst(I32, index as i64);

                pos.ins().call(insthook, &[function_ns, function_i]);
            }
        }

    }
}
