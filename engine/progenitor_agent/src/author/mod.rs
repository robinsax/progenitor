// proc_macro is obviously an option for codegen, but the idea here is we want
// models and logic to be transparent, which proc macro isn't.
mod scribe;
mod value;
mod schema;
mod effect;

use bytes::Bytes;

use progenitor::Value;

use super::errors::ExecError;

use self::effect::author_effect;
use self::scribe::Scribe;
use self::schema::author_schema_fn;
use self::value::author_value;

pub struct AuthorInput {
    pub value: Value,
    pub as_module: bool
}

pub fn author(input: AuthorInput) -> Result<Bytes, ExecError> {
    let archetype: String = input.value.lookup("archetype")?.try_into()?;
    let name = input.value.lookup("name");
    let value = input.value.lookup("value")?;

    let mut scribe = Scribe::new(1024);

    scribe = match archetype.as_str() {
        "value" => author_value(scribe, value)?,
        "type" => author_schema_fn(scribe, name?.try_into()?, value)?,
        "effect" => author_effect(scribe, name?.try_into()?, value)?,
        _ => return Err(ExecError::Io(format!("invalid archetype {}", archetype)))
    };

    Ok(scribe.into_result(input.as_module))
}
