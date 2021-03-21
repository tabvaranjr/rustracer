use std::ops::{Add, Div, Mul, Neg, Sub};

pub const EPSILON: f32 = 0.0001;

fn is_approx(a: f32, b: f32, esp: Option<f32>) -> bool {
    (a - b).abs() <= esp.unwrap_or(EPSILON)
}

#[derive(Debug)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn from_vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        // FIXME: having a proper type would be much better.
        assert!(self.is_vector() && rhs.is_vector());

        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: 0.0,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        is_approx(self.x, other.x, None)
            && is_approx(self.y, other.y, None)
            && is_approx(self.z, other.z, None)
            && is_approx(self.w, other.w, None)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    //Scenario: A tuple with w=1.0 is a point
    // Given a ← tuple(4.3, -4.2, 3.1, 1.0)
    //  Then a.x = 4.3
    //   And a.y = -4.2
    //   And a.z = 3.1
    //   And a.w = 1.0
    //   And a is a point
    //   And a is not a vector
    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    //Scenario: A tuple with w=0 is a vector
    // Given a ← tuple(4.3, -4.2, 3.1, 0.0)
    //  Then a.x = 4.3
    //   And a.y = -4.2
    //   And a.z = 3.1
    //   And a.w = 0.0
    //   And a is not a point
    //   And a is a vector
    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

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

    #[test]
    fn is_approx_with_tuples() {
        let t1 = Tuple::new(1.0, -1.0, 2.3, 4.5);
        let t2 = Tuple::new(1.000001, -1.00005, 2.30003, 4.500005);

        assert_eq!(t1, t2);
    }

    // Scenario: point() creates tuples with w=1
    //  Given p ← point(4, -4, 3)
    //   Then p = tuple(4, -4, 3, 1)}
    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple::from_point(4.0, -4.0, 3.0);
        let expected = Tuple::new(4.0, -4.0, 3.0, 1.0);

        assert_eq!(p, expected);
    }

    // Scenario: vector() creates tuples with w=0
    //  Given v ← vector(4, -4, 3)
    //   Then v = tuple(4, -4, 3, 0)
    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v = Tuple::from_vector(4.0, -4.0, 3.0);
        let expected = Tuple::new(4.0, -4.0, 3.0, 0.0);

        assert_eq!(v, expected);
    }

    // Scenario: Adding two tuples
    //  Given a1 ← tuple(3, -2, 5, 1)
    //    And a2 ← tuple(-2, 3, 1, 0)
    //   Then a1 + a2 = tuple(1, 1, 6, 1)
    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_eq!(a1 + a2, expected);
    }

    // Scenario: Subtracting two points
    //  Given p1 ← point(3, 2, 1)
    //    And p2 ← point(5, 6, 7)
    //   Then p1 - p2 = vector(-2, -4, -6)
    #[test]
    fn subtracting_two_points() {
        let a1 = Tuple::from_point(3.0, 2.0, 1.0);
        let a2 = Tuple::from_point(5.0, 6.0, 7.0);
        let expected = Tuple::from_vector(-2.0, -4.0, -6.0);

        assert_eq!(a1 - a2, expected);
    }

    // Scenario: Subtracting a vector from a point
    //  Given p ← point(3, 2, 1)
    //    And v ← vector(5, 6, 7)
    //   Then p - v = point(-2, -4, -6)
    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::from_point(3.0, 2.0, 1.0);
        let v = Tuple::from_vector(5.0, 6.0, 7.0);
        let expected = Tuple::from_point(-2.0, -4.0, -6.0);

        assert_eq!(p - v, expected);
    }

    // Scenario: Subtracting two vectors
    // Given v1 ← vector(3, 2, 1)
    //   And v2 ← vector(5, 6, 7)
    //  Then v1 - v2 = vector(-2, -4, -6)
    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::from_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::from_vector(5.0, 6.0, 7.0);
        let expected = Tuple::from_vector(-2.0, -4.0, -6.0);

        assert_eq!(v1 - v2, expected);
    }

    // Scenario: Subtracting a vector from the zero vector
    //  Given zero ← vector(0, 0, 0)
    //    And v ← vector(1, -2, 3)
    //   Then zero - v = vector(-1, 2, -3)
    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::from_vector(0.0, 0.0, 0.0);
        let v = Tuple::from_vector(1.0, -2.0, 3.0);
        let expected = Tuple::from_vector(-1.0, 2.0, -3.0);

        assert_eq!(zero - v, expected);
    }

    // Scenario: Negating a tuple
    //  Given a ← tuple(1, -2, 3, -4)
    //   Then -a = tuple(-1, 2, -3, 4)
    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);

        assert_eq!(-a, expected);
    }

    // Scenario: Multiplying a tuple by a scalar
    //  Given a ← tuple(1, -2, 3, -4)
    //   Then a * 3.5 = tuple(3.5, -7, 10.5, -14)
    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s: f32 = 3.5;
        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);

        assert_eq!(a * s, expected);
    }

    // Scenario: Multiplying a tuple by a fraction
    //  Given a ← tuple(1, -2, 3, -4)
    //   Then a * 0.5 = tuple(0.5, -1, 1.5, -2)
    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s: f32 = 0.5;
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(a * s, expected);
    }

    // Scenario: Dividing a tuple by a scalar
    //  Given a ← tuple(1, -2, 3, -4)
    //   Then a / 2 = tuple(0.5, -1, 1.5, -2)
    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s: f32 = 2.0;
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);

        assert_eq!(a / s, expected);
    }

    // Scenario: Computing the magnitude of vector(1, 0, 0)
    //  Given v ← vector(1, 0, 0)
    //   Then magnitude(v) = 1
    #[test]
    fn computing_magnitude_of_vector_ex() {
        let v = Tuple::from_vector(1.0, 0.0, 0.0);
        let expected = 1.0;

        assert_eq!(v.magnitude(), expected);
    }

    // Scenario: Computing the magnitude of vector(0, 1, 0)
    //  Given v ← vector(0, 1, 0)
    //   Then magnitude(v) = 1
    #[test]
    fn computing_magnitude_of_vector_ey() {
        let v = Tuple::from_vector(0.0, 1.0, 0.0);
        let expected = 1.0;

        assert_eq!(v.magnitude(), expected);
    }

    // Scenario: Computing the magnitude of vector(0, 0, 1)
    //  Given v ← vector(0, 0, 1)
    //   Then magnitude(v) = 1
    #[test]
    fn computing_magnitude_of_vector_ez() {
        let v = Tuple::from_vector(0.0, 0.0, 1.0);
        let expected = 1.0;

        assert_eq!(v.magnitude(), expected);
    }

    // Scenario: Computing the magnitude of vector(1, 2, 3)
    //  Given v ← vector(1, 2, 3)
    //   Then magnitude(v) = √14
    #[test]
    fn computing_magnitude_of_vector_pos() {
        let v = Tuple::from_vector(1.0, 2.0, 3.0);
        let expected = (14.0f32).sqrt();

        assert_eq!(v.magnitude(), expected);
    }

    // Scenario: Computing the magnitude of vector(-1, -2, -3)
    //  Given v ← vector(-1, -2, -3)
    //   Then magnitude(v) = √14
    #[test]
    fn computing_magnitude_of_vector_neg() {
        let v = Tuple::from_vector(-1.0, -2.0, -3.0);
        let expected = (14.0f32).sqrt();

        assert_eq!(v.magnitude(), expected);
    }

    // Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
    //  Given v ← vector(4, 0, 0)
    //   Then normalize(v) = vector(1, 0, 0)
    #[test]
    fn normalizing_vector_gives_ex() {
        let v = Tuple::from_vector(4.0, 0.0, 0.0);
        let expected = Tuple::from_vector(1.0, 0.0, 0.0);

        assert_eq!(v.normalize(), expected);
    }

    // Scenario: Normalizing vector(1, 2, 3)
    //  Given v ← vector(1, 2, 3)
    //  Then normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)
    #[test]
    fn normalizing_vector() {
        let v = Tuple::from_vector(1.0, 2.0, 3.0);
        let expected = Tuple::from_vector(0.26726, 0.53452, 0.80178);

        assert_eq!(v.normalize(), expected);
    }

    // Scenario: The magnitude of a normalized vector
    //  Given v ← vector(1, 2, 3)
    //   When norm ← normalize(v)
    //   Then magnitude(norm) = 1
    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::from_vector(1.0, 2.0, 3.0);
        let n = v.normalize();
        let expected = 1.0;

        assert!(is_approx(n.magnitude(), expected, None));
    }

    // Scenario: The dot product of two tuples
    //  Given a ← vector(1, 2, 3)
    //    And b ← vector(2, 3, 4)
    //   Then dot(a, b) = 20
    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::from_vector(1.0, 2.0, 3.0);
        let b = Tuple::from_vector(2.0, 3.0, 4.0);
        let expected = 20.0;

        assert_eq!(a.dot(&b), expected);
    }

    // Scenario: The cross product of two vectors
    //  Given a ← vector(1, 2, 3)
    //    And b ← vector(2, 3, 4)
    //   Then cross(a, b) = vector(-1, 2, -1)
    //    And cross(b, a) = vector(1, -2, 1)
    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::from_vector(1.0, 2.0, 3.0);
        let b = Tuple::from_vector(2.0, 3.0, 4.0);
        let expected_ab = Tuple::from_vector(-1.0, 2.0, -1.0);
        let expected_ba = Tuple::from_vector(1.0, -2.0, 1.0);

        assert_eq!(a.cross(&b), expected_ab);
        assert_eq!(b.cross(&a), expected_ba);
    }
}
