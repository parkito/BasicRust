fn main() {
    bar().unwrap();
}

fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        //executes when and object goes out a scope
        fn drop(&mut self) {
            println!("drop is executed");
        }
    }

    // The dtor of _exit will run however the function `bar` is exited.
    let _exit = Foo;
    // Normal return.
    Ok(())
}

// It is not guaranteed that destructors will run.
// For example, if there is an infinite loop in a function or if running a function crashes before exit.
// Destructors are also not run in the case of a panic in an already panicking thread.
// Therefore, destructors cannot be relied on as finalizers where it is absolutely essential that finalisation happens.

// This pattern introduces some hard to notice, implicit code.
// Reading a function gives no clear indication of destructors to be run on exit. This can make debugging tricky.

// Requiring an object and Drop impl just for finalisation is heavy on boilerplate.