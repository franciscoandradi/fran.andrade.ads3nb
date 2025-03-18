pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len  {
        product *= unsafe { *ptr.offset(i as isize) }; //foi necessário colocar em unsafe para rodar o test
    }
    product
}
//O problema é que o loop começa em 1, "for i in 1..len"
//ignorando o primeiro elemento do array, levando a um resultado incorreto.
//mudando para "for i in 0..len" assim o teste funcionará
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}

