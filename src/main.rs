use core::panic;
use std::fs::File;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

struct MapSize {
    width: u32,
    height: u32,
}

type Wall = (u32, u32, u32, Orientation);

impl MapSize {
    fn new(width: u32, height: u32) -> MapSize {
        MapSize { width, height }
    }

    fn walls(&self) -> Vec<Wall> {
        vec![
            (0, 0, self.width, Orientation::Left),
            (0, 0, self.height, Orientation::Down),
            (0, self.height, self.width, Orientation::Left),
            (self.width, 0, self.height, Orientation::Down),
        ]
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn get_random() -> Orientation {
        Orientation::Right
    }
}

#[derive(Clone, Debug)]
struct MapObject {
    x: u32,
    y: u32,
    orientation: Orientation,
    prefab: Prefab,
}

impl MapObject {
    fn new(x: u32, y: u32, orientation: Orientation, prefab: &Prefab) -> MapObject {
        MapObject {
            x,
            y,
            orientation,
            prefab: prefab.clone(),
        }
    }
}

struct Map {
    walls: [Vec<MapObject>; 4],
    inside_objects: Vec<MapObject>,
}

#[derive(Clone, Debug, Deserialize)]
struct Prefab {
    width: u32,
    height: u32,
    render: char,
}

fn generate_walls(
    rng: &mut ChaCha8Rng,
    map_size: &MapSize,
    map_orientation: &Orientation,
    prefabs_path: String,
) -> [Vec<MapObject>; 4] {
    let prefab_io = File::open(prefabs_path).unwrap();
    let prefabs: Vec<Prefab> = serde_yaml::from_reader(prefab_io).unwrap();
    println!("loaded {} prefabs", prefabs.len());

    let mut result: [Vec<MapObject>; 4] = Default::default();
    for (indx, wall) in map_size.walls().iter().enumerate() {
        result[indx] = generate_wall(rng, wall, &prefabs);
        println!(
            "generated wall #{} by using {} prefabs, wall {:?}",
            indx,
            result[indx].len(),
            result[indx]
        );
    }

    // TODO: Join the corners by adding corner-tiles (replacing the ends of the walls)

    // TODO: Generate entrace and exit by using map orientation (by forcing a exit prefab somewhere)

    result
}

fn generate_wall(rng: &mut ChaCha8Rng, parameters: &Wall, prefabs: &Vec<Prefab>) -> Vec<MapObject> {
    let mut result: Vec<MapObject> = Vec::new();

    let mut current_len: u32 = 0;
    while current_len != parameters.2 {
        let i = rng.gen_range(0..prefabs.len());
        if current_len + prefabs[i].width > parameters.2 {
            continue;
        } else {
            let x_coord: u32;
            let y_coord: u32;
            if parameters.3 == Orientation::Left {
                x_coord = parameters.0 + current_len;
                y_coord = parameters.1;
            } else if parameters.3 == Orientation::Down {
                x_coord = parameters.0;
                y_coord = parameters.1 + current_len;
            } else {
                panic!("orientation of a wall should never be anything else than left or down");
            }

            current_len += prefabs[i].width;

            result.push(MapObject::new(
                x_coord,
                y_coord,
                parameters.3.clone(),
                &prefabs[i],
            ));
        }
    }

    // TODO: Randomize the result's order, so that the filler '1'-spaces are not at the end

    // TODO: Need to rotate the prefab so that the "inside" portion is inside
    //          (enemy camp is on the correct side of the wall)

    result
}

fn generate_insides(rng: &mut ChaCha8Rng) -> Vec<MapObject> {
    // TODO: Implement
    vec![]
}

fn generate_map(rng: &mut ChaCha8Rng) -> Map {
    let map_size = MapSize::new(70, 35);
    let orientation = Orientation::get_random();

    let walls = generate_walls(rng, &map_size, &orientation, "data/prefabs.yml".to_string());
    let inside_objects = generate_insides(rng);

    Map {
        walls,
        inside_objects,
    }
}

fn render_map(map: &Map) {
    for w in &map.walls {
        for obj in w {
            for _ in 0..obj.prefab.width {
                print!("{}", obj.prefab.render);
            }
        }
        println!();
    }
}

fn main() {
    let seed = 2;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let map = generate_map(&mut rng);
    render_map(&map);
}
