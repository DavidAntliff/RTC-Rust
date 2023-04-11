use std::ops;

#[derive(Debug)]
struct Tuple {
    x: f64, y: f64, z: f64, w: f64,
}

fn dot(a: &Tuple, b: &Tuple) -> f64 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w)
}

impl_op_ex_commutative!(* |a: Tuple, b: f64| -> Tuple {
    Tuple{x: a.x * b, y: a.y * b, z: a.z * b, w: a.w * b}
});

impl_op_ex!(- |a: Tuple, b: Tuple| -> Tuple {
    Tuple{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z, w: a.w - b.w}
});

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
