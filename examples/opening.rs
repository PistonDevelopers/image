//! An example of opening an image.

#![feature(convert)]

extern crate image;

use std::env;
use std::fs::File;
use std::path::PathBuf;

use image::GenericImage;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Use the open function to load an image from a PAth.
    // ```open``` returns a dynamic image.
    let im = image::open(&PathBuf::from(file.clone())).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", im.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    let ref mut fout = File::create(&PathBuf::from(format!("{}.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    let _ = im.save(fout, image::PNG).unwrap();
}
