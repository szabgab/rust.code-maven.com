fn main() {
    let good_addresses = vec![
        "foo@bar.com",
        "Foo@Bar.Com",
        "foo.bar@bar.com",
        "foo+bar@bar.com",
        "foo-bar@bar.com",
        "foo@foo-bar.com",
        "foo_bar@bar.com",
        "foo@bar.nosuchname",
        "foo{bar}@bar.com",
        "foo?@bar.com",
        "|foo@bar.com",
        "/foo@bar.com",
        "foo@bar",      // Why is this accepted as valid?
    ];

    for address in good_addresses {
        assert!(validator::validate_email(address));
    }

    let bad_addresses = vec![
        "  foo@bar.com",
        "foobar.com",
        "foo bar@bar.com",
        "foo@bar|.com",
        "foo@foo_bar.com",
    ];

    for address in bad_addresses {
        assert!(!validator::validate_email(address));
    }
}
