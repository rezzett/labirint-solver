struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn new(y: usize, x: usize) -> Point {
        Point { y, x }
    }
}

fn main() {
    // 0 -> wall 1 -> path 2 -> exit
    let mut labirint = [[2usize, 1, 1, 1], [0, 0, 1, 1], [0, 0, 0, 1]];
    let mut path = vec![];
    path.push(Point::new(0, 3));
    let mut steep = 0;

    loop {
        let x = path.get(steep).unwrap().x;
        let y = path.get(steep).unwrap().y;
        labirint[y][x] = 0;

        if y + 1 < labirint.len() {
            // down
            if labirint[y + 1][x] == 2 {
                println!("Exit found!");
                break;
            } else if labirint[y + 1][x] == 1 {
                path.push(Point::new(y + 1, x));
                println!("Moved down");
                steep += 1;
                continue;
            }
        }

        if y >= 1 {
            // up
            if labirint[y - 1][x] == 2 {
                println!("Exit found!");
                break;
            } else if labirint[y - 1][x] == 1 {
                println!("Moved up");
                path.push(Point::new(y - 1, x));
                steep += 1;
                continue;
            }
        }

        if x >= 1 {
            // left
            if labirint[y][x - 1] == 2 {
                println!("Exit found!");
                break;
            } else if labirint[y][x - 1] == 1 {
                println!("Moved left");
                path.push(Point::new(y, x - 1));
                steep += 1;
                continue;
            }
        }

        if x < labirint[y].len() - 1 {
            // right
            if labirint[y][x + 1] == 2 {
                println!("Exit found!");
                break;
            } else if labirint[y][x + 1] == 1 {
                println!("Moved right");
                path.push(Point::new(y, x + 1));
                steep += 1;
                continue;
            }
        }

        if path.len() > 0 {
            path.pop().unwrap();
            if path.len() <= 0 {
                println!("There is no way! :-\\");
                break;
            }
            steep -= 1;
            println!("Moved back");
        }
    }
}
