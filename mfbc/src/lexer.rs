#[derive(PartialEq, Debug, Clone)]
pub enum Commands {
    IncDp,     // >
    DecDp,     // <
    IncVal,    // +
    DecVal,    // -
    Output,    // .
    Input,     // ,
    LoopEnter, //[
    LoopEnd,   //]
    Unknown,   // eg. comments
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub command: Commands,
    pub pos: usize,
}

impl From<char> for Commands {
    fn from(ch: char) -> Self {
        match ch {
            '>' => Self::IncDp,
            '<' => Self::DecDp,
            '+' => Self::IncVal,
            '-' => Self::DecVal,
            '.' => Self::Output,
            ',' => Self::Input,
            '[' => Self::LoopEnter,
            ']' => Self::LoopEnd,
            _ => Self::Unknown,
        }
    }
}

/// # Lex source code and return tokens
///
/// Adds command and pos inside Token struct
/// Ignores Command::Unknown
///
/// ## Returns: Vector of Token
pub fn lex(source: &str) -> Vec<Token> {
    let source: Vec<char> = source.chars().collect();
    let mut result: Vec<Token> = Vec::new();

    let mut idx = 0;
    for ch in source {
        idx += 1;
        let command = Commands::from(ch);

        if let Commands::Unknown = command {
            continue;
        }

        result.push(Token { command, pos: idx });
    }

    result
}

#[cfg(test)]
mod commands_enum_tests {
    use super::*;

    #[test]
    fn test_incdp() {
        let token = Commands::from('>');
        assert_eq!(Commands::IncDp, token);
    }

    #[test]
    fn test_decdp() {
        let token = Commands::from('<');
        assert_eq!(Commands::DecDp, token);
    }
    #[test]
    fn test_incval() {
        let token = Commands::from('+');
        assert_eq!(Commands::IncVal, token);
    }
    #[test]
    fn test_decval() {
        let token = Commands::from('-');
        assert_eq!(Commands::DecVal, token);
    }
    #[test]
    fn test_output() {
        let token = Commands::from('.');
        assert_eq!(Commands::Output, token);
    }
    #[test]
    fn test_input() {
        let token = Commands::from(',');
        assert_eq!(Commands::Input, token);
    }
    #[test]
    fn test_loopenter() {
        let token = Commands::from('[');
        assert_eq!(Commands::LoopEnter, token);
    }
    #[test]
    fn test_loopend() {
        let token = Commands::from(']');
        assert_eq!(Commands::LoopEnd, token);
    }
    #[test]
    fn test_unknown() {
        let token = Commands::from('💩');
        assert_eq!(Commands::Unknown, token);
    }
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn test_lexer() {
        assert_eq!(
            lex(">Hello! This should get ignored :)<+-.,[]💩"),
            vec![
                Token {
                    command: Commands::IncDp,
                    pos: 1
                },
                Token {
                    command: Commands::DecDp,
                    pos: 35
                },
                Token {
                    command: Commands::IncVal,
                    pos: 36
                },
                Token {
                    command: Commands::DecVal,
                    pos: 37
                },
                Token {
                    command: Commands::Output,
                    pos: 38
                },
                Token {
                    command: Commands::Input,
                    pos: 39
                },
                Token {
                    command: Commands::LoopEnter,
                    pos: 40
                },
                Token {
                    command: Commands::LoopEnd,
                    pos: 41
                }
            ]
        )
    }
}
