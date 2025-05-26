use std::collections::VecDeque; 

fn main() {
    let mut vetor = vec![34, 7, 23, 32, 5, 62, 31, 12, 43, 3];

    println!("Vetor original: {:?}", vetor);

    // Ordenando o vetor com sort()
    vetor.sort();

    println!("Vetor ordenado: {:?}", vetor);
}