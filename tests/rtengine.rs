#[cfg(test)]
mod rtengine_tests {
    use glam::Vec3A;
    use ndarray::Array2;
    use raytracing::core::rtengine::RTEngine;

    #[test]
    fn basic_instanciation() {
        let width = 300;
        let height = 300;
        let screen = (-1., 1., 1., -1.);
        let mut pixels = Array2::<Vec3A>::default((width, height));

        step_height = (screen.3 - screen.1) / (height as f32);
        step_width = (screen.2 - screen.0) / (width as f32);
        for i in 0..height {
            y = screen.1 + i * step_height;
            for j in 0..width {
                x = screen.0 + j * step_width;
                pixels[[i, j]] = Vec3A::new(x, y, 0.);
            }
        }

        let red_sphere: Sphere = Sphere {
            center: Vec3A::new(-0.2, 0., -1.),
            radius: 0.7,
        };
        let violet_sphere: Sphere = Sphere {
            center: Vec3A::new(0.1, -0.3, 0),
            radius: 0.1,
        };
        let green_sphere: Sphere = Sphere {
            center: Vec3A::new(-0.3, 0., 0.),
            radius: 0.15,
        };
        let plane: Sphere = Sphere {
            center: Vec3A::new(0., -9000., 0),
            radius: 9000. - 0.7,
        };

        let red_material: Material = Material {
            ambiant: Vec3A::new(0.1, 0., 0.),
            diffuse: Vec3A::new(0.7, 0., 0.),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };
        let violet_material: Material = Material {
            ambiant: Vec3A::new(0.1, 0., 0.1),
            diffuse: Vec3A::new(0.7, 0., 0.7),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };
        let green_material: Material = Material {
            ambiant: Vec3A::new(0., 0.1, 0),
            diffuse: Vec3A::new(0., 0.6, 0),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };
        let plane_material: Material = Material {
            ambiant: Vec3A::new(0.1, 0.1, 0.1),
            diffuse: Vec3A::new(0.6, 0.6, 0.6),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };

        let all_objects: Vec<Sphere> = vec![red_sphere, violet_sphere, green_sphere, plane];
        let materials: Vec<Material> = vec![
            red_material,
            violet_material,
            green_material,
            plane_material,
        ];

        let mut rte = RTEngine {
            pos_camera: Vec3A::new(0., 0., 1.),
            pos_pixels: pixels,
            pos_light: Vec3A::new(5., 5., 5.),
            objects: all_objects,
            material: materials,
        };

        assert_eq!(rte.pos_camera, Vec3A::new(0., 0., 1.));
        assert_eq!(rte.pos_pixels.len(), width * height);
    }

    #[test]
    // Size of array of pixels should be the same as the size
    // of pos_pixels inputs after computing the path tracing algorithm.
    //
    // Meaning that if you have a screen of x*y pixels, you have x*y colors.
    // pt = Path tracing
    fn pt_size_of_output() {
        let width = 300;
        let height = 300;
        let screen = (-1., 1., 1., -1.);
        let mut pixels = Array2::<Vec3A>::default((width, height));

        step_height = (screen.3 - screen.1) / (height as f32);
        step_width = (screen.2 - screen.0) / (width as f32);
        for i in 0..height {
            y = screen.1 + i * step_height;
            for j in 0..width {
                x = screen.0 + j * step_width;
                pixels[[i, j]] = Vec3A::new(x, y, 0.);
            }
        }

        let red_sphere: Sphere = Sphere {
            center: Vec3A::new(-0.2, 0., -1.),
            radius: 0.7,
        };
        let violet_sphere: Sphere = Sphere {
            center: Vec3A::new(0.1, -0.3, 0),
            radius: 0.1,
        };
        let green_sphere: Sphere = Sphere {
            center: Vec3A::new(-0.3, 0., 0.),
            radius: 0.15,
        };
        let plane: Sphere = Sphere {
            center: Vec3A::new(0., -9000., 0),
            radius: 9000. - 0.7,
        };

        let red_material: Material = Material {
            ambiant: Vec3A::new(0.1, 0., 0.),
            diffuse: Vec3A::new(0.7, 0., 0.),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };
        let violet_material: Material = Material {
            ambiant: Vec3A::new(0.1, 0., 0.1),
            diffuse: Vec3A::new(0.7, 0., 0.7),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };
        let green_material: Material = Material {
            ambiant: Vec3A::new(0., 0.1, 0),
            diffuse: Vec3A::new(0., 0.6, 0),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };
        let plane_material: Material = Material {
            ambiant: Vec3A::new(0.1, 0.1, 0.1),
            diffuse: Vec3A::new(0.6, 0.6, 0.6),
            specular: Vec3A::new(1., 1., 1.),
            shininess: 100.,
            reflection: 0.5,
        };

        let all_objects: Vec<Sphere> = vec![red_sphere, violet_sphere, green_sphere, plane];
        let materials: Vec<Material> = vec![
            red_material,
            violet_material,
            green_material,
            plane_material,
        ];

        let mut rte = RTEngine {
            pos_camera: Vec3A::new(0., 0., 1.),
            pos_pixels: pixels,
            pos_light: Vec3A::new(5., 5., 5.),
            objects: all_objects,
            material: materials,
        };

        let c = rte.path_tracing();

        assert_eq!(c.len(), rte.pos_pixels.len());
    }
}
