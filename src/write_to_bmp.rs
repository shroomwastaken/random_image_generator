use std::{fs::File, io::Write};

#[allow(unused)]
pub fn write_to_bmp(file: &mut File, pixels: Vec<u16>, res: &(u32, u32)) {
    write_bmp_and_dbi_headers(file, res);
    write_pixel_array(file, pixels, res);
}

#[allow(unused)]
pub fn write_bmp_and_dbi_headers(file: &mut File, resolution: &(u32, u32)) {
    // docs found at https://en.wikipedia.org/wiki/BMP_file_format
    file.write_all(&[0x42, 0x4d]); // bmp header field

    // header_size + (floor((bits_per_pixel * image_width + 31) / 32) * 4) * image_height
    file.write_all(&(54 + f32::floor(24f32 * resolution.0 as f32 + 31f32 / 32f32) as u32 * 4 * resolution.1).to_le_bytes()); // size of file

    file.write_all(&[
        0x00, 0x00,             // optional values
        0x00, 0x00,             // can be 0

        0x36, 0x00, 0x00, 0x00, // pointer to start of pixel array
        0x28, 0x00, 0x00, 0x00, // size of BITMAPINFOHEADER type header (40 bytes)
    ]);

    file.write_all(&resolution.0.to_le_bytes()); // width of image
    file.write_all(&resolution.1.to_le_bytes()); // height of image

    file.write_all(&[
        0x01, 0x00,               // number of color planes (always 1)
        0x08, 0x00,               // bits per pixel (24 in this case)
        0x00, 0x00, 0x00, 0x00,   // compression method (BI_RGB/none in this case) 
        0x00, 0x00, 0x00, 0x00,   // size of raw bitmap data, a dummy 0 can be given for BI_RBG bitmaps

        0x13, 0x0b, 0x00, 0x00,   // resolution of image (width, height)
        0x13, 0x0b, 0x00, 0x00,   // ^ wiki says it should be 2835 each

        0x00, 0x00, 0x00, 0x00,   // number of colors in color pallet, if 0 it defaults to 2 ^ bits_per_pixel
        0x00, 0x00, 0x00, 0x00,   // the number of important colors used, or 0 when every color is important
    ]); 
}

#[allow(unused)]
pub fn write_pixel_array(file: &mut File, pixels: Vec<u16>, res: &(u32, u32)) {
    for i in 0..pixels.len() {
        file.write_all(&pixels[i].to_le_bytes());
    }
}