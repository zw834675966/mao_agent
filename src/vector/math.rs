//! Vector mathematics utilities (SIMD friendly, auto-vectorized).

/// Calculate the dot product of two equal-length vectors.
#[inline]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "Vectors must have same dimension");
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

/// Calculate the Euclidean L2 norm of a vector.
#[inline]
pub fn l2_norm(v: &[f32]) -> f32 {
    v.iter().map(|&x| x * x).sum::<f32>().sqrt()
}

/// Normalize a vector in-place to unit L2 length.
pub fn normalize_in_place(v: &mut [f32]) {
    let norm = l2_norm(v);
    if norm > f32::EPSILON {
        let inv_norm = 1.0 / norm;
        for x in v.iter_mut() {
            *x *= inv_norm;
        }
    }
}

/// Return a normalized clone of the vector.
pub fn normalize(v: &[f32]) -> Vec<f32> {
    let mut cloned = v.to_vec();
    normalize_in_place(&mut cloned);
    cloned
}

/// Calculate Cosine Similarity between two vectors: dot(a, b) / (||a|| * ||b||).
/// If vectors are pre-normalized, dot_product is equivalent and faster.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = dot_product(a, b);
    let norm_a = l2_norm(a);
    let norm_b = l2_norm(b);
    if norm_a <= f32::EPSILON || norm_b <= f32::EPSILON {
        0.0
    } else {
        (dot / (norm_a * norm_b)).clamp(-1.0, 1.0)
    }
}

/// Calculate Euclidean distance between two vectors.
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len(), "Vectors must have same dimension");
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let diff = x - y;
            diff * diff
        })
        .sum::<f32>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert_eq!(dot_product(&a, &b), 32.0);
    }

    #[test]
    fn test_normalization_and_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let c = vec![2.0, 0.0, 0.0];

        assert_eq!(cosine_similarity(&a, &b), 0.0);
        assert!((cosine_similarity(&a, &c) - 1.0).abs() < 1e-6);

        let norm_c = normalize(&c);
        assert_eq!(norm_c, vec![1.0, 0.0, 0.0]);
    }
}
