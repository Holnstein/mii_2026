use nalgebra::DMatrix;

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub(crate) fn rotation_cos_sin(a_pp: f32, a_qq: f32, a_pq: f32) -> Option<(f32, f32)> {
    if a_pq == 0.0 {
        return None;
    }
    let c = (a_qq - a_pp) / (2.0 * a_pq);
    let root = (c * c + 1.0).sqrt();
    let t = if c > 0.0 { 1.0 / (c + root) } else { 1.0 / (c - root) };
    let cos = 1.0 / (1.0 + t * t).sqrt();
    let sin = t * cos;
    Some((cos, sin))
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub(crate) fn rotate_a(
    a: &mut DMatrix<f32>,
    p: usize,
    q: usize,
    t: f32,
    cos: f32,
    sin: f32,
) -> f32 {

    let a_pp: f32 = a[(p, p)];
    let a_qq: f32 = a[(q, q)];
    let a_pq: f32 = a[(p, q)];

    a[(p, p)] = a_pp - a_pq * t;
    a[(q, q)] = a_qq + a_pq * t;

    let fraction = sin / (1.0 + cos);
    for r in 0..a.nrows() {
        if r != q && r != p {
            let a_rp = a[(r, p)];
            a[(r, p)] = a_rp - sin * (a[(r, q)] + fraction * a_rp);
            a[(r, q)] = a[(r, q)] + sin * (a_rp - fraction * a[(r, q)]);
            a[(p, r)] = a[(r, p)];
            a[(q, r)] = a[(r, q)];
        }
    }

    a[(p, q)] = 0.0;
    a[(q, p)] = 0.0;

    a_pq.powi(2)
}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub(crate) fn rotate_v(v: &mut DMatrix<f32>, p: usize, q: usize, cos: f32, sin: f32) {

    for r in 0..v.nrows() {
        let v_rp = v[(r, p)];
        v[(r, p)] = v_rp * cos - v[(r, q)] * sin;
        v[(r, q)] = v_rp * sin + v[(r, q)] * cos;
    }

}

// TODO: написать контракт вручную
// TODO: написать доккомментарий вручную
pub fn off_sum(a: &DMatrix<f32>) -> f32 {
    let mut off_sum = 0.0;
    let n  = a.nrows();

    for p in 0..n - 1 {
        for q in p + 1..n {
            off_sum += a[(p, q)] * a[(p, q)];
        }
    }

    off_sum
}

// TODO: написать тест вручную
