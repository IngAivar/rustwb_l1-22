fn remove_element(vec: &mut Vec<i32>, index: usize) -> Option<i32> {
    if index < vec.len() {
        Some(vec.remove(index))
    } else {
        None
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // Удаляем элемент с индексом 2 (т.е. число 3)
    let removed = remove_element(&mut numbers, 2);
    println!("Удаленный элемент: {:?}", removed);
    println!("Новый вектор: {:?}", numbers);
}