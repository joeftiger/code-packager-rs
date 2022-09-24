use code_packager::{TagError, Tags};

static CONTENT: &str = include_str!("res/nested_content.rs");

static OUTER: &str = include_str!("res/nested_outer.rs");

static INNER: &str = include_str!("res/nested_inner.rs");

static INNERMOST: &str = include_str!("res/nested_innermost.rs");

#[test]
fn test_outer() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("outer")
        .add_exclude("inner")
        .add_exclude("innermost")
        .end_tag("end");

    let trimmed = tag.package(CONTENT).unwrap();
    assert_eq!(OUTER, &trimmed);
}

#[test]
fn test_outer2() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("outer")
        .add_exclude("inner")
        .add_include("innermost")
        .end_tag("end");

    let trimmed = tag.package(CONTENT).unwrap();
    assert_eq!(OUTER, &trimmed);
}

#[test]
fn test_outer_error() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("outer")
        .add_exclude("inner")
        .end_tag("end");

    assert_eq!(Err(TagError::UnexpectedEndTag(22)), tag.package(CONTENT));
}

#[test]
fn test_inner() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("outer")
        .add_include("inner")
        .add_exclude("innermost")
        .end_tag("end");

    let trimmed = tag.package(CONTENT).unwrap();
    assert_eq!(INNER, &trimmed);
}

#[test]
fn test_inner_error2() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("outer")
        .add_include("inner")
        .end_tag("end");

    assert_eq!(Err(TagError::UnexpectedEndTag(22)), tag.package(CONTENT));
}

#[test]
fn test_innermost() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("outer")
        .add_include("inner")
        .add_include("innermost")
        .end_tag("end");

    let trimmed = tag.package(CONTENT).unwrap();
    assert_eq!(INNERMOST, &trimmed);
}
