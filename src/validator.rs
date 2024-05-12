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

    pub fn validate(&self, value: &str) -> Result<(), String> {
        match self {
            Self::String => Ok(()),
            Self::Bool => {
                if ["true", "false"].contains(&value) {
                    Ok(())
                } else {
                    Err(format!("Value does not match to schema. value: {}, schema: bool", value))
                }
            }
            Self::Int => {
                match value.parse::<i32>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Value does not match to schema. value: {}, schema: int", value))
                }
            }
            Self::Float => {
                match value.parse::<f64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Value does not match to schema. value: {}, schema: float", value))
                }
            }
        }
    }
}

#[cfg(test)]
mod from {
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

#[cfg(test)]
mod validate {
    use super::*;

    #[test]
    fn string_ok() {
        let validator = Validator::String;
        let actual = validator.validate("hello");
        assert_eq!(actual, Ok(()));
    }

    #[test]
    fn bool_ok() {
        let validator = Validator::Bool;
        let actual = validator.validate("true");
        assert_eq!(actual, Ok(()));
        let actual = validator.validate("false");
        assert_eq!(actual, Ok(()));
    }

    #[test]
    fn bool_err() {
        let validator = Validator::Bool;
        let actual = validator.validate("hello");
        assert_eq!(actual, Err("Value does not match to schema. value: hello, schema: bool".to_string()));
    }

    #[test]
    fn int_ok() {
        let validator = Validator::Int;
        let actual = validator.validate("10");
        assert_eq!(actual, Ok(()));
    }

    #[test]
    fn int_err() {
        let validator = Validator::Int;
        let actual = validator.validate("hello");
        assert_eq!(actual, Err("Value does not match to schema. value: hello, schema: int".to_string()));
    }

    #[test]
    fn float_ok() {
        let validator = Validator::Float;
        let actual = validator.validate("10.5");
        assert_eq!(actual, Ok(()));
    }

    #[test]
    fn float_err() {
        let validator = Validator::Float;
        let actual = validator.validate("hello");
        assert_eq!(actual, Err("Value does not match to schema. value: hello, schema: float".to_string()));
    }
}
