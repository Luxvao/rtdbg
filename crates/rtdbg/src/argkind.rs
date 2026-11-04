use serde_json::Value;

use crate::repl::Error;

#[derive(Debug, Clone)]
pub enum ArgKind {
    Int(i64),
    ScriptId(u64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<ArgKind>),
    Garbage(String),
}

#[derive(Copy, Clone, Debug)]
enum ParseState {
    Normal,
    String,
}

impl TryInto<Value> for ArgKind {
    type Error = Error;

    fn try_into(self) -> Result<Value, Self::Error> {
        Ok(match self {
            ArgKind::Int(int) => serde_json::to_value(int)?,
            ArgKind::ScriptId(id) => serde_json::to_value(id)?,
            ArgKind::Float(float) => serde_json::to_value(float)?,
            ArgKind::String(str) => serde_json::to_value(str)?,
            ArgKind::Bool(b) => serde_json::to_value(b)?,
            ArgKind::Array(arg_kinds) => {
                let values = arg_kinds
                    .into_iter()
                    .map(|a| a.try_into())
                    .collect::<Result<Vec<Value>, _>>()?;

                serde_json::to_value(values)?
            }
            ArgKind::Garbage(_) => return Err(Error::RefusingToSerialiseGarbage),
        })
    }
}

impl ArgKind {
    // Parse everything up to the end of the line
    pub fn parse(str: &str) -> Result<Vec<ArgKind>, Error> {
        let (mut parsed, rest) = ArgKind::parse_partial(str)?;

        if !rest.trim().is_empty() {
            parsed.push(ArgKind::parse_sub(&rest)?);
        }

        Ok(parsed)
    }

    // Parses what's finished and returns a string of what's left
    pub fn parse_partial(str: &str) -> Result<(Vec<ArgKind>, String), Error> {
        let mut state = ParseState::Normal;

        let mut scratch = String::new();

        let mut arrays: Vec<Vec<ArgKind>> = vec![Vec::new()];

        for (index, char) in str.chars().enumerate() {
            match state {
                ParseState::Normal => match char {
                    '"' => {
                        scratch.push('"');
                        state = ParseState::String;
                    }
                    '[' => {
                        arrays.push(vec![]);
                    }
                    ']' => {
                        // On ] we also clean scratch just the same as on ',' if we actually have anything in there
                        if !scratch.trim().is_empty() {
                            let scratch = std::mem::take(&mut scratch);

                            arrays
                                .last_mut()
                                .ok_or(Error::MismatchedParens(index))?
                                .push(ArgKind::parse_sub(&scratch)?);
                        }

                        let curr_arr = arrays.pop().ok_or(Error::MismatchedParens(index))?;

                        arrays
                            .last_mut()
                            .ok_or(Error::MismatchedParens(index))?
                            .push(ArgKind::Array(curr_arr));
                    }
                    ',' => {
                        if !scratch.trim().is_empty() {
                            let scratch = std::mem::take(&mut scratch);

                            arrays
                                .last_mut()
                                .ok_or(Error::MismatchedParens(index))?
                                .push(ArgKind::parse_sub(&scratch)?);
                        }
                    }
                    c => {
                        scratch.push(c);
                    }
                },
                ParseState::String => {
                    if char == '"' {
                        scratch.push('"');
                        state = ParseState::Normal;
                    } else {
                        scratch.push(char);
                    }
                }
            }
        }

        if arrays.len() != 1 {
            return Err(Error::InvalidInput);
        }

        Ok((
            arrays
                .into_iter()
                .next()
                .ok_or(Error::MismatchedParens(0))?,
            scratch.to_string(),
        ))
    }

    pub fn parse_sub(str: &str) -> Result<ArgKind, Error> {
        let str = str.trim();

        // String
        if let Some(string) = str.strip_circumfix('"', '"') {
            return Ok(ArgKind::String(string.to_string()));
        }

        // Int
        if let Ok(int) = str.parse::<i64>() {
            return Ok(ArgKind::Int(int));
        }

        // ScriptId
        if let Some(str) = str.strip_prefix('#') {
            if let Ok(id) = str.parse::<u64>() {
                return Ok(ArgKind::ScriptId(id));
            }
        }

        // Float
        if let Ok(float) = str.parse::<f64>() {
            return Ok(ArgKind::Float(float));
        }

        // Bool
        if let Ok(bool) = str.parse::<bool>() {
            return Ok(ArgKind::Bool(bool));
        }

        // Anything weird
        Ok(ArgKind::Garbage(str.to_string()))
    }
}
