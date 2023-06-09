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

// [[file:../ui.note::30bab14c][30bab14c]]
/// Calculate translation and scale transformations for picked atom
fn transformation_for_picked_atom(atoms_aabb: &[AxisAlignedBoundingBox], pick: Vec3) -> Mat4 {
    // find picked atom center
    let mut distances: Vec<_> = atoms_aabb
        .iter()
        .enumerate()
        .map(|(i, aabb)| ((pick - aabb.center()).magnitude(), i))
        .collect();
    distances.sort_by_key(|&(d, i)| (d as f64).as_ordered_float());
    let (_, i) = distances[0];
    let picked = atoms_aabb[i];
    // ideal radius = size / 2 / sqrt(3)
    Mat4::from_translation(picked.center()) * Mat4::from_scale(picked.size().magnitude() / 2.0 / 1.5)
}
// 30bab14c ends here

// [[file:../ui.note::34893a09][34893a09]]
use vecfx::*;

fn draw_molecule(mol: &Molecule) {
    let window = Window::new(WindowSettings {
        title: "GCHEMOL molecule Viewer".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.0, 4.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let axes = Axes::new(&context, 0.08, 20.0);

    let mut sphere = CpuMesh::sphere(8);
    // sphere.transform(&Mat4::from_scale(0.4)).unwrap();

    let mut pick_mesh = Gm::new(
        Mesh::new(&context, &sphere),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Color::new(0, 255, 0, 100),
                ..Default::default()
            },
        ),
    );

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let light0 = DirectionalLight::new(&context, 3.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 3.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    let bonds = draw_bonds(&context, &mol);
    let atoms = draw_atoms(&context, &mol);
    let centers: Vec<_> = atoms.iter().map(|s| s.aabb()).collect();
    window.render_loop(move |mut frame_input| {
        let mut change = frame_input.first_frame;
        change |= camera.set_viewport(frame_input.viewport);

        let objects = axes
            .into_iter()
            .chain(atoms.iter().map(|x| x as &dyn Object))
            .chain(bonds.iter().map(|x| x as &dyn Object));

        for event in frame_input.events.iter() {
            match event {
                Event::MousePress { button, position, .. } => {
                    if *button == MouseButton::Left {
                        let pixel = (
                            (frame_input.device_pixel_ratio * position.0) as f32,
                            (frame_input.viewport.height as f64 - frame_input.device_pixel_ratio * position.1) as f32,
                        );

                        if let Some(pick) = pick(&context, &camera, pixel, &atoms) {
                            // set effect for picked atom
                            let trans = transformation_for_picked_atom(&centers, pick);
                            pick_mesh.set_transformation(trans);
                            change = true;
                        }
                    }
                }
                _ => {}
            }
        }

        change |= control.handle_events(&mut camera, &mut frame_input.events);
        // draw
        if change {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(&camera, objects.chain(&pick_mesh), &[&ambient, &light0, &light1]);
        } else {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(&camera, objects, &[&ambient, &light0, &light1]);
        }

        FrameOutput {
            swap_buffers: change,
            ..Default::default()
        }
    });
}
// 34893a09 ends here

// [[file:../ui.note::e47921f2][e47921f2]]
pub fn main() {
    // let mut mol = Molecule::from_file("tests/files/MFI.gjf").unwrap();
    let mut mol = Molecule::from_file("tests/files/CH4.gjf").unwrap();
    mol.recenter();
    mol.unbuild_crystal();
    mol.rebond();
    draw_molecule(&mol);
}
// e47921f2 ends here
