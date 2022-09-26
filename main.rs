use bmp::{self, Pixel};
use std::io::{stdout, Write};


fn draw_pixel(path: &str) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };
    image.set_pixel(50, 50, bmp::Pixel::new(255, 255, 255));

    image.save(path).expect("This should save correctly.");
}


fn draw_line(path: &str, start: (u32, u32), end: (u32, u32), color: Pixel) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };

        for x in start.0..end.0 {
            image.set_pixel(x, x, color);
        }

        // for x in (start.0..end.0).rev() {
        //     image.set_pixel(x, x, color);


    image.save(path).expect("This should save correctly.");
}


fn draw_cross(path: &str, start: (u32, u32), end: (u32, u32), color: Pixel) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };

        for x in start.0..end.0 {
            image.set_pixel(x, x, color);
        }

        for x in (start.0..end.0).rev() {
            image.set_pixel(x, end.1-x-1, color);
        }

    image.save(path).expect("This should save correctly.");
}

fn draw_outline(color: Pixel, xStart: u32, xEnd: u32, yStart: u32, yEnd: u32, image: &mut bmp::Image) {
    

    // println!("X start: ");

    // let mut input_line = String::new();
    // std::io::stdin()
    //     .read_line(&mut input_line)
    //     .expect("Failed to read line");
    // let xStart: u32 = input_line.trim().parse().expect("Input not an integer");

    // println!("Y start: ");

    // let mut input_line = String::new();
    // std::io::stdin()
    //     .read_line(&mut input_line)
    //     .expect("Failed to read line");
    // let yStart: u32 = input_line.trim().parse().expect("Input not an integer");

    // println!("X end: ");

    // let mut input_line = String::new();
    // std::io::stdin()
    //     .read_line(&mut input_line)
    //     .expect("Failed to read line");
    // let xEnd: u32 = input_line.trim().parse().expect("Input not an integer");

    // println!("Y end: ");

    // let mut input_line = String::new();
    // std::io::stdin()
    //     .read_line(&mut input_line)
    //     .expect("Failed to read line");
    // let yEnd: u32 = input_line.trim().parse().expect("Input not an integer");

    if xEnd > image.get_width() || yEnd > image.get_height() {
        panic!("no good");
    }


    for pix in xStart..=xEnd {
        image.set_pixel(pix, yStart, color);
        image.set_pixel(pix, yEnd, color);
    }


    for pix in yStart..=yEnd {
        image.set_pixel(xStart, pix, color);
        image.set_pixel(xEnd, pix, color);
    }

    


}


fn draw_filled(path: &str, color: Pixel, xStart: u32, xEnd: u32, yStart: u32, yEnd: u32) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };

    for x in xStart..xEnd {
        for y in yStart..yEnd {
            image.set_pixel(x, y, bmp::consts::HOT_PINK);
        }
    }

    
    
    draw_outline(color, xStart, xEnd, yStart, yEnd, &mut image);
    image.save(path).expect("This should save correctly.");
}


fn rainbow(path: &str) {
    let mut image = match bmp::open(path) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 139)
    };
    for x in (0..100).rev() {
        for y in 0..139 {
            if y >= 0 && y <= 19 {
                image.set_pixel(x, y, bmp::consts::RED);
            }
            if y >= 20 && y <= 39 {
                image.set_pixel(x, y, bmp::consts::ORANGE);

            }
            if y >= 40 && y <= 59 {
                image.set_pixel(x, y, bmp::consts::YELLOW);

            }
            if y >= 60 && y <= 79 {
                image.set_pixel(x, y, bmp::consts::GREEN);
            }
            if y >= 80 && y <= 99 {
                image.set_pixel(x, y, bmp::consts::AQUA);

            }
            if y >= 100 && y <= 119 {
                image.set_pixel(x, y, bmp::consts::BLUE);
    
            }
            if y >= 120 && y < 139 {
                image.set_pixel(x, y, bmp::consts::INDIGO);

            }

        } 
        image.save(path).expect("This should save correctly.");
    }

}

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");

    // let reverse = std::env::args().nth(3).expect("You must provide a end.");
    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();
    

    match op.as_str() {
        "pixel\n" => draw_pixel(path.as_str()),
        "line\n" => draw_line(path.as_str(), (0,0),(100,100), bmp::consts::RED),
        "cross\n" => draw_cross(path.as_str(), (0,0),(100,100), bmp::consts::RED),
        "outline\n" => {
            





            println!("X start: ");

            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let xStart: u32 = input_line.trim().parse().expect("Input not an integer");
        
            println!("Y start: ");
        
            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let yStart: u32 = input_line.trim().parse().expect("Input not an integer");
        
            println!("X end: ");
        
            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let xEnd: u32 = input_line.trim().parse().expect("Input not an integer");
        
            println!("Y end: ");
        
            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let yEnd: u32 = input_line.trim().parse().expect("Input not an integer");



            let mut image = match bmp::open(path.clone()) {
                Ok(i) => i,
                Err(_) => bmp::Image::new(100, 100)
            };





            draw_outline( bmp::consts::RED, xStart, xEnd, yStart, yEnd, &mut image);
            image.save(path).expect("This should save correctly.");
        }
        "filled\n" => {



            println!("X start: ");

            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let xStart: u32 = input_line.trim().parse().expect("Input not an integer");
        
            println!("Y start: ");
        
            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let yStart: u32 = input_line.trim().parse().expect("Input not an integer");
        
            println!("X end: ");
        
            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let xEnd: u32 = input_line.trim().parse().expect("Input not an integer");
        
            println!("Y end: ");
        
            let mut input_line = String::new();
            std::io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let yEnd: u32 = input_line.trim().parse().expect("Input not an integer");


            draw_filled(path.as_str(), bmp::consts::RED, xStart, xEnd, yStart, yEnd);
        },
        "rainbow\n" => {
            rainbow(path.as_str());
        },



        _ =>  {
            eprintln!("The operation {op} was not recognised!");
        } 
    };
}


