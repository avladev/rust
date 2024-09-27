use crate::content::Media;

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    pub fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_by_index(&self, index: usize) -> MightHave {
        if self.items.len() > index {
            MightHave::Value(&self.items[index])
        } else {
            MightHave::None
        }
    }

    pub fn get_by_index2(&self, index: usize) -> Option<&Media> {
        self.items.get(index)
    }
}

pub enum MightHave<'a> {
    Value(&'a Media),
    None
}