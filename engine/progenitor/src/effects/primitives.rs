use super::super::schema::{Type, Value, Condition};
use super::super::store::Store;
use super::errors::EffectError;
use super::effect::effect_fn;
use super::context::Context;

#[apply(effect_fn)]
pub async fn open_store<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let driver_name: String = archetype.lookup("driver")?.try_into()?;
    let store_name: String = archetype.lookup("name")?.try_into()?;
    let schema: Type = archetype.lookup("schema")?.try_into()?;

    let store = context.registry().create_store(schema, &driver_name, store_name.clone())?;

    context.set(store_name, store)?;

    Ok(())
}

#[apply(effect_fn)]
pub async fn store_read<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let store_name: String = archetype.lookup("from_store")?.try_into()?;
    let state_key_name: String = archetype.lookup("to_state")?.try_into()?;

    let store = context.get::<Store>(store_name)?;

    let mut query = store.query();
    if let Ok(filter_value) = archetype.lookup("filter") {
        query = query.filter(Condition::parse_from_value(filter_value)?);
    }

    let one: bool = archetype.lookup("one").is_ok();

    if one {
        context.set(state_key_name, query.one().await?)?;
    }
    else {
        context.set(state_key_name, Value::List(query.all().await?))?;
    }

    Ok(())
}

#[apply(effect_fn)]
pub async fn store_write<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let archetype = context.archetype()?;

    let store_name: String = archetype.lookup("to_store")?.try_into()?;
    let state_key_name: String = archetype.lookup("from_state")?.try_into()?;

    let value = context.get::<Value>(state_key_name)?;

    let store = context.get::<Store>(store_name)?;

    store.put(value.clone()).await?;

    Ok(())
}
