use std::fmt;

use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Diagnostic, Error, Serialize)]
#[diagnostic(help("Make sure SteamVR is installed."))]
pub struct InitializationError {
    pub name: String,
    pub code: u32,
}

impl fmt::Display for InitializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ToAgent {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationName {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FromAgent {
    InitializationCompleted,
    InitializationError(InitializationError),
    ApplicationName(Option<ApplicationName>),
}
