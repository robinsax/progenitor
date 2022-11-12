use progenitor::{Type, Value};

use super::super::errors::ExecError;
use super::scribe::Scribe;

// TODO: Dumb setup with the rv.
fn author_schema_elem(mut scribe: Scribe, typ: &Type) -> Scribe {
    match typ {
        Type::Map(members) => {
            scribe = scribe
                .line()
                .write_ext("Type", "progenitor")
                .write("::Map(")
                .write_ext("HashMap", "std::collections")
                .write("::from([")
                .tab_in();

            for (key, value_type) in members.iter() {
                scribe = scribe
                    .line().write("(")
                    .tab_in().line()
                    .write(format!("\"{}\".into(),", key).as_str());

                scribe = author_schema_elem(scribe, value_type)
                    .tab_out().line()
                    .write("),");
            }

            scribe.tab_out().line().write("]))")
        },
        Type::List(inner) => {
            scribe = scribe
                .line()
                .write_ext("Type", "progenitor")
                .write("::List(")
                .write_ext("Box", "std::boxed")
                .write("::new(")
                .tab_in();
            
            author_schema_elem(scribe, inner.as_ref())
                .tab_out().line()
                .write("))")
        },
        Type::String => scribe.line().write("Type::String"),
        Type::Int32 => scribe.line().write("Type::Int32"),
        Type::Uint32 => scribe.line().write("Type::Uint32"),
        Type::Float64 => scribe.line().write("Type::Float64"),
        Type::Bool => scribe.line().write("Type::Bool")
    }
}

pub(super) fn author_schema_fn(
    mut scribe: Scribe, name: String, schema_ex: Value
) -> Result<Scribe, ExecError> {
    let schema: Type = schema_ex.try_into()?;

    scribe = scribe.write("pub ").start_fn(format!("{}_type", name).as_str(), "Type");
    
    Ok(author_schema_elem(scribe, &schema).end_fn())
}
