use crate::command::{ChooseCommand, GameCommand, InventoryCommand, LookCommand, QuitCommand, StatusCommand};

#[derive(Debug)]
pub struct ParseError;

pub fn parse_command(line: &str) -> Result<Box<dyn GameCommand>, ParseError>{
    match line {
        "look\n" => Ok(Box::new(LookCommand)),
        "inventory\n" => Ok(Box::new(InventoryCommand)),
        "status\n" => Ok(Box::new(StatusCommand)),
        "quit\n" => Ok(Box::new(QuitCommand)),
        _ if line.starts_with("choose ") => {
            let nb = line["choose ".len()..].trim();
            match nb.parse::<usize>() {
                Ok(n) => Ok(Box::new(ChooseCommand { choice: n })),
                Err(_) => Err(ParseError),
            }
        },
        _ => Err(ParseError)
    }
}