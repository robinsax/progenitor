use progenitor::Value;

use super::super::errors::ExecError;
use super::scribe::Scribe;

fn author_value_elem(mut scribe: Scribe, value: &Value) -> Scribe {
    match value {
        Value::Map(members) => {
            scribe = scribe
                .line()
                .write_ext("Value", "progenitor")
                .write("::map_from([")
                .tab_in();

            for (key, value_type) in members.iter() {
                scribe = scribe
                    .write(",").line().write("(")
                    .tab_in().line()
                    .write(format!("\"{}\".into(),", key).as_str());

                scribe = author_value_elem(scribe, value_type)
                    .tab_out().line()
                    .write(")");
            }

            scribe.tab_out().line().write("])")
        },
        Value::List(members) => {
            scribe = scribe
                .line()
                .write_ext("Value", "progenitor")
                .write("::List(")
                .write_ext("Vec", "std::vec")
                .write("::from([")
                .tab_in();
            
            for inner in members.iter() {
                scribe = author_value_elem(scribe, inner)
            }

            scribe.tab_out().line().write("]))")
        },
        Value::Str(inner) => scribe.line().write(format!("Value::str_from(\"{}\")", inner).as_str()),
        Value::Int32(inner) => scribe.line().write(format!("Value::Int32({})", inner).as_str()),
        Value::Uint32(inner) => scribe.line().write(format!("Value::Uint32({})", inner).as_str()),
        Value::Float64(inner) => scribe.line().write(format!("Value::Float64({})", inner).as_str()),
        Value::Bool(inner) => scribe.line().write(format!("Value::Bool({})", inner).as_str()),
        Value::Null => scribe.line().write("Value::Null")
    }
}

pub(super) fn author_value(
    scribe: Scribe, value: Value
) -> Result<Scribe, ExecError> {    
    Ok(author_value_elem(scribe, &value))
}
