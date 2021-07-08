use std::str::FromStr;
use std::convert::Infallible;

#[derive(PartialEq, Debug)]
pub enum Language {
    C,
    CC,
    COBOL,

    D,

    Erlang,

    FSharp,
    Forth,

    Golang,

    Haskell,

    Lua,

    VisualBasic,

    Other(String),
}

impl FromStr for Language {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.clone().to_lowercase();

        let ret = match lowercase.as_ref() {
            "c" => Self::C,
            "c++" | "cc" | "cpp" | "cxx" => Self::CC,
            "cobol" => Self::COBOL,
            "d" => Self::D,
            "erlang" => Self::Erlang,
            "fsharp" => Self::FSharp,
            "forth" => Self::Forth,
            "golang" => Self::Golang,
            "haskell" => Self::Haskell,
            "lua" => Self::Lua,
            "visual basic" => Self::VisualBasic,
            _ => Self::Other(lowercase),
        };

        Ok(ret)
    }
}
