/// Loads an event from its module. First argument is the name of the event, the second is the predicate to enable it.
#[macro_export]
macro_rules! load_event {
    ($name:ident) => {
        mod $name;
        pub use $name::run as $name;
    };
    ($name:ident,$predicate:ident) => {
        #[cfg($predicate)]
        mod $name;
        #[cfg($predicate)]
        pub use $name::run as $name;
    };
}

/// Loads a command from its module. First argument is the name of the command, the second is the predicate to enable it.
#[macro_export]
macro_rules! load_command {
    ($name:ident) => {
        mod $name;
        pub use $name::$name;
    };
    ($name:ident,$predicate:ident) => {
        #[cfg($predicate)]
        mod $name;
        #[cfg($predicate)]
        pub use $name::$name;
    };
}
