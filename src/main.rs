mod graphics;

use graphics::{
    canvas::Canvas,
    matrix::{mstack::MStack, transform as tr},
    processes::pipe_to_magick,
    // parser::DWScript,
    // utils::display_ppm,
    Matrix,
    PPMImg, RGB,
};

// # compilation:
// cargo run --release



fn main() {
    let mut convert = pipe_to_magick(vec!["ppm:-", "img.png"]);

    // child should have a stdin, so we directly unwrap
    let mut magick_in = convert.stdin.take().unwrap();
    let mut img = PPMImg::new_with_bg(500, 500, 255, RGB::new(253, 255, 186));
    let mut polygons = Matrix::new_polygon_matrix();

    #[allow(dead_code)]
    // colors!
    // let default_fg = img.fg_color;
    // let light_yellow = RGB::new(245, 236, 66);
    // let blue = RGB::new(66, 135, 245);
    // let magenta = RGB::new(239, 66, 245);
    // let purple = RGB::new(209, 66, 245);
    // let brown = RGB::new(212, 143, 78);

    for rot in (0..360).into_iter().step_by(120) {
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
            polygons.add_sphere((0., 0., 0.), 200.);
            img.render_polygon_with_stack(&stack, &polygons);
            polygons.clear();
        }

        // // draw the torus around the sphere, rotate on rot
        // stack.push_matrix();
        // {
        //     stack.transform_top(&(tr::rotatez(45.) * tr::rotatey(rot as f64)));
        //     polygons.add_torus((0., 0., 0.), 10., 70.);
        //     img.render_polygon_with_stack(&stack, &polygons);
        //     polygons.clear();
        // }
        // stack.pop_matrix();

        println!("Iteration at {}", rot);

        img.write_bin_to_buf(&mut magick_in)
            .expect("Error writing img data");

        // sets everything to bg_color, clear zbuf
        img.clear();
    }

    drop(magick_in);

    println!("Waiting for convert/magick to exit...");
    let output = convert.wait().expect("Failed to wait on convert/magick");
    println!("convert/magick {}", output);
}
