macro_rules! load_migration {
    ($($name:ident),+) => {
        $(
			#[allow(non_snake_case)]
            mod $name;
			pub use $name::migration as $name;
        )*
    };
}

use load_migration;

load_migration!(U1__guild, U2__user, U3__member, U4__autorole, U5__sticky);
