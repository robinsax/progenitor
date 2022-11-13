use super::super::schema::{Type, Value, Expression};
use super::super::store::Store;
use super::errors::EffectError;
use super::effect::effect_fn;
use super::context::Context;

#[apply(effect_fn)]
pub async fn open_store_effect<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let driver_name: String = archetype.lookup("driver")?.try_into()?;
    let store_name: String = archetype.lookup("name")?.try_into()?;
    let schema: Type = archetype.lookup("schema")?.try_into()?;

    let store = context.registry().create_store(schema, &driver_name, store_name.clone())?;

    context.set(store_name, store)?;

    Ok(())
}

#[apply(effect_fn)]
pub async fn load_from_store<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let store_name: String = archetype.lookup("from_store")?.try_into()?;
    let state_key_name: String = archetype.lookup("to_state")?.try_into()?;
    let filter = Expression::parse_from_value(archetype.lookup("filter")?)?;

    let store = context.get::<Store>(store_name)?;

    let result = store.query().filter(filter).all().await?;

    context.set(state_key_name, result)?;

    Ok(())
}

#[apply(effect_fn)]
pub async fn write_to_store<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let store_name: String = archetype.lookup("to_store")?.try_into()?;
    let state_key_name: String = archetype.lookup("from_state")?.try_into()?;

    let value = context.get::<Value>(state_key_name)?;

    let store = context.get::<Store>(store_name)?;

    store.put(value.clone()).await?;

    Ok(())
}
