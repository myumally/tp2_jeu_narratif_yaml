use std::io;
use serde_yaml;

#[derive(Debug)]
pub enum StoryError{
    UnvalidStartScene,
    NonUniqueIdScene,
    UnexistantChoice,
    IoError(io::Error),
    SerdeError(serde_yaml::Error)
}

impl From<io::Error> for StoryError {
    fn from(err: io::Error) -> Self {
        StoryError::IoError(err)
    }
}

impl From<serde_yaml::Error> for StoryError {
    fn from(err: serde_yaml::Error) -> Self {
        StoryError::SerdeError(err)
    }
}