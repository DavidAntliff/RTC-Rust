use derive_more::{Mul, Sub};

#[derive(Debug)]
struct Tuple {
    x: f64, y: f64, z: f64, w: f64,
}

fn dot(a: &Tuple, b: &Tuple) -> f64 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w)
}

macro_rules! tuple_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Tuple;
            fn mul(self, rhs: $rhs) -> Tuple {
                Tuple{x: self.x * rhs, y: self.y * rhs,
                      z: self.z * rhs, w: self.w * rhs}
            }
        }
    }
}

tuple_mul!(Tuple, f64);
tuple_mul!(Tuple, &f64);
tuple_mul!(&Tuple, f64);
tuple_mul!(&Tuple, &f64);


macro_rules! tuple_sub {
    ( $lhs:ty , $rhs:ty ) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = Tuple;
            fn sub(self, rhs: $rhs) -> Tuple {
                Tuple{x: self.x - rhs.x, y: self.y - rhs.y,
                      z: self.z - rhs.z, w: self.w - rhs.w}
            }
        }
    }
}

tuple_sub!(Tuple, Tuple);
tuple_sub!(Tuple, &Tuple);
tuple_sub!(&Tuple, Tuple);
tuple_sub!(&Tuple, &Tuple);


fn reflect(inc: &Tuple, normal: &Tuple) -> Tuple {
    //inc - normal * 2.0 * inc.dot(&normal)
    let a = dot(&inc, &normal);
    let b = 2.0 * a;
    let c = normal * b;
    inc - c
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn something() {
        let v = Tuple { x: 1.0, y: -1.0, z: 0.0, w: 0.0 };
        let n = Tuple { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };

        //let r = reflect(&v, &n);
        let r = v - n * 2.0 * dot(&v, &n);

        println!("{:?}", r);
    }
}
