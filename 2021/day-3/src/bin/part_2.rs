fn main() {
    let input = day_3::get_input();

    let threshold = input.len() / 2;
    let mut counts = [0; 16];

    for value in &input {
        for idx in 0..16 {
            if value & (1 << (15 - idx)) > 0 {
                counts[idx] = counts[idx] + 1
            }
        }
    }

    let mut gamma_rate: u16 = 0;
    let mut epsilon_rate: u16 = 0;

    let mut oxygen = input.clone();
    let mut co2 = input.clone();

    for idx in 0..16 {
        let bitmask = 1 << (15 - idx);
        if counts[idx] == 0 {
            continue;
        }

        if counts[idx] >= threshold {
            gamma_rate = gamma_rate | bitmask;
        } else {
            epsilon_rate = epsilon_rate | bitmask;
        }

        if oxygen.len() > 1 {
            filter_values(&mut oxygen, bitmask, |value| value & bitmask > 0)
        }

        if co2.len() > 1 {
            filter_values(&mut co2, bitmask, |value| value & bitmask == 0)
        }
    }

    if oxygen.len() != 1 {
        panic!("found multiple oxygen measurements");
    }

    let oxygen_rate = oxygen.first().unwrap();

    if co2.len() != 1 {
        panic!("found multiple oxygen measurements");
    }

    let co2_rate = co2.first().unwrap();

    println!("Gamma Rate: {}", gamma_rate);
    println!("Epsilon Rate: {}", epsilon_rate);
    println!("Answer #1: {}", gamma_rate as u64 * epsilon_rate as u64);

    println!("\nOxygen Generator Rating: {}", oxygen_rate);
    println!("CO2 Scrubber rating: {}", co2_rate);
    println!("Answer #2: {}", *oxygen_rate as u64 * *co2_rate as u64);
}

fn get_count(input: &Vec<u16>, bitmask: u16) -> usize {
    let mut count = 0;
    for value in input {
        if value & bitmask > 0 {
            count = count + 1;
        }
    }
    count
}

fn filter_values<F: FnMut(&u16) -> bool>(input: &mut Vec<u16>, bitmask: u16, mut condition: F) {
    let ones = get_count(&input, bitmask);
    if ones == 0 {
        return;
    }

    let zeros = input.len() - ones;

    if ones >= zeros {
        input.retain(condition);
    } else {
        input.retain(|value| !condition(value));
    }
}
