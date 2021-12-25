use nannou::prelude::*;

fn step(grid: &mut Vec<Vec<char>>) -> bool {
    let mut moved = false;
    let mut newgrid = grid.clone();
    // rightwards first
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let newc = (c + 1) % grid[0].len();
            if grid[r][c] == '>' && grid[r][newc] == '.' {
                moved = true;
                newgrid[r][newc] = '>';
                newgrid[r][c] = '.';
            }
        }
    }
    *grid = newgrid;
    newgrid = grid.clone();
    // downwards next
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let newr = (r + 1) % grid.len();
            if grid[r][c] == 'v' && grid[newr][c] == '.' {
                moved = true;
                newgrid[newr][c] = 'v';
                newgrid[r][c] = '.';
            }
        }
    }
    *grid = newgrid;
    moved
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    initial_grid: Vec<Vec<char>>,
    grid: Vec<Vec<char>>,
    num_stopped: usize,
}

fn model(_app: &App) -> Model {
    let grid = common::read_2d(&common::read_file("d25.txt"));
    Model {
        initial_grid: grid.clone(),
        grid,
        num_stopped: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let moved = step(&mut model.grid);
    if moved {
        model.num_stopped = 0;
    } else {
        model.num_stopped += 1;
    }
    if model.num_stopped > 100 {
        model.grid = model.initial_grid.clone();
        model.num_stopped = 0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let boundary = app.window_rect();
    let draw = app.draw();
    draw.background().color(BEIGE);
    let rl = model.grid.len();
    let cl = model.grid[0].len();
    for (ridx, r) in model.grid.iter().enumerate() {
        for (cidx, c) in r.iter().enumerate() {
            let color = if *c == '>' {
                DARKGREEN
            } else if *c == 'v' {
                RED
            } else {
                BLACK
            };
            let x = map_range(cidx, 0, cl - 1, boundary.left(), boundary.right());
            let y = map_range(ridx, 0, rl - 1, boundary.top(), boundary.bottom());
            draw.text(&c.to_string())
                .color(color)
                .x_y(x, y)
                .w(boundary.w() / cl as f32)
                .h(boundary.h() / rl as f32);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
