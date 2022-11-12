use progenitor::Value;

use super::super::errors::ExecError;
use super::scribe::Scribe;

fn author_read_state(mut scribe: Scribe, params: &Value) -> Result<Scribe, ExecError> {
    let type_value = params.lookup("type")?;
    let type_src: String = type_value.index(0)?.try_into()?;
    let type_name: String = type_value.index(1)?.try_into()?;
    let var_name: String = params.lookup("name")?.try_into()?;

    scribe = scribe
        .line()
        .write(format!("let {} = state.get::<", var_name).as_str())
        .write_ext(&type_name, &type_src)
        .write(format!(">(\"{}\").await?;", var_name).as_str());

    Ok(scribe)
}

fn author_write_state(mut scribe: Scribe, params: &Value) -> Result<Scribe, ExecError> {
    let var_name: String = params.lookup("name")?.try_into()?;

    scribe = scribe
        .line()
        .write(format!("state.set(\"{}\", {}).await?;", var_name, var_name).as_str());

    Ok(scribe)
}

fn author_effect_step(scribe: Scribe, step: &Value) -> Result<Scribe, ExecError> {
    let which: String = step.index(0)?.try_into()?;
    let params = &(step.index(1)?);

    match which.as_str() {
        "read_state" => author_read_state(scribe, params),
        "write_state" => author_write_state(scribe, params),
        _ => Err(ExecError::Io("invalid state step type".into()))
    }
}

pub(super) fn author_effect(mut scribe: Scribe, name: String, value: Value) -> Result<Scribe, ExecError> {
    scribe = scribe
        .write("#[apply(")
        .write_ext("effect_fn", "progenitor")
        .write(")]")
        .line()
        .write(format!("pub fn {}_effect<'ef>(state: &'ef ", name).as_str())
        .write_ext("State", "progenitor")
        .write(") -> Result<(), ")
        .write_ext("EffectError", "progenitor")
        .write("> {")
        .tab_in();

    let steps_lookup = value.lookup("steps")?;
    let steps = steps_lookup.elements()?;

    for step in steps {
        scribe = author_effect_step(scribe, step)?;
    }

    Ok(scribe.tab_out().line().write("}"))
}
