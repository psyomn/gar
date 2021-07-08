use gar::models::languages::Language;

#[test]
fn match_language_test() {
    assert_eq!(Language::C, "C".parse::<Language>().unwrap());
    assert_eq!(Language::CC, "C++".parse::<Language>().unwrap());
    assert_eq!(Language::CC, "cXx".parse::<Language>().unwrap());
    assert_eq!(Language::CC, "cPP".parse::<Language>().unwrap());
    assert_eq!(Language::Forth, "Forth".parse::<Language>().unwrap());

    assert_eq!(Language::Other("blargotron".into()),
               "blargotron".parse::<Language>().unwrap());
}
