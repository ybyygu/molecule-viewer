// [[file:../ui.note::602bbe8e][602bbe8e]]
use gchemol::prelude::*;
use gchemol::{Atom, Molecule};

use three_d::*;
// 602bbe8e ends here

// [[file:../ui.note::*base][base:1]]
type ChemGeomMater = Gm<Mesh, PhysicalMaterial>;
// base:1 ends here

// [[file:../ui.note::373fa44e][373fa44e]]
fn new_atom_sphere(context: &Context, coord: [f32; 3], size: f32) -> ChemGeomMater {
    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Color::new_opaque(255, 0, 0),
                ..Default::default()
            },
        ),
    );
    sphere.set_transformation(Mat4::from_translation(coord.into()) * Mat4::from_scale(size));
    sphere
}
// 373fa44e ends here

// [[file:../ui.note::03701512][03701512]]
fn new_bond_cylinder(context: &Context, coord: [f32; 3], size: f32) -> Gm<Mesh, PhysicalMaterial> {
    let mut cylinder = Gm::new(
        Mesh::new(&context, &CpuMesh::cylinder(16)),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Color::new_opaque(0, 255, 0),
                ..Default::default()
            },
        ),
    );
    cylinder.set_transformation(Mat4::from_translation(coord.into()) * Mat4::from_nonuniform_scale(size, 0.07, 0.07));
    cylinder
}
// 03701512 ends here

// [[file:../ui.note::7c7026e3][7c7026e3]]
fn draw_atoms(context: &Context) -> Vec<ChemGeomMater> {
    let f = "tests/files/CH4.gjf";
    let mol = Molecule::from_file(f).unwrap();
    mol.atoms().map(|(_, a)| draw_atom(context, a)).collect()
}

fn draw_atom(context: &Context, atom: &Atom) -> ChemGeomMater {
    let coord = atom.position().map(|x| x as f32);
    new_atom_sphere(context, coord, 0.6)
}
// 7c7026e3 ends here

// [[file:../ui.note::e47921f2][e47921f2]]
pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_orthographic(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        2.5,
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let mut sphere = new_atom_sphere(&context, [0.0, 1.3, 0.0], 0.5);
    let mut cylinder = new_bond_cylinder(&context, [1.0, 0.0, 0.0], 0.1);

    let axes = Axes::new(&context, 0.08, 20.0);

    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    let atoms = draw_atoms(&context);
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let cylinder = new_bond_cylinder(&context, [0.2; 3], 2.0);
        let objects = axes
            .into_iter()
            .chain(atoms.iter().map(|x| x as &dyn Object))
            .chain(&cylinder);

        // let objects = sphere
        //     .into_iter() //
        //     // .chain(&cylinder)
        //     .chain(&axes);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, objects, &[&light0, &light1]);

        FrameOutput::default()
    });
}
// e47921f2 ends here
