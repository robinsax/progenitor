use std::collections::HashMap;
use std::str::Chars;

use crate::schema::Value;

use super::errors::SerialError;
use super::value::SerialValue;
use super::format::SerialFormat;

struct JsonParser<'ps> {
    position: u32,
    input: Chars<'ps>,
    peeked: Option<Option<char>>
}

// TODO: Null.
// TODO: Malicious input protection.
// All parse methods assume the invariant that the first token they're going to consume
// is valid for the given to-be-parsed type.
impl<'ps> JsonParser<'ps> {
    fn new(input: &'ps str) -> Self {
        Self {
            input: input.chars(),
            position: 0,
            peeked: None
        }
    }

    fn error(&self, message: &'static str) -> SerialError {
        SerialError::Parse(format!("Syntax error at position {}: {}", self.position, message).into())
    }

    fn raw_consume(&mut self, incl_ws: bool) -> Option<char> {
        loop {
            self.position += 1;

            match self.input.next() {
                Some(token) => {
                    if incl_ws || (token != ' ' && token != '\n' && token != '\r') {
                        return Some(token);
                    }
                },
                None => return None
            };
        }
    }

    fn peek_optional(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = Some(self.raw_consume(false));
        }

        self.peeked.unwrap()
    }

    fn peek(&mut self) -> Result<char, SerialError> {
        self.peek_optional().ok_or_else(|| self.error("Unterminated value"))
    }

    fn next_optional(&mut self) -> Option<char> {
        if let Some(next) = self.peeked {
            self.peeked = None;

            next
        }
        else {
            self.raw_consume(false)
        }
    }

    fn next(&mut self) -> Result<char, SerialError> {
        self.next_optional().ok_or_else(|| self.error("Unterminated value"))
    }

    fn raw_parse_string(&mut self) -> Result<String, SerialError> {
        let mut is_escape = false;
        // TODO: Adaptive capacity.
        let mut parsed = String::with_capacity(32);

        self.next()?;

        loop {
            // Careful, peeking above here will break things.
            let token = self.raw_consume(true).ok_or_else(|| self.error("Unterminated string"))?;

            if token == '"' && !is_escape {
                break;
            }

            if token == '\\' {
                is_escape = !is_escape;
            }

            parsed.push(token);
        }

        Ok(parsed)
    }

    fn parse_number(&mut self) -> Result<Value, SerialError> {
        let mut whole = 0;
        let mut negative = false;
        let mut fractional_digits = 0;
        let mut fractional = None;

        let first_token = self.peek()?;
        if first_token == '-' {
            negative = true;

            self.next()?;
        }

        loop {
            let token = self.next()?;

            if token == '.' {
                if fractional.is_some() {
                    return Err(self.error("Repeat decimal token"));
                }

                fractional = Some(0);
                continue;
            }

            let part = token.to_digit(10)
                .ok_or_else(|| self.error("Invalid token for number"))?;

            if let Some(current) = fractional {
                fractional = Some((current * 10) + part);
                fractional_digits += 1;
            }
            else {
                whole = (whole * 10) + part;
            }

            let maybe_next_token = self.peek_optional();
            if let Some(next_token) = maybe_next_token {
                if next_token != '.' && !next_token.is_numeric() {
                    break;
                }
            }
            else {
                break;
            }
        }

        // Return value resolved as:
        // Float64 if it has a fraction part,
        // Int32 if it is explicitly signed (negative),
        // Uint32 otherwise.

        if let Some(frac) = fractional {
            let mut real = whole as f64 + ((frac as f64) / ((10_i32.pow(fractional_digits)) as f64));
            if negative {
                real *= -1.0;
            }

            Ok(Value::Float64(real))
        }
        else if negative {
            match i32::try_from(whole) {
                Ok(signed) => Ok(Value::Int32(signed * -1)),
                Err(_) => Err(self.error("Numeric overflow"))
            }
        }
        else {
            Ok(Value::Uint32(whole))
        }
    }

    fn parse_string(&mut self) -> Result<Value, SerialError> {
        Ok(Value::Str(self.raw_parse_string()?))
    }

    fn parse_object(&mut self) -> Result<Value, SerialError> {
        // TODO: Adaptive capacity.
        let mut result = HashMap::with_capacity(8);

        self.next()?;

        loop {
            let token = self.peek()?;

            if token == '}' {
                self.next()?;
                break;
            }

            let key = self.raw_parse_string()?;

            if self.next()? != ':' {
                return Err(self.error("Object key without trailing :"))                
            }

            let value = self.parse()?;

            result.insert(key, value);

            let next_token = self.peek()?;
            if next_token == ',' {
                self.next()?;
            }
            else if next_token != '}' {
                return Err(self.error("Invalid token after object value position"));
            }
        }

        Ok(Value::Map(result))
    }

    fn parse_array(&mut self) -> Result<Value, SerialError> {
        // TODO: Adaptive capacity.
        let mut result = Vec::with_capacity(8);

        self.next()?;

        loop {
            let token = self.peek()?;

            if token == ']' {
                self.next()?;
                break;
            }

            result.push(self.parse()?);
            
            let next_token = self.peek()?;
            if next_token == ',' {
                self.next()?;
            }
            else if next_token != ']' {
                return Err(self.error("Invalid token after array element position"));
            }
        }

        Ok(Value::List(result))
    }

    fn parse(&mut self) -> Result<Value, SerialError> {
        let next_token = self.peek()?;

        if next_token == '-' || next_token.is_numeric() {
            return self.parse_number();
        }

        match next_token {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string(),
            _ => Err(self.error("Invalid token in value position"))
        }
    }
}

