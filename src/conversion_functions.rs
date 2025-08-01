use crate::geometry_functions::{construct_buffered_bounding_box, triangulate};
use crate::write_functions::write_obj_file;
use ecitygml::operations::GeometryCollector;
use ecitygml_core::model::building::Building;
use ecitygml_core::model::common::{CityObjectClass, LevelOfDetail};
use ecitygml_core::operations::{FeatureWithGeometry, Visitable};
use egml::model::base::Id;
use egml::model::geometry::{MultiSurface, Polygon};
use rayon::prelude::*;

pub fn collect_building_geometries(
    input_building: &mut Building,
    tbw: bool,
    add_bb: bool,
    add_json: bool,
    import_bb: bool,
) {
    // Initialize an empty bounding box
    let mut bbox = (Vec::new(), Vec::new());

    // Distinguish the different cases of the bounding box
    if add_bb {
        // Calculate the bounding box and add little pyramids in the corners
        bbox = construct_buffered_bounding_box(input_building);
    } else if import_bb {
        // Import the bounding box from an external file
        // todo: Muss noch implementiert werden.
    }

    let building_id = &input_building.city_object.gml.id;

    // get the translation parameter into a local crs in case it is desired
    let mut dx: f64 = 0.0;
    let mut dy: f64 = 0.0;
    let mut dz: f64 = 0.0;
    if tbw {
        // Calculate the envelope of the building and get the tranformation parameters from it
        if let Some(envelope) = input_building.envelope() {
            let upper_corner = envelope.upper_corner();
            let lower_corner = envelope.lower_corner();
            dx = -((upper_corner.x() + lower_corner.x()) / 2.0);
            dy = -((upper_corner.y() + lower_corner.y()) / 2.0);
            dz = -((upper_corner.z() + lower_corner.z()) / 2.0);
        } else {
            println!("Envelope hat keine gültigen lower/upper corner Koordinaten.");
        }
    }

    let mut collector_1 = GeometryCollector::new();
    input_building.accept(&mut collector_1);

    // Assuming collector_1.city_objects is a Vec or similar iterable
    collector_1
        .city_objects
        .par_iter()
        .for_each(|collected_geometry| {
            // Obtain all the different data
            let gml_id = &collected_geometry.1.gml.id;
            let class = collected_geometry.1.class;
            let implicit_geometries = &collected_geometry.1.implicit_geometries;
            let multi_surfaces = &collected_geometry.1.multi_surfaces;
            let solids = &collected_geometry.1.solids;

            // Process the MultiSurfaces
            for multi_surface in multi_surfaces {
                process_multi_surface(&multi_surface, building_id, class, dx, dy, dz, &bbox);
            }
        });
}

pub fn process_multi_surface(
    input_multi_surface: &(&LevelOfDetail, &MultiSurface),
    building_id: &Id,
    class: CityObjectClass,
    dx: f64,
    dy: f64,
    dz: f64,
    bbox: &(Vec<[f64; 3]>, Vec<[u64; 3]>),
) {
    let stuffs = &input_multi_surface.1.surface_member();
    let stuff_gml_id = &input_multi_surface.1.gml.id;
    stuffs.par_iter().for_each(|surface_member| {
        process_surface_member(
            surface_member,
            building_id,
            stuff_gml_id,
            class,
            false,
            dx,
            dy,
            dz,
            bbox,
        );
    });

}
pub fn process_surface_member(
    input_surface_member: &Polygon,
    building_id: &Id,
    multi_surface_id: &Id,
    thematic_info: CityObjectClass,
    process_with_poly_id: bool,
    dx: f64,
    dy: f64,
    dz: f64,
    bbox: &(Vec<[f64; 3]>, Vec<[u64; 3]>),
) {
    // todo: implement a way to get the thematic info in form of a string
    let thematic_info_string = "tbd";
    let (triangles, all_points) = triangulate(input_surface_member);
    let input_surface_member_id = &input_surface_member.gml.id;
    if process_with_poly_id {
        write_obj_file(
            all_points,
            triangles,
            building_id,
            input_surface_member_id,
            thematic_info_string,
            dx,
            dy,
            dz,
            bbox,
        );

    // Calculate the bounding box and add little pyramids in the corners
    } else {
        write_obj_file(
            all_points,
            triangles,
            building_id,
            input_surface_member_id,
            &thematic_info_string,
            dx,
            dy,
            dz,
            bbox,
        );
    }
}
