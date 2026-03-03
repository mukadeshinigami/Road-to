/// Функция check_vibe для задачи «FizzBuzz 2.0»
/// 
/// Использует match для классификации числа:
/// - Делится на 3 и 5 -> "FizzBuzz"
/// - Диапазон 1..=10 -> "Small number"
/// - Отрицательное -> "Negative vibe"
/// - Остальное -> "Just a number"
pub fn check_vibe(n: i32) -> &'static str {
    match n {
        // Охранное выражение (guard) для проверки делимости на 3 и 5 (15)
        n if n % 3 == 0 && n % 5 == 0 => "FizzBuzz",
        
        // Использование диапазона
        1..=10 => "Small number",
        
        // Охранное выражение для отрицательных чисел
        n if n < 0 => "Negative vibe",
        
        // Значение по умолчанию
        _ => "Just a number",
    }
}
