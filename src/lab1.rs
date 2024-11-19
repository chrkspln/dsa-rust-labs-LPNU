use std::io;
use std::io::Write;

const X_BOT: f64 = 0.0;
const X_TOP: f64 = 1.0;
const K: u16 = 10;
const LN_3: f64 = 1.0986122886681098; // Попередньо обчислене значення ln(3)

/// Основна функція для лабораторної роботи
pub fn lab1() {
    println!("Лабораторна робота 1: Rust. Функції та ітераційні процеси.");
    print!("Введіть кількість врахованих термінів ряду: ");
    io::stdout().flush().unwrap();
    let mut usr_input = String::new();
    io::stdin().read_line(&mut usr_input).unwrap();
    let n: u16 = usr_input.trim().parse().expect("Помилка вводу!");

    print!("Введіть значення для епсілон: ");
    io::stdout().flush().unwrap();
    usr_input.clear();
    io::stdin().read_line(&mut usr_input).unwrap();
    let epsilon: f64 = usr_input.trim().parse().expect("Помилка вводу!");

    println!("{:<15} {:<15} {:<15} {:<15}", "x", "Sn", "Se", "y_exact");

    let step = (X_TOP - X_BOT) / (K - 1) as f64;
    for j in 0..=K {
        let x = X_BOT + j as f64 * step;
        println!(
            "{:<15.6} {:<15.6} {:<15.6} {:<15.6}",
            x,
            sum1(x, n),
            sum2(x, epsilon),
            y(x)
        );
    }
}

/// Розрахунок суми через визначену кількість членів ряду (N)
fn sum1(x: f64, num_terms: u16) -> f64 {
    (0..=num_terms).map(|k| term_of_series(x, k as i32)).sum()
}

/// Розрахунок суми ряду до досягнення точності (epsilon)
fn sum2(x: f64, epsilon: f64) -> f64 {
    let mut sum = 0.0;
    let mut current_term: f64 = 1.0;
    let mut k = 0;
    while current_term.abs() > epsilon {
        current_term = term_of_series(x, k);
        sum += current_term;
        k += 1;
    }
    sum
}

/// Розрахунок окремого члена ряду
fn term_of_series(x: f64, k: i32) -> f64 {
    (LN_3 * x).powi(k) / factorial(k) as f64
}

/// Обчислення точного значення y(x) = 3^x
fn y(x: f64) -> f64 {
    3.0_f64.powf(x)
}

/// Обчислення факторіалу для цілого числа
fn factorial(n: i32) -> i32 {
    (1..=n).product()
}
