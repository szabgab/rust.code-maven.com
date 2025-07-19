mod youtube_tag;

fn main() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(youtube_tag::YouTubeTag)
        .build()
        .unwrap()
        .parse(r#"Video: {% youtube id="hello" %}"#)
        .unwrap();

    let globals = liquid::object!({});

    let output = template.render(&globals).unwrap();
    assert_eq!(
        output,
        r#"Video: <a href="https://youtube.com/watch?v=hello">video</a>"#
    );
}
