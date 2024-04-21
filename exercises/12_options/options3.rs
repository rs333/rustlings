// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // Solution 1
    // if let Some(p) = &y {
    //     println!("Co-ordinates are {},{} ", p.x, p.y);
    // } else {
    //     panic!("no match!");
    // }
    // Solution 2
    //match &y {
    //    Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
    //    _ => panic!("no match!"),
    //}
    // Other solution
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
