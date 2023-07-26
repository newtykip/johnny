use color_eyre::Report;
use common::Data;
use poise::FrameworkError;

macro_rules! load_error {
    ($($name: ident)*) => {
        $(
            mod $name;
            pub use $name::run as $name;
        )*
    };
}

load_error!(command);

pub async fn error_handler(error: FrameworkError<'_, Data, Report>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            panic!("Failed to start bot: {:?}", error)
        }
        FrameworkError::Command { error, ctx } => command(error, ctx).await,
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                panic!("Error while handling error: {:?}", e);
            }
        }
    }
}
