pub mod catalog;
mod media;

#[derive(Debug)]
pub enum Media {
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
    Placeholder,
}

impl Media {
    pub fn print(&self) {
        println!("{:#?}", self);
    }

    pub fn description(&self) -> String {
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

    pub fn description2(&self) -> String {
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