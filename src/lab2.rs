fn find_second_smallest_positive(array: &[f64]) -> Option<f64> {
    let mut positives: Vec<f64> = array.iter().cloned().filter(|&x| x > 0.0).collect();
    positives.sort_by(|a, b| a.partial_cmp(b).unwrap());
    positives.get(1).cloned()
}

fn find_second_largest_negative(array: &[f64]) -> Option<f64> {
    let mut negatives: Vec<f64> = array.iter().cloned().filter(|&x| x < 0.0).collect();
    negatives.sort_by(|a, b| b.partial_cmp(a).unwrap());
    negatives.get(1).cloned()
}

fn calculate_fourth_powers_sum(array: &[f64], elem1: f64, elem2: f64) -> f64 {
    let index1 = array.iter().position(|&x| x == elem1).expect("Елемент 1 не знайдено.");
    let index2 = array.iter().position(|&x| x == elem2).expect("Елемент 2 не знайдено.");

    let (start, end) = if index1 < index2 {
        (index1 + 1, index2)
    } else {
        (index2 + 1, index1)
    };

    array[start..end]
        .iter()
        .map(|&x| x.powi(4))
        .sum()
}

pub fn lab2 (array: &[f64]) -> () {
    let second_smallest_positive = find_second_smallest_positive(array).expect("Не знайдено другий найменший додатний елемент.");
    let second_largest_negative = find_second_largest_negative(array).expect("Не знайдено другий найбільший від'ємний елемент.");
    let sum = calculate_fourth_powers_sum(array, second_smallest_positive, second_largest_negative);

    println!("Лабораторна робота 2: Rust. Робота із масивами.");
    println!("Масив: {:?}", array);
    println!("Другий найменший додатний: {}", second_smallest_positive);
    println!("Другий найбільший від'ємний: {}", second_largest_negative);
    println!("Сума четвертих степенів: {:.4}", sum);
}