//NIVEL 1

use std::collections::VecDeque; 

fn main() {
    let mut vetor: VecDeque<i32> = VecDeque::from(vec![34, 7, 23, 32, 5, 62, 31, 12, 43, 3]);

    println!("Vetor original: {:?}", vetor);

    let mut vetor_vec: Vec<_> = vetor.iter().cloned().collect();
    vetor_vec.sort();

    vetor = VecDeque::from(vetor_vec);

    println!("Vetor ordenado: {:?}", vetor);
}

//NIVEL 2

struct VetorOrdenavel {
     dados: Vec<i32>,
}

impl VetorOrdenavel {
    fn novo() -> Self {
        let mut dados = [34, 7, 23, 32, 5, 62, 31, 12, 43, 3];
        VetorOrdenavel {  dados: dados.to_vec() }
    }

    fn exibir(&self) {
        println!("{:?}", self.dados);
    }

    fn ordenar(&mut self) {
        Self::quicksort(&mut self.dados);
    }

    fn quicksort(vetor: &mut [i32]) {
        let len = vetor.len();
        if len <= 1 {
            return;
        }

        let pivot_index = len / 2;
        let pivot = vetor[pivot_index];

        let mut left = 0;
        let mut right = len - 1;

        while left <= right {
            while vetor[left] < pivot {
                left += 1;
            }
            while vetor[right] > pivot {
                right = right.saturating_sub(1);
            }
            if left <= right {
                vetor.swap(left, right);
                left += 1;
                right = right.saturating_sub(1);
            }
        }

        if right > 0 {
            Self::quicksort(&mut vetor[0..=right]);
        }
        if left < len {
            Self::quicksort(&mut vetor[left..]);
        }
    }
}

fn main() {
    let mut vetor = VetorOrdenavel::novo();

    println!("Vetor original:");
    vetor.exibir();

    vetor.ordenar();

    println!("Vetor ordenado:");
    vetor.exibir();
}