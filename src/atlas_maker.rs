use image::io::Reader as ImageReader;
use image::{GenericImageView};

struct Atlas {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Atlas {
    fn append_image_buffer(&mut self, buffer: Vec<u8>, size: usize) {
        // NOTE: buffer.len() = width * height * color_type;
        // currently we take only squared images, so image_size is both width and height
        let buffer_line_len = size * 4; // rgba takes 4 bytes
        let atlas_line_len = self.width * 4;

        // NOTE: rust range [a..b] is inclusive on the lower bound and exclusive on the upper
        for i in 0..size {
            let s = i * buffer_line_len;
            let e = s + buffer_line_len;
            let original_buffer_line_end = (i + 1) * atlas_line_len; // index of the end of each line of the original buffer
            let bytes_added_len = i * buffer_line_len; // bytes that were added to the buffer so far
            let line_end = original_buffer_line_end + bytes_added_len;

            self.buffer.splice(line_end..line_end, buffer[s..e].to_vec());
        }

        self.width += size;
        // self.height doesn't change yet. will be, once the vertical append will be supported
    }
}

// TODO: fix return type, remove unwraps
pub fn make_atlas(file_paths: Vec<String>) -> bool {
    let mut atlas = Atlas {
        buffer: Vec::new(),
        width: 0,
        height: 32, // TODO: shouldn't be set here, should be taken from the images
    };

    for img_path in file_paths.iter() {
        let img = ImageReader::open(img_path)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let img_width = u32::from(img.width());
        atlas.append_image_buffer(
            img.into_bytes().to_vec(),
            img_width as usize
        );
    }

    println!("atlas.buffer.len(): {:?}", atlas.buffer.len());

    let atlas_img = image::save_buffer_with_format(
        String::from("./output.png"),
        atlas.buffer.as_slice(),
        atlas.width as u32,
        atlas.height as u32,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    );

    println!("{:?}", atlas_img);

    return true;
}
