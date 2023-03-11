// [[file:../ui.note::602bbe8e][602bbe8e]]
use gchemol::prelude::*;
use gchemol::{Atom, Molecule};

use three_d::*;
// 602bbe8e ends here

// [[file:../ui.note::*base][base:1]]
type ChemGeomMater = Gm<Mesh, PhysicalMaterial>;
// base:1 ends here

// [[file:../ui.note::373fa44e][373fa44e]]
fn new_atom_sphere(context: &Context, coord: [f32; 3], size: f32, color: Color) -> ChemGeomMater {
    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: color,
                ..Default::default()
            },
        ),
    );
    sphere.set_transformation(Mat4::from_translation(coord.into()) * Mat4::from_scale(size));
    sphere
}
// 373fa44e ends here

// [[file:../ui.note::03701512][03701512]]
fn new_bond_cylinder(
    context: &Context,
    coord1: impl Into<Vec3>,
    coord2: impl Into<Vec3>,
    size: f32,
) -> Gm<Mesh, PhysicalMaterial> {
    let coord1 = coord1.into();
    let coord2 = coord2.into();

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

    let thickness = 0.07;
    let direction = coord2 - coord1;
    let length = direction.magnitude();
    let translation = Mat4::from_translation(coord1);
    let rotation: Mat4 = Quat::from_arc(Vec3::unit_x(), direction.normalize(), None).into();
    let scale = Mat4::from_nonuniform_scale(length, thickness, thickness);
    cylinder.set_transformation(translation * rotation * scale);
    cylinder
}
// 03701512 ends here

// [[file:../ui.note::7c7026e3][7c7026e3]]
fn draw_atoms(context: &Context, mol: &Molecule) -> Vec<ChemGeomMater> {
    mol.atoms().map(|(_, a)| draw_atom(context, a)).collect()
}

fn draw_atom(context: &Context, atom: &Atom) -> ChemGeomMater {
    let coord = atom.position().map(|x| x as f32);
    let size = (atom.get_cov_radius().unwrap_or(0.5) + 0.5) / 3.0;
    let color = atom_color(atom);
    new_atom_sphere(context, coord, size as f32, color)
}
// 7c7026e3 ends here

// [[file:../ui.note::2f286ebd][2f286ebd]]
fn draw_bonds(context: &Context, mol: &Molecule) -> Vec<ChemGeomMater> {
    let mut bonds = vec![];
    for (u, v, b) in mol.bonds() {
        if !b.is_dummy() {
            let au = mol.get_atom_unchecked(u);
            let av = mol.get_atom_unchecked(v);
            let pu: Vec3 = au.position().map(|x| x as f32).into();
            let pv: Vec3 = av.position().map(|x| x as f32).into();
            bonds.push(draw_bond(context, pu, pv));
        }
    }
    bonds
}

fn draw_bond(context: &Context, coord1: Vec3, coord2: Vec3) -> ChemGeomMater {
    new_bond_cylinder(context, coord1, coord2, 1.2)
}
// 2f286ebd ends here

// [[file:../ui.note::b1456f78][b1456f78]]
fn atom_color(atom: &Atom) -> Color {
    match atom.symbol() {
        "C" => Color::new_opaque(144, 144, 144),
        "Si" => Color::new_opaque(94, 94, 94),
        "O" => Color::new_opaque(0, 255, 0),
        "O" => Color::new_opaque(48, 80, 248),
        "H" => Color::new_opaque(255, 255, 255),
        _ => unimplemented!(),
    }
}
// b1456f78 ends here

// [[file:../ui.note::34893a09][34893a09]]
fn draw_molecule(mol: &Molecule) {
    let window = Window::new(WindowSettings {
        title: "GCHEMOL molecule Viewer".to_string(),
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

    let axes = Axes::new(&context, 0.08, 20.0);

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let light0 = DirectionalLight::new(&context, 3.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 3.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));
    // let directional = DirectionalLight::new(&context, 5.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0));

    let bonds = draw_bonds(&context, &mol);
    let atoms = draw_atoms(&context, &mol);
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let objects = axes
            .into_iter()
            .chain(atoms.iter().map(|x| x as &dyn Object))
            .chain(bonds.iter().map(|x| x as &dyn Object));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, objects, &[&ambient, &light0, &light1]);

        FrameOutput::default()
    });
}
// 34893a09 ends here

// [[file:../ui.note::e47921f2][e47921f2]]
pub fn main() {
    // let mut mol = Molecule::from_file("tests/files/MFI.gjf").unwrap();
    let mut mol = Molecule::from_file("tests/files/CH4.gjf").unwrap();
    mol.unbuild_crystal();
    mol.rebond();
    draw_molecule(&mol);
}
// e47921f2 ends here
