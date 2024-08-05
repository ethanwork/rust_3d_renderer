pub fn swap<T: Copy>(a: &mut T, b: &mut T) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}