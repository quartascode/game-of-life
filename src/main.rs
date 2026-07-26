const HEIGHT: usize = 30;
const WIDTH:  usize = 30;

const ALIVE_CHAR: &str = "**";
const DEAD_CHAR: &str = "--";

fn index (x: usize, y: usize) -> usize {
    return y * WIDTH + x;
}

fn main() {
    let mut cells = [false; HEIGHT * WIDTH];

    cells[3 * WIDTH + 2] = true;

    let mut buffer: String = String::new();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if cells[index(j, i)] {
                buffer.push_str(ALIVE_CHAR);
            } else {
                buffer.push_str(DEAD_CHAR);
            }
        }
        buffer.push_str("\n");
    }

    print!("{}", buffer);
}
