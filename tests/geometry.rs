#[cfg(test)]
mod vec3a_tests{
    // use raytracing::utils::geometry::Vec3A;
    use glam::Vec3A;

    #[test]
    fn default_instanciation() {
        let p = Vec3A::default();

        assert_eq!(p.x, 0.0f32);
        assert_eq!(p.y, 0.0f32);
        assert_eq!(p.z, 0.0f32);
    }

    #[test]
    fn xyz_instanciation() {
        let x = -3.5;
        let y = -6.5;
        let z = 4.0;
        let p = Vec3A::new(x, y, z);

        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
        assert_eq!(p.z, z);
    }

    #[test]
    fn simple_add() {
        let offset = 10f32;

        let p1 = Vec3A::default();
        let p2 = Vec3A::new(offset, offset, offset);

        let p = p1 + p2;

        assert_eq!(p.x, p1.x + p2.x);
        assert_eq!(p.y, p1.y + p2.y);
        assert_eq!(p.z, p1.z + p2.z);
    }

    #[test]
    fn simple_eq() {
        let v = 10f32;

        let p1 = Vec3A::new(v, v, v);
        let p2 = Vec3A::new(v, v, v);

        assert_eq!(p1, p2);
    }

    #[test]
    fn various_operator() {
        let mut p1 = Vec3A::default();
        let mut sum : f32 = 0f32;

        for i in 0..100 {
            let n = i as f32; 
            p1.x += n;
            p1.y += n;
            p1.z += n;

            sum += n;
        }

        let p2 = Vec3A::new(sum, sum, sum);
        assert_eq!(p1, p2);
        assert_eq!(p1 + p2, p1 + p1);
    }
}
