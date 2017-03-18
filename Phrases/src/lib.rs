pub mod greetings {
    pub mod english; //exported into ./greetings/english.rs -- rust will automatically look for these
    pub mod french {
        pub fn hello() -> String { "bonjour".to_string() }
        pub fn goodbye() -> String { "au revoir".to_string() }
    }
}

//built in testing support
//only works when you're pointed to the directory
#[test]
fn english_greeting_correct() {
    assert_eq!("hello", greetings::english::hello());
}
