// proc_macro is obviously an option for codegen, but the idea here is we want
// models and logic to be transparent, which proc macro isn't.
mod scribe;
mod schema;
mod effect;

use bytes::Bytes;

use progenitor::Value;

use super::errors::ExecError;

use self::effect::author_effect;
use self::scribe::Scribe;
use self::schema::author_schema_fn;

pub struct AuthorInput {
    pub value: Value,
    pub as_module: bool
}

pub fn author(input: AuthorInput) -> Result<Bytes, ExecError> {
    let archetype: String = input.value.lookup("archetype")?.try_into()?;
    let name: String = input.value.lookup("name")?.try_into()?;
    let value = input.value.lookup("value")?;

    let mut scribe = Scribe::new(1024);

    scribe = match archetype.as_str() {
        "schema" => author_schema_fn(scribe, name, value)?,
        "effect" => author_effect(scribe, name, value)?,
        _ => return Err(ExecError::Io(format!("invalid archetype {}", archetype)))
    };

    Ok(scribe.into_result(input.as_module))
}
