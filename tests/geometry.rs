#[cfg(test)]
mod point_tests{
    use raytracing::utils::geometry::Point;

    #[test]
    fn default_instanciation() {
        let p = Point{..Default::default()};

        assert_eq!(p.x, 0.0f32);
        assert_eq!(p.y, 0.0f32);
        assert_eq!(p.z, 0.0f32);
    }

    #[test]
    fn simple_add() {
        let offset = 10f32;

        let p1 = Point{..Default::default()};
        let p2 = Point{x : offset, y : offset, z : offset};

        let p = p1 + p2;

        assert_eq!(p.x, p1.x + p2.x);
        assert_eq!(p.y, p1.y + p2.y);
        assert_eq!(p.z, p1.z + p2.z);
    }

    #[test]
    fn simple_eq() {
        let v = 10f32;

        let p1 = Point{x : v, y : v, z : v};
        let p2 = Point{x : v, y : v, z : v};

        assert_eq!(p1, p2);
    }

    #[test]
    fn various_operator() {
        let mut p1 = Point{..Default::default()};
        let mut sum : f32 = 0f32;

        for i in 0..100 {
            let n = i as f32; 
            p1.x += n;
            p1.y += n;
            p1.z += n;

            sum += n;
        }

        let p2 = Point{ x : sum, y : sum, z : sum};
        assert_eq!(p1, p2);
        assert_eq!(p1 + p2, p1 + p1);
    }
}
