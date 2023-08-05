use std::fs::File;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

struct MapSize {
    width: u32,
    height: u32,
}

impl MapSize {
    fn new(width: u32, height: u32) -> MapSize {
        MapSize { width, height }
    }

    fn walls(&self) -> Vec<u32> {
        vec![self.width, self.height, self.width, self.height]
    }
}

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

#[derive(Clone)]
struct MapObject {
    x: u32,
    y: u32,
    prefab: Prefab,
}

impl MapObject {
    fn new(x: u32, y: u32, prefab: &Prefab) -> MapObject {
        MapObject {
            x,
            y,
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
    for (indx, wall_len) in map_size.walls().iter().enumerate() {
        result[indx] = generate_wall(rng, *wall_len, &prefabs);
        println!(
            "generated wall #{} by using {} prefabs",
            indx,
            result[indx].len()
        );
    }

    // TODO: Join the corners by adding corner-tiles (replacing the ends of the walls)

    // TODO: Generate entrace and exit by using map orientation (by forcing a exit prefab somewhere)

    result
}

fn generate_wall(rng: &mut ChaCha8Rng, wall_length: u32, prefabs: &Vec<Prefab>) -> Vec<MapObject> {
    let mut result: Vec<MapObject> = Vec::new();

    let mut current_len: u32 = 0;
    while current_len != wall_length {
        let i = rng.gen_range(0..prefabs.len());
        if current_len + prefabs[i].width > wall_length {
            continue;
        } else {
            current_len += prefabs[i].width;

            // TODO: Correctly fill in the x and y coordinates
            result.push(MapObject::new(current_len, 0, &prefabs[i]));
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
