// Реализовать пересечение двух неупорядоченных множеств.
use std::collections::HashSet;

fn main() {
    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();

    //  пересечение
    let intersection: HashSet<_> = set_a.intersection(&set_b).collect();

    println!("Intersection: {:?}", intersection);
}
