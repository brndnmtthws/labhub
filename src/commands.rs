use regex::Regex;
use std::convert::TryFrom;

fn tokenize_comment(body: &str) -> Vec<&str> {
    body.split_whitespace().collect()
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommandAction {
    Retry,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub username: String,
    pub command: CommandAction,
    pub args: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    BadUsername,
    UnknownCommand,
    InvalidLength,
    InvalidFormat,
}

pub fn parse_body(body: &str, for_username: &str) -> Result<Command, CommandError> {
    Command::parse_from(body, for_username)
}

impl TryFrom<&str> for CommandAction {
    type Error = CommandError;
    fn try_from(body: &str) -> Result<Self, Self::Error> {
        match body.to_lowercase().as_ref() {
            "retry" => Ok(CommandAction::Retry),
            _ => Err(CommandError::UnknownCommand),
        }
    }
}

impl Command {
    fn parse_from(body: &str, for_username: &str) -> Result<Self, CommandError> {
        let tokens = tokenize_comment(body);
        if tokens.len() < 2 {
            return Err(CommandError::InvalidLength);
        }
        lazy_static! {
            static ref RE: Regex = Regex::new("^@(.*)$").unwrap();
        }
        Ok(Command {
            username: match RE.captures(tokens[0]) {
                Some(cap) => {
                    let username = cap[1].to_string();
                    if username != for_username {
                        return Err(CommandError::BadUsername);
                    } else {
                        username
                    }
                }
                _ => return Err(CommandError::InvalidFormat),
            },
            command: CommandAction::try_from(tokens[1])?,
            args: tokens
                .iter()
                .skip(2)
                .map(std::string::ToString::to_string)
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::run_test;

    #[test]
    fn test_tokenize_comment() {
        run_test(|| {
            assert_eq!(tokenize_comment("hello fren"), vec!["hello", "fren"]);
            assert_eq!(tokenize_comment("    hello fren"), vec!["hello", "fren"]);
            assert_eq!(tokenize_comment("hello fren     "), vec!["hello", "fren"]);
            assert_eq!(
                tokenize_comment("    hello    fren    "),
                vec!["hello", "fren"]
            );
        });
    }

    #[test]
    fn test_from_string() {
        run_test(|| {
            assert_eq!(
                Command::parse_from("lol", "bot").unwrap_err(),
                CommandError::InvalidLength
            );
            assert_eq!(
                Command::parse_from("herp derp nerp", "bot").unwrap_err(),
                CommandError::InvalidFormat
            );
            assert_eq!(
                Command::parse_from("@bot derp nerp", "bot").unwrap_err(),
                CommandError::UnknownCommand
            );
            assert_eq!(
                Command::parse_from("@bot retry nerp", "bot")
                    .unwrap()
                    .command,
                CommandAction::Retry
            );
            assert_eq!(
                Command::parse_from("@bot retry nerp", "bot").unwrap().args,
                vec!["nerp"]
            );
        });
    }

    #[test]
    fn test_is_valid() {
        run_test(|| {
            let command = Command::parse_from("@bot retry nerp", "bot");
            assert_eq!(command.is_ok(), true);
            assert_eq!(
                command.ok(),
                Some(Command {
                    username: "bot".into(),
                    command: CommandAction::Retry,
                    args: vec!["nerp".to_string()]
                })
            );
        });
    }

    #[test]
    fn test_wrong_username() {
        run_test(|| {
            let command = Command::parse_from("@not retry nerp", "bot");
            assert_eq!(command.is_err(), true);
            assert_eq!(command.err(), Some(CommandError::BadUsername));
        });
    }

    #[test]
    fn test_parse_body() {
        run_test(|| {
            assert_eq!(
                parse_body("@bot retry nerp", "bot").unwrap().command,
                CommandAction::Retry
            );
            assert_eq!(
                parse_body("@not retry nerp", "bot").unwrap_err(),
                CommandError::BadUsername
            );
        });
    }
}
