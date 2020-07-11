mod graphics;

use graphics::{
    canvas::Canvas,
    matrix::{mstack::MStack, transform as tr},
    processes::pipe_to_magick,
    // parser::DWScript,
    // utils::display_ppm,
    Matrix,
    PPMImg,
    RGB,
};

// # compilation:
// cargo run --release

fn main() {
    let mut convert = pipe_to_magick(vec!["ppm:-", "img.gif"]);

    // child should have a stdin, so we directly unwrap
    let mut magick_in = convert.stdin.take().unwrap();
    let mut img = PPMImg::new(500, 500, 255);
    let mut polygons = Matrix::new_polygon_matrix();

    // colors!
    let default_fg = img.fg_color;
    let light_yellow = RGB::new(245, 236, 66);
    let blue = RGB::new(66, 135, 245);
    let magenta = RGB::new(239, 66, 245);
    let purple = RGB::new(209, 66, 245);
    let brown = RGB::new(212, 143, 78);

    for rot in (0..360).into_iter().step_by(10) {
        let mut stack: Vec<Matrix> = Vec::<Matrix>::new_stack();

        // moving to the center
        stack.push_matrix();
        stack.transform_top(&tr::mv(250., 250., 0.));
        // stack.transform_top(&tr::rotatex(rot as f64));

        // drawing center sphere, rotate on rot
        stack.push_matrix();
        {
            stack.transform_top(
                &(tr::rotatex(rot as f64) * tr::rotatey(rot as f64) * tr::rotatez(rot as f64)),
            );
            polygons.add_sphere((0., 0., 0.), 40.);
            img.render_polygon_with_stack(&stack, &polygons);
            polygons.clear();
        }
        stack.pop_matrix();

        // draw the torus around the sphere, rotate on rot
        stack.push_matrix();
        {
            stack.transform_top(&(tr::rotatez(45.) * tr::rotatey(rot as f64)));
            polygons.add_torus((0., 0., 0.), 10., 70.);
            img.render_polygon_with_stack(&stack, &polygons);
            polygons.clear();
        }
        stack.pop_matrix();

        // move away from center, draw first orbit
        stack.push_matrix();
        {
            // remember: transform_top needs to take the transformation in the opposite direction
            stack.transform_top(&tr::rotatez(rot as f64)); // <- var here
            stack.transform_top(&tr::mv(150., 0., 0.));

            polygons.add_sphere((0., 0., 0.), 30.);
            polygons *= tr::rotatex(rot as f64) * tr::rotatey(rot as f64);
            img.set_fg_color(magenta);
            img.render_polygon_with_stack(&stack, &polygons);
            polygons.clear();

            stack.push_matrix();
            {
                stack.transform_top(&tr::rotatex(rot as f64 * 3.)); // <- var here
                stack.transform_top(&tr::mv(0., 80., 0.));

                polygons.add_sphere((0., 0., 0.), 20.);
                img.set_fg_color(light_yellow);
                img.render_polygon_with_stack(&stack, &polygons);
                polygons.clear();

                polygons.add_torus((0.,0.,0.,), 5., 40.);
                polygons *= tr::rotatez(-45.) * tr::rotatey(rot as f64 * 4.);
                img.set_fg_color(brown);
                img.render_polygon_with_stack(&stack, &polygons);
                polygons.clear();
            }
            stack.pop_matrix();

            stack.push_matrix();
            {
                stack.transform_top(&tr::rotatex(rot as f64 * 3.));
                stack.transform_top(&tr::mv(0., -80., 0.));
                polygons.add_sphere((0., 0., 0.), 20.);
                img.set_fg_color(light_yellow);
                img.render_polygon_with_stack(&stack, &polygons);
                img.set_fg_color(default_fg);
                polygons.clear();
            }
            stack.pop_matrix();
        }
        stack.pop_matrix();

        stack.push_matrix();
        {
            stack.transform_top(&tr::rotatez(rot as f64)); // <- var here
            stack.transform_top(&tr::mv(-200., 0., 0.));

            polygons.add_sphere((0., 0., 0.), 30.);
            img.set_fg_color(blue);
            img.render_polygon_with_stack(&stack, &polygons);
            polygons.clear();


            stack.push_matrix();
            {
                stack.transform_top(&tr::rotatez(-rot as f64 * 3.));
                stack.transform_top(&tr::mv(80., 0., 0.));

                polygons.add_sphere((0., 0., 0.), 20.);
                img.set_fg_color(purple);
                img.render_polygon_with_stack(&stack, &polygons);
                polygons.clear();
            }
            stack.pop_matrix();

            stack.push_matrix();
            {
                stack.transform_top(&tr::rotatez(-rot as f64 * 3.));
                stack.transform_top(&tr::mv(-80., 0., 0.));

                polygons.add_sphere((0., 0., 0.), 20.);
                img.render_polygon_with_stack(&stack, &polygons);
                img.set_fg_color(default_fg);
                polygons.clear();
            }
            stack.pop_matrix();
        }
        stack.pop_matrix();

        img.write_bin_to_buf(&mut magick_in)
            .expect("Error writing img data");

        img.clear();
    }

    drop(magick_in);

    println!("Waiting for convert/magick to exit...");
    let output = convert.wait().expect("Failed to wait on convert/magick");
    println!("convert/magick {}", output);
}
