fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("python"),
        String::from("javascript"),
    ];
    let result = next_language(&languages, "python");
    println!("{:#?}", result);
}

fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true
        }
    }

    languages.last().unwrap()
}
