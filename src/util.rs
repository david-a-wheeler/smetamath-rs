pub fn ptr_eq<T>(x: &T, y: &T) -> bool {
    x as *const _ == y as *const _
}
