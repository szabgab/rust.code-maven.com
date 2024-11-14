use std::io::Write;

use liquid_core::error::ResultLiquidReplaceExt;
use liquid_core::model::Scalar;
use liquid_core::parser::TryMatchToken;
use liquid_core::Language;
use liquid_core::Renderable;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::ValueView;
use liquid_core::{ParseTag, TagReflection, TagTokenIter};
use serde::Serialize;

#[derive(Copy, Clone, Debug, Default)]
pub struct LatestTag;

impl TagReflection for LatestTag {
    fn tag(&self) -> &'static str {
        "latest"
    }

    fn description(&self) -> &'static str {
        ""
    }
}

impl ParseTag for LatestTag {
    fn parse(
        &self,
        mut arguments: TagTokenIter<'_>,
        _options: &Language,
    ) -> Result<Box<dyn Renderable>> {
        arguments
            .expect_next("limit expected")?
            .expect_str("limit")
            .into_result_custom_msg("limit expected.")?;

        arguments
            .expect_next("Assignment operator \"=\" expected.")?
            .expect_str("=")
            .into_result_custom_msg("Assignment operator \"=\" expected.")?;

        let token = arguments.expect_next("Identifier or value expected")?;
        let value = match token.expect_literal() {
            TryMatchToken::Matches(name) => name.to_kstr().to_string(),
            TryMatchToken::Fails(name) => return name.raise_error().into_err(),
        };
        let limit = match value.parse::<u8>() {
            Ok(value) => value,
            Err(_) => return Err(liquid_core::error::Error::with_msg("Expected number")),
        };

        let key = arguments.next();

        let tag = match key {
            Some(token) => {
                token
                    .expect_str("tag")
                    .into_result_custom_msg("expected tag")?;
                arguments
                    .expect_next("Assignment operator \"=\" expected.")?
                    .expect_str("=")
                    .into_result_custom_msg("Assignment operator \"=\" expected.")?;

                let literal = arguments.expect_next("value of tag")?.expect_literal();

                let value = match literal {
                    TryMatchToken::Matches(name) => name.to_kstr().to_string(),
                    TryMatchToken::Fails(name) => return name.raise_error().into_err(),
                };
                Some(value)
            }
            None => None,
        };

        arguments.expect_nothing()?;

        Ok(Box::new(Latest { limit, tag }))
    }

    fn reflection(&self) -> &dyn TagReflection {
        self
    }
}

#[derive(Debug)]
struct Latest {
    limit: u8,
    tag: Option<String>,
}

impl Renderable for Latest {
    fn render_to(&self, writer: &mut dyn Write, runtime: &dyn Runtime) -> Result<()> {
        let mut count = 0;

        let selected_tag = self.tag.clone().unwrap_or_default();

        match runtime.get(&[Scalar::new("items")]) {
            // Ok(values) => values.as_array().unwrap().values().collect::<Vec<_>>(),
            Ok(values) => {
                for val in values.as_array().unwrap().values() {
                    let obj = val.as_object().unwrap();
                    let text = obj.get("text").unwrap().to_kstr().to_string();
                    let tag = obj.get("tag").unwrap().to_kstr().to_string();
                    if self.tag.is_some() && tag != selected_tag {
                        continue;
                    }

                    write!(writer, "<li>{} ({})</li>", text, tag).replace("Failed to render")?;
                    count += 1;
                    if count >= self.limit {
                        break;
                    }
                }
            }
            Err(_) => return Err(liquid_core::error::Error::with_msg("Expected number")),
        };

        //println!("{:?}", things[0]);
        //println!("text: {:?}", things[0].as_object().unwrap().get("text"));
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Item<'a> {
    text: &'a str,
    id: u32,
    tag: &'a str,
}

impl<'a> Item<'a> {
    pub fn new(text: &'a str, id: u32, tag: &'a str) -> Self {
        Self { text, id, tag }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_items() -> &'static [Item<'static>] {
        let items = &[
            Item {
                text: "one",
                id: 1,
                tag: "web",
            },
            Item {
                text: "two",
                id: 2,
                tag: "programming",
            },
            Item {
                text: "three",
                id: 3,
                tag: "web",
            },
            Item {
                text: "four",
                id: 4,
                tag: "programming",
            },
            Item {
                text: "five",
                id: 5,
                tag: "web",
            },
            Item {
                text: "six",
                id: 6,
                tag: "programming",
            },
            Item {
                text: "seven",
                id: 7,
                tag: "web",
            },
            Item {
                text: "eight",
                id: 8,
                tag: "programming",
            },
            Item {
                text: "nine",
                id: 9,
                tag: "web",
            },
            Item {
                text: "ten",
                id: 10,
                tag: "web",
            },
        ];
        items
    }

    use liquid_core::object;
    use liquid_core::parser;
    use liquid_core::runtime;
    use liquid_core::runtime::RuntimeBuilder;
    use liquid_core::Value;

    fn options() -> Language {
        let mut options = Language::default();
        options
            .tags
            .register("latest".to_string(), LatestTag.into());
        options
    }

    #[test]
    fn latest_5() {
        let options = options();
        let template = parser::parse(r#"{% latest limit=5 %}"#, &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let objects = get_items()
            .iter()
            .map(|item| {
                let obj = object!({"text": item.text, "id": item.id, "tag": item.tag});
                Value::Object(obj)
            })
            .collect::<Vec<_>>();

        runtime.set_global("items".into(), Value::Array(objects));

        let output = template.render(&runtime).unwrap();
        assert_eq!(
            output,
            r#"<li>one (web)</li><li>two (programming)</li><li>three (web)</li><li>four (programming)</li><li>five (web)</li>"#
        );
    }

    #[test]
    fn latest_5_web() {
        let options = options();
        let template = parser::parse(r#"{% latest limit=5   tag="web" %}"#, &options)
            .map(runtime::Template::new)
            .unwrap();

        let runtime = RuntimeBuilder::new().build();

        let objects = get_items()
            .iter()
            .map(|item| {
                let obj = object!({"text": item.text, "id": item.id, "tag": item.tag});
                Value::Object(obj)
            })
            .collect::<Vec<_>>();

        runtime.set_global("items".into(), Value::Array(objects));

        let output = template.render(&runtime).unwrap();
        assert_eq!(
            output,
            r#"<li>one (web)</li><li>three (web)</li><li>five (web)</li><li>seven (web)</li><li>nine (web)</li>"#
        );
    }
}
