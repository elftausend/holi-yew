#[test]
fn test_tag_cleaner() {
    let mut tags = "Test, Tag A, B, C U".to_string();
    tags = tags.replace(',', "");

    assert_eq!(tags, "Test Tag A B C U")
}