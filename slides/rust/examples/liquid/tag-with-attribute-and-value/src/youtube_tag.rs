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
pub struct YouTubeTag;

impl TagReflection for YouTubeTag {
    fn tag(&self) -> &'static str {
        "youtube"
    }

    fn description(&self) -> &'static str {
        ""
    }
}

impl ParseTag for YouTubeTag {
    fn parse(
        &self,
        mut arguments: TagTokenIter<'_>,
        _options: &Language,
    ) -> Result<Box<dyn Renderable>> {
        let field = arguments
            .expect_next("Identifier expected.")?
            .expect_identifier()
            .into_result()?
            .to_string();
        if field != "id" {
            return Err(liquid_core::error::Error::with_msg("Expected id"));
        }

        arguments
            .expect_next("Assignment operator \"=\" expected.")?
            .expect_str("=")
            .into_result_custom_msg("Assignment operator \"=\" expected.")?;

        let token = arguments.expect_next("Identifier or value expected")?;
        let value = match token.expect_literal() {
            TryMatchToken::Matches(name) => name.to_kstr().into_string(),
            TryMatchToken::Fails(name) => return name.raise_error().into_err(),
        };

        let id = value;

        // no more arguments should be supplied, trying to supply them is an error
        arguments.expect_nothing()?;

        Ok(Box::new(YouTube { id }))
    }

    fn reflection(&self) -> &dyn TagReflection {
        self
    }
}

#[derive(Debug)]
struct YouTube {
    id: String,
}

impl Renderable for YouTube {
    fn render_to(&self, writer: &mut dyn Write, _runtime: &dyn Runtime) -> Result<()> {
        write!(
            writer,
            r#"<a href="https://youtube.com/watch?v={}">video</a>"#,
            self.id
        )
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
            .register("youtube".to_string(), YouTubeTag.into());
        options
    }

    #[test]
    fn youtube() {
        let options = options();
        let template = parser::parse(r#"{% youtube id = "R2_D2" %}"#, &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(
            output,
            r#"<a href="https://youtube.com/watch?v=R2_D2">video</a>"#
        );
    }
}
