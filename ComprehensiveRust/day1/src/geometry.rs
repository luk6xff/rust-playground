// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
pub fn magnitude(v: &[f64;3]) -> f64 {
    return (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
pub fn normalize(v: &mut [f64;3]) {
    let mag = magnitude(&v);
    v[0] = v[0]/mag;
    v[1] = v[1]/mag;
    v[2] = v[2]/mag;
}