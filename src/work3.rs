use matrix::Matrix;
use render;
use ppm;
use consts::*;

pub fn run() {
    test_matrix();
    test_edge_list();
}

// [Work 3] Print results of some Matrix computations
pub fn test_matrix() {
    println!("GENERAL MATRIX MATH:\n");
    let m = Matrix::new4x4(
        1.0, 2.0, 3.0, 0.0,
        4.0, 5.0, 6.0, 0.0,
        7.0, 8.0, 9.0, 0.0,
        0.0, 0.0, 0.0, 1.0);
    println!("{}", m);
    println!("{}", Matrix::identity());
    println!("{}", &m * &Matrix::identity());
    println!("{}", &Matrix::identity() * &m);
    let dilate = Matrix::dilation_xyz(1.0, 3.0, 5.0);
    println!("{}", dilate);
    println!("{}", &dilate * &m);
    println!("{}", &dilate * &Matrix::identity());
    println!("============================");
}

// [Work 3] Draw same image from work 2, using Matrix for an edge list.
pub fn test_edge_list() {
    println!("EDGE MATRIX:\n");
    let mut edges = Matrix::empty();
    for i in 0..(HEIGHT / 20) {
        // down-right lines
        edges.push_edge(
            [0.0, i as f64, 0.0, 1.0],
            [(WIDTH - 1) as f64, (i * 19) as f64, 0.0, 1.0]);
        // down-left lines
        edges.push_edge(
            [(WIDTH - 1) as f64, i as f64, 0.0, 1.0],
            [0.0, (i * 19) as f64, 0.0, 1.0]);
        // up-right lines
        edges.push_edge(
            [0.0, (HEIGHT - 1 - i) as f64, 0.0, 1.0],
            [(WIDTH - 1) as f64, (HEIGHT - 1 - i * 19) as f64, 0.0, 1.0]);
        // up-left lines
        edges.push_edge(
            [(WIDTH - 1) as f64, (HEIGHT - 1 - i) as f64, 0.0, 1.0],
            [0.0, (HEIGHT - 1 - i * 19) as f64, 0.0, 1.0]);
    }
    edges = &Matrix::dilation(0.5) * &edges;
    edges = &Matrix::shear_2d(0.3, 0.8) * &edges;
    edges = &Matrix::rotation_about_z(0.1) * &edges;
    ppm::make_ppm(|image| {
        render::edge_list(image, edges);
    });
    println!("Saved transformed image to img.ppm");
}
