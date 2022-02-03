pub fn run() {
    // Primitive array
    // let arr1 = [1, 2, 3];
    // let arr2 = arr1;
    // println!("Values: {:?}", (arr1, arr2)); // [1, 2, 3], [1, 2, 3]
    // Vector
    // Non primitives: & is necessary to point to the resource (if re-assigned, the first variable is no longer holding the value by default)
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));
}