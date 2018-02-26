use std::ops::Add;

#[derive(Debug)]
pub struct Vector<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[test]
fn test_vector() {
    let v1 = Vector { x: 1, y: 2, z: 3 };
    let v2 = Vector { x: 2, y: 3, z: 5 };

    println!("v1 + v2 {:?}", v1 + v2);

    assert!(false);
}
