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

fn main() -> std::io::Result<()> {
    btree_set_example();

    Ok(())
}
