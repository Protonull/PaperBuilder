use colored::Colorize;

pub(crate) trait ExitOnError<T> {
    fn unwrap_or_exit(self, message: &str) -> T;
}

impl<T> ExitOnError<T> for Option<T> {
    fn unwrap_or_exit(self, message: &str) -> T {
        match self {
            Some(value) => {
                return value;
            },
            None => {
                println!("{}", message.yellow());
                std::process::exit(1);
            },
        }
    }
}

impl<T, E> ExitOnError<T> for Result<T, E> {
    fn unwrap_or_exit(self, message: &str) -> T {
        match self {
            Ok(value) => {
                return value;
            }
            Err(_) => {
                println!("{}", message.yellow());
                std::process::exit(1);
            }
        }
    }
}
