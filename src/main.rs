const HEIGHT: usize = 30;
const WIDTH:  usize = 30;

const ALIVE_CHAR: &str = "**";
const DEAD_CHAR: &str = "--";

fn index (x: usize, y: usize) -> usize {
    return y * WIDTH + x;
}

fn draw(alive: bool) {
    if alive {
        print!("{}", ALIVE_CHAR);
    } else {
        print!("{}", DEAD_CHAR);
    }
}

fn main() {
    let mut cells = [false; HEIGHT * WIDTH];

    cells[3 * WIDTH + 2] = true;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            draw(cells[index(j, i)]);
        }
        print!("\n");
    }
}
