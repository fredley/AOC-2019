pub fn day_eight(input: String) -> () {
    let width = 25;
    let height = 6;

    let mut x: usize;
    let mut y: usize;
    let mut layer = 0;
    let layers = input.len() / (width * height);

    let mut fewest_zeroes = width * height;
    let mut layer_zeroes: usize;
    let mut layer_ones: usize;
    let mut layer_twos: usize;
    let mut checksum = 0;

    let mut char_iter = input.chars();

    let mut image = vec!['2'; width * height];

    while layer < layers {
        y = 0;
        layer_zeroes = 0;
        layer_ones = 0;
        layer_twos = 0;
        while y < height{
            x = 0;
            while x < width {
                let current_char = char_iter.next().unwrap();
                match current_char {
                    '0' => layer_zeroes += 1,
                    '1' => layer_ones += 1,
                    '2' => layer_twos += 1,
                    _ => (),
                }
                let idx = width * y + x;
                if image[idx] == '2' && current_char != '2' {
                    image[idx] = current_char;
                }
                x += 1;
            }
            y += 1;
        }
        if layer_zeroes < fewest_zeroes {
            fewest_zeroes = layer_zeroes;
            checksum = layer_ones * layer_twos;
        }
        layer += 1;
    }

    println!("Checksum: {}", checksum);
    // render image
    println!("Image:");
    let mut image_iter = image.iter();
    y = 0;
    x = 0;
    while y < height {
        x = 0;
        while x < width {
            let mut out_pixel = ' ';
            let current_pixel = image_iter.next().unwrap();
            match current_pixel {
                '1' => out_pixel = '*',
                _ => (),
            }
            print!("{}", out_pixel);
            x += 1;
        }
        y += 1;
        print!("\n");
    }
}
