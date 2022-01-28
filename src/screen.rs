pub struct Screen {
    dim_x: i32,
    dim_y: i32,
    lines: Vec<Vec<char>>,
}

const HORIZONTAL_LINE_CHAR: char = '-';
const VERTICAL_LINE_CHAR: char = '|';

impl Screen {
    pub fn new(width: i32, height: i32) -> Self {
        let line_vec = vec![' '; width as usize];
        Self {
            dim_x: width,
            dim_y: height,
            lines: vec![line_vec.to_owned(); height as usize],
        }
    }
    pub fn draw_pixel(&mut self, x: i32, y: i32, c: char) {
        self.lines[y as usize][x as usize] = c;
    }
    pub fn draw_text(&mut self, x: i32, y: i32, text: Vec<char>) {
        let mut index = x;
        for c in text {
            self.lines[y as usize][index as usize] = c;
            index += 1;
        }
    }
    pub fn draw_box(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.lines[y as usize][(x + w - 1) as usize] = '┐';
        self.lines[(y + h - 1) as usize][(x + w - 1) as usize] = '┘';
        self.lines[y as usize][x as usize] = '┌';
        self.lines[(y + h - 1) as usize][x as usize] = '└';

        for xx in 1..w {
            self.lines[y as usize][(x + xx) as usize] = '─';
            self.lines[(y + h - 1) as usize][(x + xx) as usize] = '─';
        }
        for yy in 1..h {
            self.lines[(y + yy) as usize][x as usize] = '│';
            self.lines[(y + yy) as usize][(x + w - 1) as usize] = '│';
        }
    }
    pub fn draw_boxed_text(&mut self, x: i32, y: i32, text: Vec<char>) {
        let box_len = text.len() + 2;
        self.draw_text(x + 1, y + 1, text);
        self.draw_box(x, y, box_len as i32, 3);
    }
    pub fn draw_horizontal_line(&mut self, left: i32, right: i32, y: i32, c: Option<char>) {
        let c = c.unwrap_or_else(|| '│');
        for x in left..=right {
            self.lines[y as usize][x as usize] = c;
        }
    }
    pub fn draw_vertical_line(&mut self, top: i32, bottom: i32, x: i32) {
        for y in top..=bottom {
            self.lines[y as usize][x as usize] = '│';
        }
    }
    pub fn draw_vertical_line_complete(&mut self, top: i32, bottom: i32, x: i32) {
        for y in top..=bottom {
            let p = self.pixel(x, y);
            if p == '─' {
                let left = x != 0 && self.pixel(x - 1, y) != ' ';
                let right = x != self.dim_x - 1 && self.pixel(x + 1, y) != ' ';
                if y == top {
                    match (left, right) {
                        (true, true) => self.draw_pixel(x, y, '┬'),
                        (true, false) => self.draw_pixel(x, y, '┐'),
                        (false, true) => self.draw_pixel(x, y, '┌'),
                        (false, false) => self.draw_pixel(x, y, '┼'),
                    }
                } else if y == bottom {
                    match (left, right) {
                        (true, true) => self.draw_pixel(x, y, '┴'),
                        (true, false) => self.draw_pixel(x, y, '┘'),
                        (false, true) => self.draw_pixel(x, y, '└'),
                        (false, false) => self.draw_pixel(x, y, '┼'),
                    }
                } else {
                    match (left, right) {
                        (true, true) => self.draw_pixel(x, y, '┼'),
                        (true, false) => self.draw_pixel(x, y, '┤'),
                        (false, true) => self.draw_pixel(x, y, '├'),
                        (false, false) => self.draw_pixel(x, y, '┼'),
                    }
                }
            } else {
                // clang-format off
                match p {
                    '┐' => self.draw_pixel(x, y, '┤'),
                    '┘' => self.draw_pixel(x, y, '┤'),
                    '┌' => self.draw_pixel(x, y, '├'),
                    '└' => self.draw_pixel(x, y, '├'),
                    '┬' => self.draw_pixel(x, y, '┼'),
                    '┴' => self.draw_pixel(x, y, '┼'),
                    _ => self.draw_pixel(x, y, '│'),
                }
                // clang-format on
            }
        }
    }
    pub fn asciify(&mut self, style: u8) {
        match style {
            0 => {
                for line_vec in self.lines.iter_mut() {
                    for c in line_vec.iter_mut() {
                        match c {
                            '─' => *c = '-',
                            '│' => *c = '|',
                            '┐' => *c = '.',
                            '┘' => *c = '\'',
                            '┌' => *c = '.',
                            '└' => *c = '\'',
                            '┬' => *c = '-',
                            '┴' => *c = '-',
                            '├' => *c = '-',
                            '┤' => *c = '-',
                            '△' => *c = '^',
                            '▽' => *c = 'V',
                            _ => {
                                println!("{}?", c);
                            }
                        }
                    }
                }
            }
            1 => {
                for line_vec in self.lines.iter_mut() {
                    for c in line_vec.iter_mut() {
                        match c {
                            '─' => *c = '-',
                            '│' => *c = '|',
                            '┐' => *c = '.',
                            '┘' => *c = '\'',
                            '┌' => *c = '.',
                            '└' => *c = '\'',
                            '┬' => *c = '.',
                            '┴' => *c = '\'',
                            '├' => *c = '-',
                            '┤' => *c = '-',
                            '△' => *c = '^',
                            '▽' => *c = 'V',
                            _ => {
                                println!("{}?", c);
                            }
                        };
                    }
                }
            }
            _ => {
                println!("error style {}", style);
            }
        }
    }
    pub fn pixel(&self, x: i32, y: i32) -> char {
        self.lines[y as usize][x as usize]
    }
    pub fn resize(&mut self, dim_x: i32, dim_y: i32) {
        self.dim_x = dim_x;
        self.dim_y = dim_y;

        self.lines
            .resize_with(dim_y as usize, || vec![' '; dim_x as usize]);
    }
    pub fn append(&mut self, x: i32, y: i32, other: &mut Screen) {
        self.resize(
            self.dim_x.max(x + self.dim_x),
            self.dim_y.max(y + self.dim_y),
        );

        for dy in 0..other.dim_y {
            for dx in 0..other.dim_x {
                self.lines[(y + dy) as usize][(x + dx) as usize] =
                    other.lines[dy as usize][dx as usize];
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.dim_x
    }
    pub fn height(&self) -> i32 {
        self.dim_y
    }

    pub fn as_string(&self) -> String {
        self.lines.iter().fold("".into(), |mut acc, line_vec| {
            acc.push_str(&format!("{}\n", String::from_iter(line_vec).as_str()));
            acc
        })
    }
}
