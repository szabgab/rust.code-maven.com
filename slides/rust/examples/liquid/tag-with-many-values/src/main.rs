mod youtube_tag;

fn main() {
    one();
    two();
}

fn one() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(youtube_tag::YouTubeTag)
        .build()
        .unwrap()
        .parse("Video: {% youtube K6EvVvYnjrY %}")
        .unwrap();

    let globals = liquid::object!({});

    let output = template.render(&globals).unwrap();
    assert_eq!(
        output,
        r#"Video: <a href="https://www.youtube.com/watch?v=K6EvVvYnjrY">video</a>"#.to_string()
    );
}

fn two() {
    let template = liquid::ParserBuilder::with_stdlib()
        .tag(youtube_tag::YouTubeTag)
        .build()
        .unwrap()
        .parse("Video: {% youtube   R2_D2    K6EvVvYnjrY %}")
        .unwrap();

    let globals = liquid::object!({});

    let output = template.render(&globals).unwrap();
    assert_eq!(
        output,
        r#"Video: <a href="https://www.youtube.com/watch?v=R2_D2">video</a><a href="https://www.youtube.com/watch?v=K6EvVvYnjrY">video</a>"#.to_string()
    );
}
