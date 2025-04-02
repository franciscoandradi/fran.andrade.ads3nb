// Testes Simples para Implementação de Árvore Binária de Busca (BST)
// Os alunos devem implementar a árvore para fazer estes testes passarem

#[cfg(test)]
mod bst_tests {
    // Importe sua implementação de BST aqui
     use crate::BST;
    
    #[test]
    fn test_bst_new_and_empty() {
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();
        
        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        println!("10: {}", bst.search(10));
        println!("5: {}", bst.search(5));
        println!("15: {}", bst.search(15));
        
        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));
        
        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}
#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

//usei do arquivo main da pasta da aula que fizemos em sala
// Esqueleto para implementação da BST pelos alunos
#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>
}

//usei tambem do arquivo main da pasta da aula que fizemos em sala
impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST {root: None}

    }
    
    //aqui e onde a funcao "is empty" vai ser criada, ela sera um bool pois e assim que vamos saber se esta vazia ou nao, com true ou false,
    //entao se arvore estiver vazia ela retorna true e caso contrario ela retorna false
    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    //usei tambem do arquivo main da pasta da aula que fizemos em sala
    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        let mut current: &mut Option<Box<Node>> = &mut self.root;

        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node{
                        value,
                        left: None,
                        right: None,
                    }));
                    return
                }

                Some(node) => {
                    if value < node.value {
                        current = &mut node.left;
                    } else if value > node.value {
                        current = &mut node.right;
                    } else {
                        return;
                    }
                }
            }
        }
    }
    
    //usei tambem do arquivo main da pasta da aula que fizemos em sala
    // Buscar um valor na árvore
    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }
}