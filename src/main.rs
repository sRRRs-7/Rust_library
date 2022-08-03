mod module;
mod module2;
mod terminal;
mod string_lib;
mod error_lib;
mod concurrency;

fn main() {
    let s = greet();
    assert_eq!(s, "hello");

    terminal::terminal_io::main();
    terminal::terminal_io2::main();

    string_lib::string_lib::main();

    module::mod_borrow::borrow_constraint();
    module::mod_borrow::borrow2();
    module::mod_borrow::borrow3();

    module::mod_trait::trait1();
    module::mod_trait2::main();

    module::mod_type::type1();
    module::mod_type::shadowing();
    module::mod_type::type_as();

    module::mod_string::str_lib();

    module::mod_slice::slice();
    module::mod_slice::sequence();
    module::mod_slice::extract();

    module::mod_function::main();

    module::mod_syntax::syntax();

    module::mod_struct::main();

    module::mod_impl::main();
    module::mod_impl2::main();
    module::mod_impl3::main();

    module::mod_enum::main();

    module::mod_generics::main();

    module::mod_option::main();

    module::mod_match::main();
    module::mod_match2::main();
    module::mod_match3::main();
    module::mod_match_guard::main();

    module::mod_memory::main();

    error_lib::mod_error::file_process();

    concurrency::concurrency::main();

    // module2
    module2::mod_proprietorship::main();
    module2::mod_closure::closure();
    module2::mod_vec::main();
    module2::mod_box::main();
    module2::mod_iterator::main();

}

fn greet() -> &'static str {
    let a = "hello";
    let a2: &str;
    a2 = a;
    return a2
}