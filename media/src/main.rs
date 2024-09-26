#[derive(Debug)]
enum Media {
    Book {
        title: String,
        author: String,
    },
    Movie {
        title: String,
        director: String,
    },
    Audiobook {
        title: String
    },
    Podcast(u32),
    Placeholder
}

impl Media {
    fn print(&self) {
        println!("{:#?}", self);
    }

    fn description(&self) -> String {
        if let Self::Audiobook { title } = self {
            return title.clone();
        }

        if let Self::Book { title, author: _ } = self {
            return title.clone();
        }

        if let Self::Movie { title, director: _ } = self {
            return title.clone();
        }

        "".to_string()
    }

    fn description2(&self) -> String {
        match self {
            Self::Audiobook { title } => title.clone(),

            Self::Book { title, author: _ } => {
                title.clone()
            }

            Self::Movie { title, director: _ } => {
                title.clone()
            }

            Self::Podcast(number) => number.to_string(),
            Self::Placeholder => "".to_string()
        }
    }
}


#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }
}


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

    println!("{:#?}", catalog);
}
