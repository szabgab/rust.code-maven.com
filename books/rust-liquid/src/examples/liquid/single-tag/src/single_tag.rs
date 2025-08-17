use std::io::Write;

use liquid_core::error::ResultLiquidReplaceExt;
use liquid_core::Language;
use liquid_core::Renderable;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{ParseTag, TagReflection, TagTokenIter};

#[derive(Copy, Clone, Debug, Default)]
pub struct SingleTag;

impl TagReflection for SingleTag {
    fn tag(&self) -> &'static str {
        "single"
    }

    fn description(&self) -> &'static str {
        ""
    }
}

impl ParseTag for SingleTag {
    fn parse(
        &self,
        mut arguments: TagTokenIter<'_>,
        _options: &Language,
    ) -> Result<Box<dyn Renderable>> {
        arguments.expect_nothing()?;
        Ok(Box::new(Single))
    }

    fn reflection(&self) -> &dyn TagReflection {
        self
    }
}

#[derive(Debug)]
struct Single;

impl Renderable for Single {
    fn render_to(&self, writer: &mut dyn Write, _runtime: &dyn Runtime) -> Result<()> {
        write!(writer, "Single replaced by this string.").replace("Failed to render")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use liquid_core::parser;
    use liquid_core::runtime;
    use liquid_core::runtime::RuntimeBuilder;

    fn options() -> Language {
        let mut options = Language::default();
        options
            .tags
            .register("single".to_string(), SingleTag.into());
        options
    }

    #[test]
    fn simple() {
        let options = options();
        let template = parser::parse("{% single %}", &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(output, "Single replaced by this string.");
    }
}
