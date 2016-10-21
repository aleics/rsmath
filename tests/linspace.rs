extern crate rsmath as r;

#[cfg(test)]
mod tests {
    use r::linspace::vector::*;
    use r::linspace::point::*;
    use r::linspace::quat::*;

    // --------------- Point3D TEST ----------------------------------------

    #[test]
    fn point_init_test() {
        let p3 = Point3D::<f64>::init();

        assert_eq!(p3.x(), 0.0);
        assert_eq!(p3.y(), 0.0);
        assert_eq!(p3.z(), 0.0);
    }

    #[test]
    fn point_init_with_values_test() {
        let p3 = Point3D::<f64>::init_with_values(1.0, 2.5, -3.0);

        assert_eq!(p3.x(), 1.0);
        assert_eq!(p3.y(), 2.5);
        assert_eq!(p3.z(), -3.0);
    }
    #[test]
    fn point_set_x_test(){
        let mut a = Point3D::<u64>::init_with_values(256, 450, 178);
        a.set_x(200);

        assert_eq!(a.x(), 200);
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn point_set_y_test(){
        let mut a = Point3D::<i64>::init_with_values(-25, 40, -17);
        a.set_y(20);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), 20);
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn point_set_z_test(){
        let mut a = Point3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set_z(2.5);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), 2.5);
    }
    #[test]
    fn point_set_test(){
        let mut a = Point3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set(2.5, 0.1, 1.2);

        assert_eq!(a.x(), 2.5);
        assert_eq!(a.y(), 0.1);
        assert_eq!(a.z(), 1.2);
    }
    #[test]
    fn point_eucl_distance_test() {
        let p1 = Point3D::<i32>::init_with_values(-1, 2, 3);
        let p2 = Point3D::<i32>::init_with_values(0, 5, 8);

        let d = Point3D::<i32>::eucl_distance(&p1, &p2);

        assert_eq!(d, 5.916079783099616); // using online euclidean distance calculator
    }

    // --------------- Vector3D TEST ----------------------------------------

    #[test]
    fn vector_init_test() {
        let v = Vector3D::<f64>::init();

        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }
    #[test]
    fn vector_init_with_values_test() {
        let v = Vector3D::<f64>::init_with_values(2.0, 3.1, 5.2);

        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 3.1);
        assert_eq!(v.z(), 5.2);
    }
    #[test]
    fn vector_scale_test() {
        let mut v3 = Vector3D::<i32>::init_with_values(-1, 2, 3);
        let copy = v3.clone();
        v3.scale(2);

        assert_eq!(v3.x(), copy.x() * 2);
        assert_eq!(v3.y(), copy.y() * 2);
        assert_eq!(v3.z(), copy.z() * 2);
    }
    #[test]
    fn vector_add_test() {
        let a = Vector3D::<i32>::init_with_values(-2, 3, 5);
        let b = Vector3D::<i32>::init_with_values(5, -2, -3);

        let sum = a + b;

        assert_eq!(sum.x(), a.x() + b.x());
        assert_eq!(sum.y(), a.y() + b.y());
        assert_eq!(sum.z(), a.z() + b.z());
    }
    #[test]
    fn vector_sub_test() {
        let a = Vector3D::<f32>::init_with_values(0.2, 10.1, -2.5);
        let b = Vector3D::<f32>::init_with_values(5.7, -2.1, -3.0);

        let sub = a - b;

        assert_eq!(sub.x(), a.x() - b.x());
        assert_eq!(sub.y(), a.y() - b.y());
        assert_eq!(sub.z(), a.z() - b.z());
    }
    #[test]
    fn vector_mul_test() {
        let a = Vector3D::<u32>::init_with_values(0, 10, 5);
        let b = Vector3D::<u32>::init_with_values(7, 2, 0);

        let mul = a * b;

        assert_eq!(mul.x(), a.x() * b.x());
        assert_eq!(mul.y(), a.y() * b.y());
        assert_eq!(mul.z(), a.z() * b.z());
    }
    #[test]
    fn vector_set_x_test(){
        let mut a = Vector3D::<u64>::init_with_values(256, 450, 178);
        a.set_x(200);

        assert_eq!(a.x(), 200);
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn vector_set_y_test(){
        let mut a = Vector3D::<i64>::init_with_values(-25, 40, -17);
        a.set_y(20);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), 20);
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn vector_set_z_test(){
        let mut a = Vector3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set_z(2.5);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), 2.5);
    }
    #[test]
    fn vector_set_test(){
        let mut a = Vector3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set(2.5, 0.1, 1.2);

        assert_eq!(a.x(), 2.5);
        assert_eq!(a.y(), 0.1);
        assert_eq!(a.z(), 1.2);
    }
    #[test]
    fn vector_cross_test() {
        let a = Vector3D::<i32>::init_with_values(-1, 2, 3);
        let b = Vector3D::<i32>::init_with_values(0, 2, 5);

        let c = Vector3D::<i32>::cross(&a, &b);

        let res = Vector3D::<i32>::init_with_values(4, 5, -2); // online cross vector calculator

        assert_eq!(res == c, true);
    }
    #[test]
    fn vector_dot_test() {
        let a = Vector3D::<i64>::init_with_values(1, 0, 2);
        let b = Vector3D::<i64>::init_with_values(0, 2, 1);

        assert_eq!(a.dot(&b), 1*0 + 0*2 + 2*1);
    }
    #[test]
    fn vector_norm_test() {
        let mut a = Vector3D::<f64>::init_with_values(2f64, 1f64, 3f64);
        a.norm();

        // values compared using wolframalpha.com
        assert_eq!(a.x(), 0.5345224838248488f64);
        assert_eq!(a.y(), 0.2672612419124244f64);
        assert_eq!(a.z(), 0.8017837257372732f64);
    }

     // --------------- QUAT TEST ----------------------------------------
    #[test]
    fn quat_coord_test() {
        let q: Quat<f64> = Quat::<f64>::init_with_values(2f64, 1.5f64, -0.2f64, 3f64);

        assert_eq!(q.x(), 2f64);
        assert_eq!(q.y(), 1.5f64);
        assert_eq!(q.z(), -0.2f64);
        assert_eq!(q.w(), 3f64);
    }
    #[test]
    fn quat_set_x_test() {
        let mut q: Quat<u64> = Quat::<u64>::init_with_values(2u64, 15u64, 2u64, 3u64);
        let new_x: u64 = 3u64;
        q.set_x(new_x);
        assert_eq!(q.x(), new_x);
    }
    #[test]
    fn quat_set_y_test() {
        let mut q: Quat<u64> = Quat::<u64>::init_with_values(2u64, 15u64, 2u64, 3u64);
        let new_y: u64 = 3u64;
        q.set_y(new_y);

        assert_eq!(q.y(), new_y);
    }
    #[test]
    fn quat_set_z_test() {
        let mut q: Quat<u64> = Quat::<u64>::init_with_values(2u64, 15u64, 2u64, 3u64);
        let new_z: u64 = 3u64;
        q.set_z(new_z);

        assert_eq!(q.z(), new_z);
    }
    #[test]
    fn quat_set_w_test() {
        let mut q: Quat<u64> = Quat::<u64>::init_with_values(2u64, 15u64, 2u64, 2u64);
        let new_w: u64 = 3u64;
        q.set_w(new_w);

        assert_eq!(q.w(), new_w);
    }
    #[test]
    fn quat_set_test() {
        let mut q: Quat<i64> = Quat::<i64>::init_with_values(2i64, -15i64, 0i64, -3i64);
        
        let new_x: i64 = 0i64;
        let new_y: i64 = -2i64;
        let new_z: i64 = -1i64;
        let new_w: i64 = 3i64;
        q.set(new_x, new_y, new_z, new_w);

        assert_eq!(q.x(), new_x);
        assert_eq!(q.y(), new_y);
        assert_eq!(q.z(), new_z);
        assert_eq!(q.w(), new_w);
    }
    #[test]
    fn quat_scale_test() {
        let mut q: Quat<f64> = Quat::<f64>::init_with_values(2f64, 1.5f64, -0.2f64, 3f64);
        q.scale(2f64);

        assert_eq!(q.x(), 4f64);
        assert_eq!(q.y(), 3f64);
        assert_eq!(q.z(), -0.4f64);
        assert_eq!(q.w(), 6f64);
    }
    #[test]
    fn quat_add_test() {
        let q: Quat<i32> = Quat::<i32>::init_with_values(2i32, 1i32, -2i32, 2i32);
        let p: Quat<i32> = Quat::<i32>::init_with_values(3i32, 0i32, 2i32, -1i32);

        let add: Quat<i32> = p + q;

        assert_eq!(add.x(), 5i32);
        assert_eq!(add.y(), 1i32);
        assert_eq!(add.z(), 0i32);
        assert_eq!(add.w(), 1i32);
    }
    #[test]
    fn quat_sub_test() {
        let q: Quat<i32> = Quat::<i32>::init_with_values(2i32, 1i32, -2i32, 2i32);
        let p: Quat<i32> = Quat::<i32>::init_with_values(3i32, 0i32, 2i32, -1i32);

        let sub: Quat<i32> = p - q;

        assert_eq!(sub.x(), 1i32);
        assert_eq!(sub.y(), -1i32);
        assert_eq!(sub.z(), 4i32);
        assert_eq!(sub.w(), -3i32);
    }
    #[test]
    fn quat_neg_test() {
        let q: Quat<f64> = Quat::<f64>::init_with_values(2.3f64, 1f64, -0.2f64, 2f64);
        let neg = -q;

        assert_eq!(neg.x(), -2.3f64);
        assert_eq!(neg.y(), -1f64);
        assert_eq!(neg.z(), 0.2f64);
        assert_eq!(neg.w(), -2f64);
    }
}
