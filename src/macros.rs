/// Loads an event from its module. First argument is the name of the event, the second is the predicate to enable it.
#[macro_export]
macro_rules! load_event {
    // no predicate - name
    ($($name: ident)*) => {
        $(
            mod $name;
            pub use $name::$name;
        )*
    };
    // depends on predicate - name | predicate
    ($($name:ident | $predicate:ident)*) => {
        $(
            #[cfg($predicate)]
            mod $name;
            #[cfg($predicate)]
            pub use $name::$name;
        )*
    };
}

/// Loads a command from its module. First argument is the name of the command, the second is the predicate to enable it.
#[macro_export]
macro_rules! load_command {
    // no predicate - name
    ($($name: ident)*) => {
        $(
            mod $name;
            pub use $name::$name;
        )*
    };
    // depends on predicate - name | predicate
    ($($name: ident | $predicate: ident)*) => {
        $(
            #[cfg($predicate)]
            mod $name;
            #[cfg($predicate)]
            pub use $name::$name;
        )*
    }
}

#[macro_export]
macro_rules! use_embed {
    ($embed: expr, $base_embed: expr) => {
        {
            $embed.clone_from(&$base_embed);
            $embed
        }
    };
    ($embed: expr, $base_embed: expr, $code: block) => {
        {
            $embed.clone_from(&$base_embed);
            $code
            $embed
        }
    }
}
