use std::io::Write;

use liquid_core::error::ResultLiquidReplaceExt;
use liquid_core::parser::TryMatchToken;
use liquid_core::Language;
use liquid_core::Renderable;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::ValueView;
use liquid_core::{ParseTag, TagReflection, TagTokenIter};

#[derive(Copy, Clone, Debug, Default)]
pub struct AddNumbersTag;

impl TagReflection for AddNumbersTag {
    fn tag(&self) -> &'static str {
        "add"
    }

    fn description(&self) -> &'static str {
        ""
    }
}

impl ParseTag for AddNumbersTag {
    fn parse(
        &self,
        mut arguments: TagTokenIter<'_>,
        _options: &Language,
    ) -> Result<Box<dyn Renderable>> {
        let literal = arguments
            .expect_next("Identifier expected.")?
            .expect_literal();

        let a = match literal {
            TryMatchToken::Matches(name) => name.to_kstr().into_string().parse::<i32>().unwrap(),
            TryMatchToken::Fails(name) => return name.raise_error().into_err(),
        };

        let literal = arguments
            .expect_next("Identifier expected.")?
            .expect_literal();

        let b = match literal {
            TryMatchToken::Matches(name) => name.to_kstr().into_string().parse::<i32>().unwrap(),
            TryMatchToken::Fails(name) => return name.raise_error().into_err(),
        };

        // no more arguments should be supplied, trying to supply them is an error
        arguments.expect_nothing()?;

        Ok(Box::new(AddNumbers { a, b }))
    }

    fn reflection(&self) -> &dyn TagReflection {
        self
    }
}

#[derive(Debug)]
struct AddNumbers {
    a: i32,
    b: i32,
}

impl Renderable for AddNumbers {
    fn render_to(&self, writer: &mut dyn Write, _runtime: &dyn Runtime) -> Result<()> {
        write!(writer, "{} + {} = {}", self.a, self.b, self.a + self.b)
            .replace("Failed to render")?;
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
            .register("add".to_string(), AddNumbersTag.into());
        options
    }

    #[test]
    fn add_numbers() {
        let options = options();
        let template = parser::parse("{% add   19   23 %}", &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(output, "19 + 23 = 42".to_string());
    }

    #[test]
    fn add_negative_numbers() {
        let options = options();
        let template = parser::parse("{% add   -19   -23 %}", &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(output, "-19 + -23 = -42".to_string());
    }
}
