use itertools::Itertools;
use macroquad::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug)]
struct World {
    tiles: Vec<Vec<i8>>,
    width: usize,
    height: usize,
}

#[derive(EnumIter, Debug, PartialEq)]
enum Mode {
    Flood,
    Wall,
    Remove,
}

fn floodfill(mut world: World, start_pos: Vec2, mut steps: i32) -> Vec<Vec<i8>> {
    let new_tiles = &mut world.tiles;

    if new_tiles[start_pos.y as usize][start_pos.x as usize] == 0 {
        new_tiles[start_pos.y as usize][start_pos.x as usize] = 1;
    }

    while steps > 0 {
        for (y_idx, y) in new_tiles.clone().into_iter().enumerate() {
            for (x_idx, x) in y.into_iter().enumerate() {
                if x == 1 {
                    if x_idx as i32 - 1 >= 0
                        && x_idx - 1 < world.width
                        && new_tiles[y_idx][x_idx - 1] == 0
                    {
                        new_tiles[y_idx][x_idx - 1] = 1;
                        steps -= 1;
                    }

                    if x_idx as i32 + 1 >= 0
                        && x_idx + 1 < world.width
                        && new_tiles[y_idx][x_idx + 1] == 0
                    {
                        new_tiles[y_idx][x_idx + 1] = 1;
                        steps -= 1;
                    }

                    if y_idx as i32 - 1 >= 0
                        && y_idx - 1 < world.height
                        && new_tiles[y_idx - 1][x_idx] == 0
                    {
                        new_tiles[y_idx - 1][x_idx] = 1;
                        steps -= 1;
                    }

                    if y_idx as i32 + 1 >= 0
                        && y_idx + 1 < world.height
                        && new_tiles[y_idx + 1][x_idx] == 0
                    {
                        new_tiles[y_idx + 1][x_idx] = 1;
                        steps -= 1;
                    }
                    steps -= 1;
                }
            }
        }
    }

    new_tiles.to_vec()
}

// fn print_grid(tiles: Vec<Vec<i8>>) {
//     for y in tiles {
//         for x in y {
//             print!("{} ", x);
//         }
//         print!("\n");
//     }
// }

fn render_grid(world: World) {
    for (y_idx, y) in world.tiles.into_iter().enumerate() {
        for (x_idx, val) in y.into_iter().enumerate() {
            // eprintln!("{} {}", x_idx as f32, y_idx as f32);
            if val == 0 {
                draw_rectangle(
                    x_idx as f32 * screen_width() / world.width as f32,
                    y_idx as f32 * screen_height() / world.height as f32,
                    screen_width() / world.width as f32,
                    screen_height() / world.height as f32,
                    BLUE,
                );
            } else if val == 1 {
                draw_rectangle(
                    x_idx as f32 * screen_width() / world.width as f32,
                    y_idx as f32 * screen_height() / world.height as f32,
                    screen_width() / world.width as f32,
                    screen_height() / world.height as f32,
                    RED,
                );
            } else if val == 2 {
                draw_rectangle(
                    x_idx as f32 * screen_width() / world.width as f32,
                    y_idx as f32 * screen_height() / world.height as f32,
                    screen_width() / world.width as f32,
                    screen_height() / world.height as f32,
                    GRAY,
                );
            }
        }
    }
}

#[macroquad::main("Floodfill")]
async fn main() {
    let modes = Mode::iter().collect_vec();
    let mut mode = 0;
    let mut fill_steps = 120;
    let mut filled = false;

    let width: usize = 25;
    let height: usize = 20;

    let mut world: World = World {
        tiles: vec![vec![0; width]; height],
        width: width,
        height: height,
    };

    loop {
        clear_background(BLACK);

        let (x, y) = (
            (mouse_position().0 / (screen_width() / world.width as f32))
                .clamp(0.0, world.width as f32 - 1.0),
            (mouse_position().1 / (screen_height() / world.height as f32))
                .clamp(0.0, world.height as f32 - 1.0),
        );

        if mouse_wheel() > (0.0, 0.0) && mode != modes.len() - 1 {
            mode += 1;
        } else if mouse_wheel() < (0.0, 0.0) && mode != 0 {
            mode -= 1;
        }
        if is_key_down(KeyCode::Left) && fill_steps != 0 {
            fill_steps -= 1;
        } else if is_key_down(KeyCode::Right) {
            fill_steps += 1;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if modes[mode] == Mode::Flood && !filled {
                world.tiles = floodfill(world.clone(), Vec2 { x: x, y: y }, fill_steps);
                filled = true;
            }
            if modes[mode] == Mode::Wall {
                world.tiles[y as usize][x as usize] = 2;
            }
            if modes[mode] == Mode::Remove {
                world.tiles[y as usize][x as usize] = 0;
            }
        }

        render_grid(world.clone());

        draw_text(
            format!("Mode: {:?}", modes[mode]).as_str(),
            10.0,
            50.0,
            50.0,
            WHITE,
        );
        draw_text(
            format!("Fill steps: {:?}", fill_steps).as_str(),
            10.0,
            120.0,
            50.0,
            WHITE,
        );
        next_frame().await;
    }

    // println!("{:?}", &tiles);
}
