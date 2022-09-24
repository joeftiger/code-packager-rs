use code_packager::{TagError, Tags};

static CONTENT: &str = include_str!("res/simple_content.rs");
static IMPLEMENT: &str = include_str!("res/simple_implement.rs");
static PROVIDE: &str = include_str!("res/simple_provide.rs");

#[test]
fn test_implement() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("implement")
        .add_exclude("provide")
        .end_tag("end");

    let trimmed = tag.package(CONTENT).unwrap();
    assert_eq!(IMPLEMENT, &trimmed);
}

#[test]
fn test_provide() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_exclude("implement")
        .add_include("provide")
        .end_tag("end");

    let trimmed = tag.package(CONTENT).unwrap();
    assert_eq!(PROVIDE, &trimmed);
}

#[test]
fn test_error() {
    let tag = Tags::new()
        .tag_prefix("// @")
        .add_include("implement")
        .end_tag("end");

    assert_eq!(Err(TagError::UnexpectedEndTag(8)), tag.package(CONTENT));
}
