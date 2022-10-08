use bitvec::{macros::internal::funty::Fundamental, prelude::*, view::BitView};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::{str, thread, time::Duration};

#[derive(Clone, Copy)]
struct Cursor {
    x: u32,
    y: u32,
}

enum ErrorTypes {
    NoMoreSpaceError,
}

fn change_cursor(mut cursor: Cursor, max_cursor: Cursor) -> Result<Cursor, ErrorTypes> {
    if cursor.x < max_cursor.x {
        cursor.x += 1
    } else {
        if cursor.y < max_cursor.y {
            cursor.y += 1;
        } else {
            return Err(ErrorTypes::NoMoreSpaceError);
        }
    }
    Ok(cursor)
}

fn encode(mut img: DynamicImage, mut secret_message: String) -> DynamicImage {
    secret_message.push_str("END_OF_SECRET");
    let secret_message_bytes = secret_message.as_bytes().to_vec();

    let dimension = img.dimensions();
    let max_cursor = Cursor {
        x: dimension.0,
        y: dimension.1,
    };
    let mut cursor = Cursor { x: 0, y: 0 };

    for i in secret_message_bytes {
        let message_bits = i.view_bits::<Msb0>();
        for message_bit in message_bits {
            if *message_bit {
                print!("1")
            } else {
                print!("0")
            }
            let mut pixel = img.get_pixel(cursor.x, cursor.y);

            let pixel_r_bits = pixel.0[0].view_bits_mut::<Lsb0>();

            pixel_r_bits.set(0, message_bit.as_bool());

            let new_rgba = Rgba([
                pixel_r_bits.load::<u8>(),
                pixel.0[1],
                pixel.0[2],
                pixel.0[3],
            ]);

            img.put_pixel(cursor.x, cursor.y, new_rgba);

            cursor = match change_cursor(cursor, max_cursor) {
                Ok(v) => v,
                // TODO CHANGE LATER
                Err(_) => cursor,
            };
        }
    }

    img
}
fn decode(img: DynamicImage) -> String {
    let dimension = img.dimensions();
    let mut cursor = Cursor { x: 0, y: 0 };
    let max_cursor = Cursor {
        x: dimension.0 - 1,
        y: dimension.1 - 1,
    };
    let mut secret_message = String::from("");

    let mut total_bit: String = String::from("");

    while !secret_message.contains("END_OF_SECRET") {
        let mut bits: Vec<bool> = Vec::new();
        for _ in 0..7 {
            let pixel = img.get_pixel(cursor.x, cursor.y);
            let pixel_r_bits = pixel.0[0].view_bits::<Lsb0>();
            let bit = *pixel_r_bits.get(0).unwrap();
            bits.push(bit);

            if bit {
                total_bit.push('1');
            } else {
                total_bit.push('0');
            }

            cursor = match change_cursor(cursor, max_cursor) {
                Ok(v) => v,
                Err(_) => cursor,
            }
        }
        let mut bit_vec = bitvec![u8, Msb0;];

        for bit in bits {
            bit_vec.push(bit);
        }
        let mut u8_vec: Vec<u8> = Vec::new();
        u8_vec.push(bit_vec.load::<u8>());

        secret_message.push_str(match str::from_utf8(u8_vec.as_slice()) {
            Ok(v) => v,
            Err(_) => "END_OF_SECRET_DECODE_FAIL",
        });
    }

    secret_message
}

fn main() {
    println!("START");
    let img = image::open("./sample.png").unwrap();
    println!("LOAD");
    let encoded_image = encode(img, String::from("hi nice to meet you"));
    println!("ENCODED");
    encoded_image.save("./result.png").unwrap();
    println!("SAVED");

    let encoded_img = image::open("./result.png").unwrap();
    println!("LOADED RESULT");
    let secret_message = decode(encoded_img);
    println!("{}", secret_message);

    println!("DONE");

    // let mut pixel = img.get_pixel(0, 0);
    // println!("pixel at 0, 0 : {:?}", pixel);
    // pixel[0] = 185;

    // // let bit = pixel[0].view_bits::<Msb0>();
    // let bit = pixel[0].view_bits_mut::<Msb0>();

    // bit.set(7, false);
    // println!("Bit data of pixel, ({})", pixel[0]);

    // for (x, y, rgba) in img.pixels() {
    //     // println!("{}, {}, {:?}", x, y, rgba);
    //     println!("{:?}", rgba.0[0]);
    // }
}
