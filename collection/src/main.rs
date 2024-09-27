use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn btree_set_example() {
    let mut set = BTreeSet::new();

    set.insert("a");
    set.insert("b");
    set.insert("c");
    set.insert("a");
    set.insert("e");
    set.insert("d");

    println!("{:?}", set);

    set.remove("b");
    println!("{:?}", set);

    for value in set.iter() {
        println!("{}", value);
    }
}

fn bttree_map_example() {
    let mut map = BTreeMap::new();

    // Insert some key-value pairs
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");

    println!("{:?}", map);

    if let Some(value) = map.get(&2) {
        println!("The value for key 2 is: {}", value);
    }

    map.remove(&1);
    println!("{:?}", map);

    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
}

fn btreemap_speed() {
    let mut map = BTreeMap::new();

    for i in 0..1_000_000 {
        map.insert(i, i);
    }

    let start = std::time::Instant::now();
    for i in 0..1_000_000 {
        map.get(&i);
    }
    println!("BTreeMap get: {:?}", start.elapsed());
}

fn array_speed() {
    let mut array = [0; 1_000_000];

    let start = std::time::Instant::now();
    for i in 0..1_000_000 {
        array[i] = i;
    }
    println!("Array write time: {:?}", start.elapsed());

    let sum: usize = array.iter().sum();
    println!("Sum: {}", sum);
}

fn main() -> std::io::Result<()> {
    println!("BTreeSet Example");
    btree_set_example();

    println!("BTreeMap Example");
    bttree_map_example();

    println!("BTreeMap Speed");
    btreemap_speed();

    println!("Array Speed");
    array_speed();

    Ok(())
}
