use image::{io::Reader, DynamicImage};

pub struct Image {
    pub data: DynamicImage,
}

impl Image {
    pub fn new(path: &str) -> Image {
        let img = Reader::open(path).unwrap().decode().unwrap().flipv();

        Image { data: img }
    }
}
