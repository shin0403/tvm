extern "C" {
    fn __wasm_call_ctors();
}

extern crate ndarray;
#[macro_use]
extern crate tvm_runtime;

use ndarray::Array;
use tvm_runtime::{DLTensor, Module as _, SystemLibModule};

#[no_mangle]
pub extern "C" fn run() {
    unsafe {
        // This is necessary to invoke TVMBackendRegisterSystemLibSymbol
        // API calls.
        __wasm_call_ctors();
    }
    // try static
    let mut a = Array::from_vec(vec![1f32, 2., 3., 4.]);
    let mut b = Array::from_vec(vec![1f32, 0., 1., 0.]);
    let mut c = Array::from_vec(vec![0f32; 4]);
    let e = Array::from_vec(vec![2f32, 2., 4., 4.]);
    let mut a_dl: DLTensor = (&mut a).into();
    let mut b_dl: DLTensor = (&mut b).into();
    let mut c_dl: DLTensor = (&mut c).into();

    let syslib = SystemLibModule::default();
    let fadd = syslib
        .get_function("add")
        .expect("add function not found");
    call_packed!(fadd, &mut a_dl, &mut b_dl, &mut c_dl).unwrap();
    assert!(c.all_close(&e, 1e-8f32));

    let e = Array::from_vec(vec![0f32, 2., 2., 4.]);
    let sub = syslib
        .get_function("sub")
        .expect("sub function not found");
    call_packed!(sub, &mut a_dl, &mut b_dl, &mut c_dl).unwrap();
    assert!(c.all_close(&e, 1e-8f32));
}
