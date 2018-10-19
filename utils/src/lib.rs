//we declare what functions we use from outside rust
extern {
    fn appendNumber(x: u32);
}

//no_mangle keeps the name of the function when compiling to wasm
#[no_mangle]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}


#[no_mangle]
pub extern fn run(){
    unsafe{
        appendNumber(5)
    }
}