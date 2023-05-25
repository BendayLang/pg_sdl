use ndarray::{Array1, Array2};

/// Gauss-Seidel method for solving linear systems of equations (Ax = b).
///
/// https://en.wikipedia.org/wiki/Gauss%E2%80%93Seidel_method
pub fn gauss_seidel(a: Array2<f64>, b: Array1<f64>, x0: Option<Array1<f64>>, threshold: f64) -> Array1<f64> {
	let n = b.dim();
	let mut x0 = if x0.is_some() {
		x0.unwrap()
	} else {
		let mut x0 = Array1::<f64>::zeros(n);
		for i in 0..n {
			x0[i] = 10.0
		}
		x0
	};

	// println!("Ax=b :\n{} * {} = {}", a, x0, b);
	let mut x = Array1::<f64>::zeros(n);
	let mut step: u16 = 0;
	loop {
		step += 1;

		for i in 0..n {
			x[i] = if a[[i, i]] == 0.0 { 1.0 } else { 1.0 / a[[i, i]] }
				* (b[i]
					- (0..i).map(|j| a[[i, j]] * x[j]).sum::<f64>()
					- ((i + 1)..n).map(|j| a[[i, j]] * x0[j]).sum::<f64>())
		}

		let difference = &a.dot(&x) - &b;
		let residue: f64 = difference.iter().map(|x| x.powf(2.0)).sum();
		if residue < threshold || step > 10 {
			break x;
		}
		x0 = x.clone();
	}
}
