fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let mut colors2 = colors.clone();

    let mut color_iter = colors.iter();

    println!("{:#?}", color_iter.next());
    println!("{:#?}", color_iter.next());
    println!("{:#?}", color_iter.next());
    println!("{:#?}", color_iter.next());
    print_elements(&colors[1..3]);
    shorten_strings(&mut colors[1..3]);
    to_uppercase(&mut colors[1..3]);

    let mut destination = vec![];
    move_elements(colors, &mut destination);
    println!("move_elements: {:#?}", destination);

    println!("explode: {:#?}", explode(&colors2[1..3]));

    println!("colors2: {:#?}", colors2);
    println!("red: {:#?}", find_color_or(&colors2, "red", "orange"));
    println!("black: {:#?}", find_color_or(&colors2, "black", "orange"));
}

fn print_elements(elements: &[String]) {
    println!();

    for element in elements {
        println!("{}", element);
    }

    println!();

    elements.iter().for_each(|element| {
        println!("{}", element);
    });

    println!();

    elements.iter().map(|e| e.to_uppercase())
        .for_each(|e| {
            println!("{}", e);
        });

    println!();
}

fn shorten_strings(strings: &mut [String]) {
    strings.iter_mut()
        .for_each(|e| {
            e.truncate(1);
            println!("{}", e);
        });
}

fn to_uppercase(string: &mut [String])  {
    string.iter().map(|e| e.to_uppercase()).for_each(|s| {
        println!("Upper: {}", s);
    });
}

fn move_elements(v1: Vec<String>, v2: &mut Vec<String>) {
    v1.into_iter().for_each(|e| {
        v2.push(e);
    })
}

fn explode(elements: &[String])  -> Vec<Vec<String>> {
    elements.iter()
        .map(|e| e.chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}

fn find_color_or(elements: &[String], search: &str, default: &str) -> String {
    elements.iter()
        .find(|e| e.contains(search))
        .map_or(
            String::from(default),
            |e| e.to_string()
        )
}