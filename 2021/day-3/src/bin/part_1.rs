fn main() {
    let input = day_3::get_input();
    let threshold = input.len() / 2;
    let mut counts = [0; 16];

    for value in input {
        for idx in 0..16 {
            if value & (1 << (15 - idx)) > 0 {
                counts[idx] = counts[idx] + 1
            }
        }
    }

    let mut gamma_rate: u16 = 0;
    let mut epsilon: u16 = 0;

    for idx in 0..16 {
        if counts[idx] == 0 {
            continue;
        }

        if counts[idx] > threshold {
            gamma_rate = gamma_rate | 1 << (15 - idx);
        } else {
            epsilon = epsilon | 1 << (15 - idx);
        }
    }

    println!("{}", gamma_rate as u32 * epsilon as u32);
}
