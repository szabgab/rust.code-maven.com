use std::io::Write;

use liquid_core::error::ResultLiquidReplaceExt;
use liquid_core::model::Scalar;
use liquid_core::Language;
use liquid_core::Renderable;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::ValueView;
use liquid_core::{ParseTag, TagReflection, TagTokenIter};

#[derive(Copy, Clone, Debug, Default)]
pub struct ShowTag;

impl TagReflection for ShowTag {
    fn tag(&self) -> &'static str {
        "show"
    }

    fn description(&self) -> &'static str {
        ""
    }
}

impl ParseTag for ShowTag {
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

        arguments.expect_nothing()?;

        Ok(Box::new(Show { field }))
    }

    fn reflection(&self) -> &dyn TagReflection {
        self
    }
}

#[derive(Debug)]
struct Show {
    field: String,
}

impl Renderable for Show {
    fn render_to(&self, writer: &mut dyn Write, runtime: &dyn Runtime) -> Result<()> {
        let field = self.field.clone();
        let value = match runtime.get(&[Scalar::new(field)]) {
            Ok(value) => value,
            Err(_) => {
                return Err(liquid_core::error::Error::with_msg(format!(
                    "No value called '{}' was passed to the render function.",
                    self.field
                )))
            }
        };

        write!(writer, "{}", value.to_kstr()).replace("Failed to render")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use liquid_core::parser;
    use liquid_core::runtime;
    use liquid_core::runtime::RuntimeBuilder;
    use liquid_core::Value;

    fn options() -> Language {
        let mut options = Language::default();
        options.tags.register("show".to_string(), ShowTag.into());
        options
    }

    #[test]
    fn one() {
        let options = options();
        let template = parser::parse(r#"{% show name %}"#, &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();
        runtime.set_global("name".into(), Value::scalar("John Snow"));

        let output = template.render(&runtime).unwrap();
        assert_eq!(output, r#"John Snow"#);
    }

    #[test]
    fn three() {
        let options = options();
        let template = parser::parse(
            r#"{% show name %} - {% show color %} - {% show answer %}"#,
            &options,
        )
        .map(runtime::Template::new)
        .unwrap();

        let runtime = RuntimeBuilder::new().build();
        runtime.set_global("name".into(), Value::scalar("John Snow"));
        runtime.set_global("color".into(), Value::scalar("blue"));
        runtime.set_global("answer".into(), Value::scalar(42));

        let output = template.render(&runtime).unwrap();
        assert_eq!(output, "John Snow - blue - 42");
    }
}
