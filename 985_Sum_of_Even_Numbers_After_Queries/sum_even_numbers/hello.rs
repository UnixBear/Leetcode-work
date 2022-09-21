fn make_reference_one(r: &mut &i32) {
    *r = &1;
}

fn main() {
    let mut num_ref = &2;
    dbg!(num_ref); // is &2
    make_reference_one(&mut num_ref);
    dbg!(num_ref); // is now &1
}
