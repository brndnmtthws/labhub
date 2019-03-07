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

#[derive(Debug)]
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

impl Command {
    fn is_valid(&self, username: &str) -> Result<(), CommandError> {
        if self.username == username {
            Ok(())
        } else {
            Err(CommandError::BadUsername)
        }
    }
}

pub fn parse_body(body: &str, for_username: &str) -> Result<Command, CommandError> {
    let command = Command::try_from(body)?;
    command.is_valid(for_username)?;
    Ok(command)
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

impl TryFrom<&str> for Command {
    type Error = CommandError;

    fn try_from(body: &str) -> Result<Self, Self::Error> {
        let tokens = tokenize_comment(body);
        if tokens.len() < 2 {
            return Err(CommandError::InvalidLength);
        }
        lazy_static! {
            static ref RE: Regex = Regex::new("^@(.*)$").unwrap();
        }
        Ok(Command {
            username: match RE.captures(tokens[0]) {
                Some(cap) => cap[1].to_string(),
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
                Command::try_from("lol").unwrap_err(),
                CommandError::InvalidLength
            );
            assert_eq!(
                Command::try_from("herp derp nerp").unwrap_err(),
                CommandError::InvalidFormat
            );
            assert_eq!(
                Command::try_from("@bot derp nerp").unwrap_err(),
                CommandError::UnknownCommand
            );
            assert_eq!(
                Command::try_from("@bot retry nerp").unwrap().command,
                CommandAction::Retry
            );
            assert_eq!(
                Command::try_from("@bot retry nerp").unwrap().args,
                vec!["nerp"]
            );
        });
    }

    #[test]
    fn test_is_valid() {
        run_test(|| {
            let command = Command::try_from("@bot retry nerp").unwrap();
            assert_eq!(command.is_valid("bot").is_ok(), true);
            assert_eq!(command.is_valid("not").is_err(), true);
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
