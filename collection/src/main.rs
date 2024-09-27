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

fn main() -> std::io::Result<()> {
    // btree_set_example();

    bttree_map_example();

    Ok(())
}
