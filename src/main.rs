mod vec;

fn main() {
    let a = vec::Vec3::new(1.0, 4.0, 3.0);
    vec::print_len(&a);

    let b = vec::Vec2::new(3.0, 4.0);
    vec::print_len(&b);
    vec::print_len(&vec::mul(2.0, &b));

    //print_len(Vector::add(&a, &a));
    vec::print_len(&vec::add(&a, &a));
    vec::print_len(&vec::normalize(&a));

    println!("Hello, world!");
}
