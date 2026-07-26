use std::time::Duration;

const HEIGHT: usize = 30;
const WIDTH:  usize = 30;
const FPS:    usize = 10;
const DT:     f64   = 1.0 / FPS as f64;

const ALIVE_CHAR: &str = "$$";
const DEAD_CHAR: &str = "--";

fn index (x: usize, y: usize) -> usize {
    return y * WIDTH + x;
}

fn main() {
    let mut cells = [false; HEIGHT * WIDTH];

    cells[index(4, 5)] = true;
    cells[index(5, 5)] = true;
    cells[index(6, 5)] = true;
    cells[index(6, 4)] = true;
    cells[index(5, 3)] = true;

    cells[index(30 - 4, 5 + 2)] = true;
    cells[index(30 - 5, 5 + 2)] = true;
    cells[index(30 - 6, 5 + 2)] = true;
    cells[index(30 - 6, 4 + 2)] = true;
    cells[index(30 - 5, 3 + 2)] = true;

    print!("\x1B[2J\x1B[1;1H");
    loop {

        let mut frame_buffer: String = String::new();

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if cells[index(j, i)] {
                    frame_buffer.push_str(ALIVE_CHAR);
                } else {
                    frame_buffer.push_str(DEAD_CHAR);
                }
            }
            frame_buffer.push_str("\n");
        }

        print!("\x1b[H");
        print!("{}", frame_buffer);

        let mut next_cells = [false; HEIGHT * WIDTH];

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let mut neighbors: i32 = 0;
                for dx in -1isize..=1 {
                    for dy in -1isize..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = j as isize + dx;
                        let ny = i as isize + dy;

                        if nx < 0 || nx >= WIDTH as isize || ny < 0 || ny >= HEIGHT as isize {
                            continue;
                        }

                        if cells[index(nx as usize, ny as usize)] {
                            neighbors += 1;
                        }
                    }
                }
                if cells[index(j, i)] {
                    if neighbors < 2 || neighbors > 3 {
                        next_cells[index(j, i)] = false;
                    } else {
                        if neighbors == 2 || neighbors == 3 {
                            next_cells[index(j, i)] = true;
                        }
                    }
                } else {
                    if neighbors == 3 {
                        next_cells[index(j, i)] = true;
                    }
                }
            }
        }

        cells = next_cells;


        std::thread::sleep(Duration::from_secs_f64(DT));
    }

}
