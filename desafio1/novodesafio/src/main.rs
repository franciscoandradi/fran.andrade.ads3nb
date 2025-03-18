use novodesafio::multiply_array;

fn main() {
    let arr = [2, 3, 4]; // Array de números
    let result = unsafe { multiply_array(arr.as_ptr(), arr.len()) }; // Chamada unsafe
    println!("O produto dos elementos do array é: {}", result);
}