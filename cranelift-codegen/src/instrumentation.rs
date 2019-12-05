use crate::cursor::{Cursor, FuncCursor};
use crate::ir::{
    types::I32, entities::Value, Function, InstBuilder, LibCall, ExternalName, Signature, ExtFuncData, AbiParam
};
use crate::isa::{CallConv, TargetIsa};
use crate::flowgraph::ControlFlowGraph;

/// The main pre-opt pass.
pub fn do_instrumentation(func: &mut Function, _cfg: &mut ControlFlowGraph, isa: &dyn TargetIsa) {
//    print!("{}\n",func);
    let count_instruction = false;

    // look at only user defined functions
    if let ExternalName::User { namespace, index } = func.name {

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

        let funchook_enter = func.import_function(ExtFuncData {
            name: ExternalName::LibCall(LibCall::FuncHookEnter),
            signature: sigref,
            colocated: isa.flags().colocated_libcalls(),
        });

        let funchook_exit = func.import_function(ExtFuncData {
            name: ExternalName::LibCall(LibCall::FuncHookExit),
            signature: sigref,
            colocated: isa.flags().colocated_libcalls(),
        });

        let mut pos = FuncCursor::new(func);

        let mut is_entry: bool = true;
        let mut function_i: Option<Value> = None;
        let mut function_ns: Option<Value> = None;

        // For each instruction in each Basic block
        while let Some(_ebb) = pos.next_ebb() {
            while let Some(_inst) = pos.next_inst()
            {
                let is_return = pos.func.dfg[_inst].opcode().is_return();
                if is_entry {
                    let f_i = pos.ins().iconst(I32, index as i64);
                    let f_ns = pos.ins().iconst(I32, namespace as i64);
                    function_i = Some(f_i);
                    function_ns = Some(f_ns);

                    pos.ins().call(funchook_enter, &[f_ns, f_i]);
                    is_entry = false;
                }


                if let (Some(f_i), Some(f_ns)) = (function_i, function_ns) {
                    if count_instruction {
                        pos.ins().call(insthook, &[f_ns, f_i]);
                    }

                    if is_return {
                        pos.ins().call(funchook_exit, &[f_ns, f_i]);
                    }
                } else {
                    panic! ("Error: function info has not been inserted as constants");
                }

            }
        }

    }
//    print!("{}\n",func);
}
