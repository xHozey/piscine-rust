pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rec_space = rectangle_area(x, y);
    let obj_space;
    match kind {
        GeometricalShapes::Circle => {
            obj_space = circle_area(a) as usize;
        }
        GeometricalShapes::Rectangle => {
            obj_space = rectangle_area(a, b);
        }
        GeometricalShapes::Triangle => {
            obj_space = triangle_area(a, b) as usize;
        }
        GeometricalShapes::Square => {
            obj_space = square_area(a);
        }
    }
    times * obj_space <= rec_space
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let box_space = x * y * z;
    let obj_space;
    match kind {
        GeometricalVolumes::Cone => {
            obj_space = cone_volume(a, b) as usize
        }
        GeometricalVolumes::Cube => {
            obj_space = cube_volume(a)
        }
        GeometricalVolumes::Parallelepiped => {
            obj_space = parallelepiped_volume(a, b, c)
        }
        GeometricalVolumes::Sphere => {
            obj_space = sphere_volume(a) as usize
        }
        GeometricalVolumes::TriangularPyramid => {
            obj_space = triangular_pyramid_volume(a as f64, b) as usize
        }
    }
    box_space >= obj_space * times
}