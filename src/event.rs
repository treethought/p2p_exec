use core::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExecEvent {
    // RegisterEvent,
    Request(ExecRequest),
    Response(ExecResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExecFunction {
    Add,
    Subtract,
    Divide,
    Multiply,
}



struct RegisterEvent {
    topic: String,
    version: u8,
    function: ExecFunction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecRequest {
    pub function: ExecFunction,
    pub args: (u64, u64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecResponse {
    pub result: u64,
}

impl FromStr for ExecFunction {
    type Err = ();

    fn from_str(input: &str) -> Result<ExecFunction, Self::Err> {
        let lower = input.to_lowercase();
        match &lower[..] {
            "add" => Ok(Self::Add),
            "subtract" | "sub" => Ok(Self::Subtract),
            "divide" | "div" => Ok(Self::Divide),
            "multiply" | "mult" => Ok(Self::Multiply),
            _ => Err(()),
        }
    }
}
