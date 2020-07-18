mod graphics;

use graphics::{matrix::transform as tr, processes::pipe_to_magick, Drawer, PPMImg, RGB};

// # compilation:
// cargo run --release

fn main() {
    let mut convert = pipe_to_magick(vec!["ppm:-", "img.gif"]);

    // child should have a stdin, so we directly unwrap
    let mut magick_in = convert.stdin.take().unwrap();

    let mut drawer = Drawer::new(Box::new(PPMImg::new(500, 500, 255)));

    // colors!
    // let default_fg = drawer.get_fg_color();
    let light_yellow = RGB::new(245, 236, 66);
    // let blue = RGB::new(66, 135, 245);
    let magenta = RGB::new(239, 66, 245);
    // let purple = RGB::new(209, 66, 245);
    let brown = RGB::new(212, 143, 78);

    for rot in (0..360).into_iter().step_by(10) {
        // let mut stack: Vec<Matrix> = Vec::<Matrix>::new_stack();

        // moving to the center
        drawer.push_matrix();
        {
            drawer.transform_by(&tr::mv(250., 250., 0.));
            // drawer.transform_by(&tr::rotatex(rot as f64));

            // drawing center sphere, rotate on rot
            drawer.push_matrix();
            {
                drawer.transform_by(
                    &(tr::rotatex(rot as f64) * tr::rotatey(rot as f64) * tr::rotatez(rot as f64)),
                );
                drawer.add_sphere((0., 0., 0.), 40.);
            }
            drawer.pop_matrix();

            // draw the torus around the sphere, rotate on rot
            drawer.push_matrix();
            {
                drawer.transform_by(&(tr::rotatez(45.) * tr::rotatey(rot as f64)));
                drawer.add_torus((0., 0., 0.), 10., 70.);
            }
            drawer.pop_matrix();

            // move away from center, draw first orbit
            drawer.push_matrix();
            {
                // remember: transform_top needs to take the transformation in the opposite direction
                drawer.transform_by(&tr::rotatez(rot as f64)); // <- var here
                drawer.transform_by(&tr::mv(150., 0., 0.));

                drawer.push_matrix();
                {
                    drawer.transform_by(&tr::rotatex(rot as f64));
                    drawer.transform_by(&tr::rotatey(rot as f64));
                    drawer.set_fg_color(magenta);
                    drawer.add_sphere((0., 0., 0.), 30.);
                }
                drawer.pop_matrix();

                // draw 1st satellite
                drawer.push_matrix();
                {
                    drawer.transform_by(&tr::rotatex(rot as f64 * 3.)); // <- var here
                    drawer.transform_by(&tr::mv(0., 80., 0.));
                    drawer.set_fg_color(light_yellow);
                    drawer.add_sphere((0., 0., 0.), 20.);

                    // drawer.transform_by(&);
                    drawer.transform_by(
                        &(tr::rotatey(rot as f64 * 4.)
                            * tr::rotatex(rot as f64 * 4.)
                            * tr::rotatez(-45.)),
                    );
                    drawer.set_fg_color(brown);
                    drawer.add_torus((0., 0., 0.), 5., 40.);
                }
                drawer.pop_matrix();

                // 2nd satellite
                drawer.push_matrix();
                {
                    drawer.transform_by(&tr::rotatex(rot as f64 * 3.));
                    drawer.transform_by(&tr::mv(0., -80., 0.));
                    drawer.add_sphere((0., 0., 0.), 20.);
                }
                drawer.pop_matrix();
            }
            drawer.pop_matrix();

            drawer.push_matrix();
            {
                drawer.transform_by(&tr::rotatez(rot as f64)); // <- var here
                drawer.transform_by(&tr::mv(-200., 0., 0.));

                drawer.add_sphere((0., 0., 0.), 30.);

                drawer.push_matrix();
                {
                    drawer.transform_by(&tr::rotatez(-rot as f64 * 3.));
                    drawer.transform_by(&tr::mv(80., 0., 0.));

                    drawer.add_sphere((0., 0., 0.), 20.);
                }
                drawer.pop_matrix();

                drawer.push_matrix();
                {
                    drawer.transform_by(&tr::rotatez(-rot as f64 * 3.));
                    drawer.transform_by(&tr::mv(-80., 0., 0.));

                    drawer.add_sphere((0., 0., 0.), 20.);
                }
                drawer.pop_matrix();
            }
            drawer.pop_matrix();
        }
        drawer.pop_matrix();

        drawer
            .write_to_buf(&mut magick_in)
            .expect("Error writing img data");

        drawer.clear();
    }

    drop(magick_in);

    println!("Waiting for convert/magick to exit...");
    let output = convert.wait().expect("Failed to wait on convert/magick");
    println!("convert/magick {}", output);
}
