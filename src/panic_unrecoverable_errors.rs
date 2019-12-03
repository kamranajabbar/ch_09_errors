pub fn run() {
    // Use of Panic
    // "panic!" == executes program, print a failure message, unwind and clean up the stack, and then quit
    //panic!("crash and burn");

    // "panic!"" with Backtrace
    let v = vec![10,20,30];
    v[99];
}