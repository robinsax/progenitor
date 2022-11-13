use progenitor::Value;

use super::super::errors::ExecError;
use super::scribe::Scribe;
use super::value::author_value;

pub(super) fn author_effect(mut scribe: Scribe, name: String, value: Value) -> Result<Scribe, ExecError> {
    let base_src: String = value.lookup("from")?.index(0)?.try_into()?;
    let base_name: String = value.lookup("from")?.index(1)?.try_into()?;

    scribe = scribe
        .write_ext("archetype_effect", "progenitor")
        .write("!(")
        .tab_in().line()
        .write(format!("{}, \"", name).as_str())
        .write_ext(base_name.as_str(), base_src.as_str())
        .write("\",");
    
    scribe = author_value(scribe, value.lookup("params")?)?
        .tab_out().line()
        .write(");");

    Ok(scribe)
}