struct JsonWriter<'wr> {
    output: String,
    input: &'wr Value
}

// TODO: Formatting.
impl<'wr> JsonWriter<'wr> {
    fn new(input: &'wr Value) -> Self {
        Self {
            input,
            // TODO: Intelligent capacity. sizeof?
            output: String::with_capacity(128)
        }
    }

    fn raw_append(&mut self, string: &str) {
        self.output.push_str(&string)
    }

    fn append_string(&mut self, string: &str) {
        self.raw_append("\"");

        let mut did_escape = false;
        for token in string.chars() {
            // TODO: Pretty sure it's more complex than this.
            if token == '"' {
                self.output.push('\\');
            }

            self.output.push(token);

            did_escape = token == '\\';
        }

        if did_escape {
            self.raw_append("\\");
        }
        self.raw_append("\"");
    }

    fn append_object(&mut self, contents: &HashMap<String, Value>) {
        self.raw_append("{");

        let mut keys = contents.keys().collect::<Vec<&String>>();
        keys.sort();

        let max = contents.len() - 1;
        let mut idx = 0;
        for key in keys {
            let value = contents.get(key).unwrap();

            self.append_string(key);
            self.raw_append(":");
            self.append_value(value);

            if idx != max {
                self.raw_append(",");
            }
            idx += 1;
        }

        self.raw_append("}");
    }

    fn append_array(&mut self, contents: &Vec<Value>) {
        self.raw_append("[");

        let mut idx = 0;
        for value in contents.iter() {
            self.append_value(value);

            if idx != contents.len() - 1 {
                self.raw_append(",");
            }
            idx += 1;
        }

        self.raw_append("]");
    }

    fn append_value(&mut self, value: &Value) {
        match value {
            Value::Null => self.raw_append("null"),
            // TODO: What's faster than format!?
            Value::Bool(flag) => self.raw_append(&format!("{}", flag)),
            Value::Uint32(num) => self.raw_append(&format!("{}", num)),
            Value::Int32(num) => self.raw_append(&format!("{}", num)),
            Value::Float64(num) => self.raw_append(&format!("{}", num)),
            Value::Str(string) => self.append_string(string),
            Value::Map(contents) => self.append_object(contents),
            Value::List(contents) => self.append_array(contents)
        };
    }

    fn write(mut self) -> Result<String, SerialError> {
        self.append_value(self.input);

        Ok(self.output)
    }
}

