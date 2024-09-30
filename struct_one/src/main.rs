struct S {
    x: i32,
}

const S: S = S { x: 2 };

fn main() {
    let v = &mut S;
    v.x += 1;
    S.x += 1;

    println!("v.x = {}; S.x = {}", v.x, S.x);
    S.x += 1;
    println!("Original constant S.x = {}", S.x);
}
