use std::io::Write;

use liquid_core::error::Error;
use liquid_core::error::ResultLiquidReplaceExt;
use liquid_core::Language;
use liquid_core::Renderable;
use liquid_core::Result;
use liquid_core::Runtime;
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
        let mut ids = vec![];
        loop {
            let item = arguments.next();
            match item {
                None => break,
                Some(item) => {
                    let id = item.expect_identifier().into_result()?.to_string();

                    ids.push(id);
                }
            }
        }

        if ids.is_empty() {
            return Err(Error::with_msg("No video id provided"));
        }

        arguments.expect_nothing()?;

        Ok(Box::new(YouTube { ids }))
    }

    fn reflection(&self) -> &dyn TagReflection {
        self
    }
}

#[derive(Debug)]
struct YouTube {
    ids: Vec<String>,
}

impl Renderable for YouTube {
    fn render_to(&self, writer: &mut dyn Write, _runtime: &dyn Runtime) -> Result<()> {
        for id in &self.ids {
            write!(
                writer,
                r#"<a href="https://www.youtube.com/watch?v={}">video</a>"#,
                id
            )
            .replace("Failed to render")?;
        }
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
        let template = parser::parse("{% youtube   K6EvVvYnjrY %}", &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(
            output,
            r#"<a href="https://www.youtube.com/watch?v=K6EvVvYnjrY">video</a>"#
        );
    }

    #[test]
    fn youtube_2() {
        let options = options();
        let template = parser::parse("{% youtube   K6EvVvYnjrY   R2_D2 %}", &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(
            output,
            r#"<a href="https://www.youtube.com/watch?v=K6EvVvYnjrY">video</a><a href="https://www.youtube.com/watch?v=R2_D2">video</a>"#
        );
    }

    #[test]
    fn youtube_3() {
        let options = options();
        let template = parser::parse("{% youtube  qqrq K6EvVvYnjrY   R2_D2 %}", &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let output = template.render(&runtime).unwrap();
        assert_eq!(
            output,
            r#"<a href="https://www.youtube.com/watch?v=qqrq">video</a><a href="https://www.youtube.com/watch?v=K6EvVvYnjrY">video</a><a href="https://www.youtube.com/watch?v=R2_D2">video</a>"#
        );
    }

    #[test]
    fn youtube_missing() {
        let options = options();
        let err = parser::parse("{% youtube %}", &options)
            .map(runtime::Template::new)
            .err()
            .unwrap();
        let err = err.to_string();
        assert_eq!(err, "liquid: No video id provided\n");
    }
}
