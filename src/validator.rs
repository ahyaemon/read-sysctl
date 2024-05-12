#[derive(Debug, PartialEq)]
pub enum Validator {
    String,
    Bool,
    Int,
    Float,
}

impl Validator {

    pub fn from(value: &str) -> Result<Self, String> {
        match value {
            "string" => Ok(Self::String),
            "bool" => Ok(Self::Bool),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            _ => Err(format!("Invalid schema type: {}", value)),
        }
    }
}

pub fn validate_line(line: &str, validator: Validator) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let validator = Validator::from("string").unwrap();
        assert_eq!(validator, Validator::String);
    }

    #[test]
    fn bool() {
        let validator = Validator::from("bool").unwrap();
        assert_eq!(validator, Validator::Bool);
    }

    #[test]
    fn int() {
        let validator = Validator::from("int").unwrap();
        assert_eq!(validator, Validator::Int);
    }

    #[test]
    fn float() {
        let validator = Validator::from("float").unwrap();
        assert_eq!(validator, Validator::Float);
    }
}
