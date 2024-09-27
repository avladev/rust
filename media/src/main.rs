mod content;
use content::Catalog;
use content::Media;

fn main() {
    let mut catalog = Catalog::new();
    let book = Media::Book {
        title: String::from("1987"),
        author: String::from("Orwell"),
    };

    let audiobook = Media::Audiobook {
        title: String::from("Life of Boris"),
    };

    let movie = Media::Movie {
        title: String::from("The Machinist"),
        director: String::from("Tarantino"),
    };

    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    book.print();
    audiobook.print();
    movie.print();
    podcast.print();
    placeholder.print();

    println!("{:#?}", book.description());
    println!("{:#?}", book.description2());
    println!("{:#?}", audiobook.description());
    println!("{:#?}", audiobook.description2());
    println!("{:#?}", movie.description());
    println!("{:#?}", movie.description2());
    println!("{:#?}", podcast.description());
    println!("{:#?}", podcast.description2());
    println!("{:#?}", placeholder.description());
    println!("{:#?}", placeholder.description2());

    catalog.add(book);
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);
    let index = 1;

    match catalog.items.get(index) {
        Some(item) => {
            println!("{:#?}", item.description());
        }

        None => {
            println!("There is no item at that index");
        }
    }

    let item = catalog.get_by_index(2);

    match item {
        catalog::MightHave::Value(media) => {
            println!("{:#?} here2", media.description());
        }

        catalog::MightHave::None => {
            println!("There is no item at that index");
        }
    }

    let item = catalog.get_by_index2(1);

    match item {
        Some(item) => {
            println!("{:#?} here", item.description());
        }

        None => {
            println!("There is no item at that index");
        }
    }

    println!("{:#?} zzz", catalog.items.get(1).unwrap_or(&Media::Placeholder));
}
