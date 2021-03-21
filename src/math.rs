pub const EPSILON: f32 = 0.0001;

pub fn is_approx(a: f32, b: f32, esp: Option<f32>) -> bool {
    (a - b).abs() <= esp.unwrap_or(EPSILON)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn is_approx_inside_epsilon() {
        let a = 1.0;
        let b = 1.00001;

        assert_eq!(is_approx(a, b, None), true);
    }

    #[test]
    fn is_approx_outside_epsilon() {
        let a = 1.0;
        let b = 1.01;

        assert_eq!(is_approx(a, b, None), false);
    }
}