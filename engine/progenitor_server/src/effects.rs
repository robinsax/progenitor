use progenitor::{EffectError, Type, Context, Value, effect_fn};

use super::comm::{Response, Request};

#[apply(effect_fn)]
pub async fn write_resp<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let format_name: String = archetype.lookup("format")?.try_into()?;
    let state_key_name: String = archetype.lookup("from_state")?.try_into()?;

    let value = context.get::<Value>(state_key_name)?;

    let format = context.registry().get_serial_format(format_name.as_str())?;

    let response = Response::new(format.write(value)?);

    context.set("resp", response)?;

    Ok(())
}

#[apply(effect_fn)]
pub async fn read_req<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let format_name: String = archetype.lookup("format")?.try_into()?;
    let state_key_name: String = archetype.lookup("to_state")?.try_into()?;
    let validate_as: Type = archetype.lookup("schema")?.try_into()?;

    let req = context.get::<Request>("req")?;

    let format = context.registry().get_serial_format(format_name.as_str())?;

    // TODO: Clone really dumb.
    let value = format.parse(req.payload().clone())?;

    validate_as.validate(&value)?;

    context.set(state_key_name, value)?;

    Ok(())
}
