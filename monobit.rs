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

fn main() {
    let random_sequence = generate_random_sequence(); // Генеруємо випадкову послідовність
    let is_random = monobit_test(&random_sequence); // Застосовуємо монобітний тест до послідовності

    if is_random {
        println!("The sequence of 20,000 bits is random."); // Послідовність є випадковою
    } else {
        println!("The sequence of 20,000 bits is not random."); // Послідовність не є випадковою
    }
}
