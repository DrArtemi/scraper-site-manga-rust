#[derive(Debug)]
pub struct Comic {
    pub title: String,
    pub url: String,
    pub cover_url: String
}

impl Comic {
    pub fn new(title: String, url: String, cover_url: String) -> Comic {
        Comic {
            title,
            url,
            cover_url
        }
    }
}

#[derive(Debug)]
pub struct Chapter {
    pub url: String,
    pub number: String,
    pub date: String
}

impl Chapter {
    pub fn new(url: String, number: String, date: String) -> Chapter {
        Chapter {
            url,
            number,
            date
        }
    }
}