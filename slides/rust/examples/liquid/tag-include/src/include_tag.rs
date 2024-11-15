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
pub struct IncludeTag;

impl TagReflection for IncludeTag {
    fn tag(&self) -> &'static str {
        "include"
    }

    fn description(&self) -> &'static str {
        ""
    }
}

impl ParseTag for IncludeTag {
    fn parse(
        &self,
        mut arguments: TagTokenIter<'_>,
        _options: &Language,
    ) -> Result<Box<dyn Renderable>> {
        arguments
            .expect_next("\"file\" expected.")?
            .expect_str("file")
            .into_result_custom_msg("\"file\" expected.")?;

        arguments
            .expect_next("Assignment operator \"=\" expected.")?
            .expect_str("=")
            .into_result_custom_msg("Assignment operator \"=\" expected.")?;

        let token = arguments.expect_next("Identifier or value expected")?;
        let file = match token.expect_literal() {
            TryMatchToken::Matches(name) => name.to_kstr().into_string(),
            TryMatchToken::Fails(name) => return name.raise_error().into_err(),
        };

        arguments.expect_nothing()?;

        Ok(Box::new(Include { file }))
    }

    fn reflection(&self) -> &dyn TagReflection {
        self
    }
}

#[derive(Debug)]
struct Include {
    file: String,
}

impl Renderable for Include {
    fn render_to(&self, writer: &mut dyn Write, _runtime: &dyn Runtime) -> Result<()> {
        let content = std::fs::read_to_string(&self.file).replace("Failed to render")?;
        write!(writer, r#"{content}"#).replace("Failed to render")?;
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
            .register("include".to_string(), IncludeTag.into());
        options
    }

    #[test]
    fn include_file() {
        let options = options();
        let template = parser::parse(r#"{% include file = "files/hello.txt" %}"#, &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(output, "This is the hello file.\n");
    }
}
