macro_rules! define_commands {
    ($($variant:ident => $name:expr),* $(,)?) => {
        pub enum Command {
            $($variant),*
        }

        impl Command {
            pub fn from_str(input: &str) -> Option<Self> {
                match input {
                    $($name => Some(Self::$variant),)*
                    _ => None,
                }
            }

            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }

            pub fn variants() -> Vec<&'static str> {
                vec![$($name),*]
            }
        }
    };
}

define_commands! {
    UpdatePlan => "update",
    ExecutePlan => "execute",
    Show => "show",
    Reset => "reset",
    Help => "help",
    Exit => "exit",
}
