use rand::Rng;

// Функція для генерації випадкової послідовності байтів
fn generate_random_sequence() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut sequence = Vec::new();

    for _ in 0..2500 {
        let random_byte: u8 = rng.gen(); // Генеруємо випадковий байт
        sequence.push(random_byte); // Додаємо байт до послідовності
    }

    sequence
}

// Монобітний тест
fn monobit_test(sequence: &[u8]) -> bool {
    let mut ones_count = 0;
    let mut zeros_count = 0;

    for byte in sequence {
        let mut bitmask = 0x80;

        for _ in 0..8 {
            if byte & bitmask != 0 {
                ones_count += 1; // Збільшуємо лічильник одиниць
            } else {
                zeros_count += 1; // Збільшуємо лічильник нулів
            }

            bitmask >>= 1;
        }
    }

    ones_count > 9654 && ones_count < 10346 && zeros_count > 9654 && zeros_count < 10346
    // Перевіряємо, чи відповідають лічильники встановленим константам
}

// Тест на максимальну довжину серії
fn series_test(sequence: &[u8]) -> bool {
    let mut max_ones_series = 0;  // Максимальна довжина серії одиниць
    let mut max_zeros_series = 0; // Максимальна довжина серії нулів
    let mut ones_series = 0;      // Поточна довжина серії одиниць
    let mut zeros_series = 0;     // Поточна довжина серії нулів

    for byte in sequence {
        let mut bitmask = 0x80;

        for _ in 0..8 {
            if byte & bitmask != 0 {
                ones_series += 1;  // Збільшуємо лічильник серії одиниць
                zeros_series = 0;  // Обнуляємо лічильник серії нулів

                if ones_series > max_ones_series {
                    max_ones_series = ones_series;  // Оновлюємо максимальну довжину серії одиниць
                }
            } else {
                zeros_series += 1;  // Збільшуємо лічильник серії нулів
                ones_series = 0;     // Обнуляємо лічильник серії одиниць

                if zeros_series > max_zeros_series {
                    max_zeros_series = zeros_series;  // Оновлюємо максимальну довжину серії нулів
                }
            }

            bitmask >>= 1;
        }
    }

    max_ones_series <= 36 && max_zeros_series <= 36  // Перевіряємо, чи не перевищують найдовші серії максимально припустиме значення
}


// Тест Поккера
fn poker_test(sequence: &[u8]) -> bool {
    let block_length = 4; // Довжина блоку Поккера
    let num_blocks = sequence.len() / block_length; // Кількість блоків Поккера

    let mut block_counts = vec![0; 16]; // Лічильники для кожного блоку Поккера

    for i in 0..num_blocks {
        let block = &sequence[i * block_length..(i + 1) * block_length];
        let index = calculate_block_index(block);
        block_counts[index] += 1;
    }

    let sum_squared: u64 = block_counts.iter().map(|&count| count as u64 * count as u64).sum();
    let x3 = (16 * sum_squared as u64) / num_blocks as u64 - num_blocks as u64;

    // Перевіряємо, чи входить параметр X3 у встановлений діапазон
    let x3_lower_bound: u64 = 1;
    let x3_upper_bound: u64 = 57;

    x3 > x3_lower_bound && x3 < x3_upper_bound
}

// Функція для обчислення індексу блоку Поккера
fn calculate_block_index(block: &[u8]) -> usize {
    let mut index = 0;

    for &bit in block {
        index <<= 1;
        if bit == 1 {
            index |= 0x01;
        }
    }

    index as usize
}

// Тест на довжину серії
fn series_length_test(sequence: &[u8]) -> bool {
    let mut ones_series = vec![0; 7];  // Створення вектора для зберігання кількості серій одиниць довжиною 1, 2, 3, ..., 6 (та більше)
    let mut zeros_series = vec![0; 7]; // Створення вектора для зберігання кількості серій нулів довжиною 1, 2, 3, ..., 6 (та більше)
    let mut series_length = 0;  // Змінна для зберігання поточної довжини серії

    for bit in sequence {  // Ітерація по кожному біту у послідовності
        if *bit == 1 {  // Якщо біт дорівнює 1
            series_length += 1;  // Збільшення довжини серії
            zeros_series[series_length] += 1;  // Збільшення лічильника серій нулів відповідної довжини
            series_length = 0;  // Скидання довжини серії до 0

            if zeros_series[6] > 223 {  // Якщо кількість серій нулів довжиною 6 (та більше) більше 223
                zeros_series[6] = 223;  // Обмежуємо кількість серій нулів довжиною 6 (та більше) до 223
            }
        } else {  // Якщо біт дорівнює 0
            series_length += 1;  // Збільшення довжини серії
            ones_series[series_length] += 1;  // Збільшення лічильника серій одиниць відповідної довжини
            series_length = 0;  // Скидання довжини серії до 0

            if ones_series[6] > 223 {  // Якщо кількість серій одиниць довжиною 6 (та більше) більше 223
                ones_series[6] = 223;  // Обмежуємо кількість серій одиниць довжиною 6 (та більше) до 223
            }
        }
    }

    // Функція, яка перевіряє, чи значення потрапляє в відповідний інтервал
    let in_range = |value: u32, lower: u32, upper: u32| value >= lower && value <= upper;

    // Перевірка, чи всі кількості серій потрапляють у відповідні інтервали
    in_range(ones_series[1], 2267, 2733)
        && in_range(ones_series[2], 1079, 1421)
        && in_range(ones_series[3], 502, 748)
        && in_range(ones_series[4], 223, 402)
        && in_range(ones_series[5], 90, 223)
        && in_range(ones_series[6], 90, 223)
        && in_range(zeros_series[1], 2267, 2733)
        && in_range(zeros_series[2], 1079, 1421)
        && in_range(zeros_series[3], 502, 748)
        && in_range(zeros_series[4], 223, 402)
        && in_range(zeros_series[5], 90, 223)
        && in_range(zeros_series[6], 90, 223)
}

fn main() {
    let random_sequence = generate_random_sequence();
    println!("Random Sequence: {:?}", random_sequence);

    let is_random_monobit = monobit_test(&random_sequence);
    let is_random_series = series_test(&random_sequence);
    let is_random_poker = poker_test(&random_sequence);
    let is_series_length = series_length_test(&random_sequence);

    if is_random_monobit && is_random_series && is_random_poker && is_series_length {
        println!("The sequence of 20,000 bits is random.");
    } else {
        println!("The sequence of 20,000 bits is not random.");
    }
}
