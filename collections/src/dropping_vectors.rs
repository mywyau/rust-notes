// Like any other struct, a vector is freed when it goes out of scope

pub fn notes() {
    let v = vec![1, 2, 3, 4];

    // do something using v
}  // v goes out of scope hence is freed here