pub struct JsonSerial;

impl SerialFormat for JsonSerial {
    fn parse(&self, serial: SerialValue) -> Result<Value, SerialError> {
        let string = match String::from_utf8(serial.try_into_bytes()?.into()) {
            Ok(string) => string,
            Err(_) => return Err(SerialError::Parse("Invalid JSON string encoding".into()))
        };

        JsonParser::new(&string).parse()
    }

    fn write(&self, value: &Value) -> Result<SerialValue, SerialError> {
        let string = JsonWriter::new(value).write()?;

        Ok(SerialValue::from_string(string))
    }
}

impl JsonSerial {
    pub fn new() -> Self {
        Self { }
    }
}

// TODO: Parse fail successfully tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(
            JsonParser::new("1.145").parse(),
            Ok(Value::Float64(1.145))
        );
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            JsonParser::new("\"foo bar\"").parse(),
            Ok(Value::Str("foo bar".into()))
        );
    }

    #[test]
    fn parse_array() {
        assert_eq!(
            JsonParser::new("[1, 2, 3]").parse(),
            Ok(Value::List(Vec::from([
                Value::Uint32(1),
                Value::Uint32(2),
                Value::Uint32(3)
            ])))
        );

        assert_eq!(
            JsonParser::new("[\"foo\", \"bar\"]").parse(),
            Ok(Value::List(Vec::from([
                Value::Str("foo".into()),
                Value::Str("bar".into())
            ])))
        );
    }

    #[test]
    fn parse_object() {
        assert_eq!(
            JsonParser::new("{\"a\": 1, \"b\": \"foo\"}").parse(),
            Ok(Value::Map(HashMap::from([
                ("a".to_owned(), Value::Uint32(1)),
                ("b".to_owned(), Value::Str("foo".into()))
            ])))
        );
    }

    #[test]
    fn parse_complex() {
        assert_eq!(
            JsonParser::new("{\"foo\": [1, -2, -1.5, \"bar\"]}").parse(),
            Ok(Value::Map(HashMap::from([
                ("foo".to_owned(), Value::List(Vec::from([
                    Value::Uint32(1),
                    Value::Int32(-2),
                    Value::Float64(-1.5),
                    Value::Str("bar".into())
                ])))
            ])))
        );
    }

    #[test]
    fn write_null() {
        assert_eq!(
            JsonWriter::new(&Value::Null).write(),
            Ok("null".into())
        );
    }

    #[test]
    fn write_number() {
        assert_eq!(
            JsonWriter::new(&Value::Uint32(4)).write(),
            Ok("4".into())
        );

        assert_eq!(
            JsonWriter::new(&Value::Int32(-4)).write(),
            Ok("-4".into())
        );

        assert_eq!(
            JsonWriter::new(&Value::Float64(-4.2)).write(),
            Ok("-4.2".into())
        );
    }

    #[test]
    fn write_string() {
        assert_eq!(
            JsonWriter::new(&Value::Str("foo \"bar\"".into())).write(),
            Ok("\"foo \\\"bar\\\"\"".into())
        );

        assert_eq!(
            JsonWriter::new(&Value::Str("foo \"bar\"\\".into())).write(),
            Ok("\"foo \\\"bar\\\"\\\\\"".into())
        );
    }

    #[test]
    fn write_array() {
        assert_eq!(
            JsonWriter::new(&Value::List(Vec::from([
                Value::Str("foo".into()),
                Value::Float64(-2.2),
                Value::Null
            ]))).write(),
            Ok("[\"foo\",-2.2,null]".into())
        );
    }

    #[test]
    fn write_object() {
        assert_eq!(
            JsonWriter::new(&Value::Map(HashMap::from([
                ("foo".into(), Value::Str("bar".into())),
                ("a".into(), Value::List(Vec::from([
                    Value::Null,
                    Value::Int32(-5)
                ])))
            ]))).write(),
            Ok("{\"a\":[null,-5],\"foo\":\"bar\"}".into())
        );
    }
}
