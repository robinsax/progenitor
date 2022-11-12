// TODO: Could use Syn although it won't be necessary any time soon.
use std::collections::HashMap;

use bytes::Bytes;

pub(super) struct Scribe {
    builder: String,
    indent: usize,
    ext_types: HashMap<String, String>
}

// TODO: If going this route long term, make it actually abstract the syntax.
impl Scribe {
    pub fn new(capacity: usize) -> Self {
        Self {
            builder: String::with_capacity(capacity),
            indent: 0,
            ext_types: HashMap::new()
        }
    }

    pub fn write(mut self, string: &str) -> Self {
        self.builder.push_str(string);

        self
    }

    pub fn write_ext(mut self, name: &str, import_path: &str) -> Self {
        self.ext_types.insert(name.to_owned(), import_path.to_owned());

        self.write(name)
    }

    pub fn line(self) -> Self {
        let indent = self.indent;

        self.write(format!("\n{}", "    ".repeat(indent)).as_str())
    }

    pub fn tab_in(mut self) -> Self {
        self.indent += 1;

        self
    }

    pub fn tab_out(mut self) -> Self {
        self.indent -= 1;

        self
    }

    pub fn start_fn(self, name: &str, rv_t: &str) -> Self {
        self
            .write(format!("fn {}() -> {} {{", name, rv_t).as_str())
            .tab_in()
    }

    pub fn end_fn(self) -> Self {
        self
            .tab_out()
            .line()
            .write("}")
    }

    pub fn into_result(self, as_module: bool) -> Bytes {
        let mut result = self.builder;

        // TODO: Dumb!
        if as_module {
            let mut preface_scribe = Self::new(256);
            
            for (type_name, import_src) in self.ext_types {
                preface_scribe = preface_scribe
                    .write(format!("use {}::{};", import_src, type_name).as_str())
                    .line();
            }

            result = format!("{}{}", preface_scribe.line().builder, result);
        }

        Bytes::from(result)
    }
}
