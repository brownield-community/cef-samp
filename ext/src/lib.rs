#[no_mangle]
pub extern "C" fn ext_test_function() -> u32 {
    println!("Test");
    return 1;
}