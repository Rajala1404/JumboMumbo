/// Returns a new Vec that doesn't contain the specified elements
pub async fn remove_elements_vec<T: Clone>(source: &Vec<T>, remove: Vec<usize>) -> Vec<T> {
    let mut result = Vec::new();

    for (i, enemy) in source.iter().enumerate() {
        if !remove.contains(&i)  {
            result.push(enemy.to_owned());
        }
    }

    result
}