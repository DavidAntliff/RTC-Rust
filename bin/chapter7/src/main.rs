use rust_rtc::world::world;
use rust_rtc::spheres::sphere;
use rust_rtc::materials::default_material;
use rust_rtc::colors::color;
use rust_rtc::transformations::scaling;
use rust_rtc::lights::point_light;
use rust_rtc::tuples::point;

fn main() {

    let mut w = world();
    let mut floor = sphere(1);
    floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material = default_material();
    floor.material.color = color(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
    w.add_object(floor);

    /*

        let left_wall = sphere(2);
        left_wall.set_transform(translation(0.0, 0.0, 5.0) *
        rotation_y(-pi / 4.0) *
        rotation_x(pi / 2.0) *
        scaling(10.0, 0.01, 10.0));
        left_wall.material() = floor.material();

        let right_wall = sphere(3);
        right_wall.set_transform(translation(0.0, 0.0, 5.0) *
        rotation_y(pi / 4.0) *
        rotation_x(pi / 2.0) *
        scaling(10.0, 0.01, 10.0));
        right_wall.material() = floor.material();

        let middle = sphere(4);
        middle.set_transform(translation(-0.5, 1.0, 0.5));
        middle.material() = material();
        middle.material().set_color(color(0.1, 1.0, 0.5));
        middle.material().set_diffuse(0.7);
        middle.material().set_specular(0.3);

        let right = sphere(5);
        right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
        right.material() = material();
        right.material().set_color(color(0.5, 1.0, 0.1));
        right.material().set_diffuse(0.7);
        right.material().set_specular(0.3);

        let left = sphere(6);
        left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
        left.material() = material();
        left.material().set_color(color(1.0, 0.8, 0.1));
        left.material().set_diffuse(0.7);
        left.material().set_specular(0.3);

        w.add_object(left_wall);
        w.add_object(right_wall);
        w.add_object(middle);
        w.add_object(right);
        w.add_object(left);
     */
        w.add_light(point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0)));

    /*
        let cam = camera(100, 50, pi / 3.0);
        //let cam = camera(1600, 800, pi / 3.0);
        cam.set_transform(view_transform(point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0)));

        let canvas = render(cam, w);

        let ppm = ppm_from_canvas(&canvas);
        println!("{}", ppm);

     */
}
