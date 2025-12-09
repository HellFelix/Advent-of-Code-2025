use std::{fs::File, i64, io::Read};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}
impl Pos {
    pub fn parse(s: &str) -> Self {
        let mut i = s.split(",");
        Self {
            x: i.next().unwrap().parse().unwrap(),
            y: i.next().unwrap().parse().unwrap(),
        }
    }

    pub fn rectangle_size(&self, other: &Self) -> i64 {
        // plus one because inclusive
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

const ASSUMPTION_CLOCKWISE: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeType {
    StraightX,
    StraightY,

    // bool is true if outside corner (convex). false if inside (concave)
    TopRight(bool),
    TopLeft(bool),
    BottomRight(bool),
    BottomLeft(bool),
}

struct Shape {
    nodes: Vec<(Pos, NodeType)>,
}
impl Shape {
    pub fn construct(positions: Vec<Pos>) -> Self {
        let mut nodes = Vec::new();
        for i in 0..positions.len() {
            let before = positions[if i == 0 { positions.len() - 1 } else { i - 1 }];
            let currect = positions[i];
            let after = positions[if i == positions.len() - 1 { 0 } else { i + 1 }];

            if before.x == currect.x {
                if currect.x == after.x {
                    nodes.push((currect, NodeType::StraightY));
                } else if before.y < currect.y {
                    // We've gone down
                    if after.x < currect.x {
                        nodes.push((currect, NodeType::BottomRight(ASSUMPTION_CLOCKWISE)));
                    } else {
                        nodes.push((currect, NodeType::BottomLeft(!ASSUMPTION_CLOCKWISE)));
                    }
                } else {
                    // We've gone up
                    if after.x < currect.x {
                        nodes.push((currect, NodeType::TopRight(!ASSUMPTION_CLOCKWISE)));
                    } else {
                        nodes.push((currect, NodeType::TopLeft(ASSUMPTION_CLOCKWISE)));
                    }
                }
            } else if before.y == currect.y {
                if currect.y == after.y {
                    nodes.push((currect, NodeType::StraightX));
                } else if before.x < currect.x {
                    // We've gone to the right
                    if after.y < currect.y {
                        nodes.push((currect, NodeType::BottomRight(!ASSUMPTION_CLOCKWISE)));
                    } else {
                        nodes.push((currect, NodeType::TopRight(ASSUMPTION_CLOCKWISE)));
                    }
                } else {
                    // We've gone to the left
                    if after.y < currect.y {
                        nodes.push((currect, NodeType::BottomLeft(ASSUMPTION_CLOCKWISE)));
                    } else {
                        nodes.push((currect, NodeType::TopLeft(!ASSUMPTION_CLOCKWISE)));
                    }
                }
            }
        }

        Self { nodes }
    }

    pub fn shift_out(self) -> Vec<Pos> {
        let mut res_nodes = Vec::new();
        for n in self.nodes {
            match n.1 {
                NodeType::TopRight(is_outer) => {
                    let shifted = if is_outer {
                        Pos {
                            x: n.0.x + 1,
                            y: n.0.y - 1,
                        }
                    } else {
                        Pos {
                            x: n.0.x - 1,
                            y: n.0.y + 1,
                        }
                    };
                    res_nodes.push(shifted);
                }
                NodeType::BottomRight(is_outer) => {
                    let shifted = if is_outer {
                        Pos {
                            x: n.0.x + 1,
                            y: n.0.y + 1,
                        }
                    } else {
                        Pos {
                            x: n.0.x - 1,
                            y: n.0.y - 1,
                        }
                    };
                    res_nodes.push(shifted);
                }
                NodeType::TopLeft(is_outer) => {
                    let shifted = if is_outer {
                        Pos {
                            x: n.0.x - 1,
                            y: n.0.y - 1,
                        }
                    } else {
                        Pos {
                            x: n.0.x + 1,
                            y: n.0.y + 1,
                        }
                    };
                    res_nodes.push(shifted);
                }
                NodeType::BottomLeft(is_outer) => {
                    let shifted = if is_outer {
                        Pos {
                            x: n.0.x - 1,
                            y: n.0.y + 1,
                        }
                    } else {
                        Pos {
                            x: n.0.x + 1,
                            y: n.0.y - 1,
                        }
                    };
                    res_nodes.push(shifted);
                }
                _ => {}
            }
        }

        res_nodes
    }
}

#[derive(Debug)]
enum Edge {
    // y-val, x_min, x_max
    X(i64, (i64, i64)),

    // x-val, y_min, y_max
    Y(i64, (i64, i64)),
}
impl Edge {
    pub fn crosses(&self, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
        match self {
            Self::X(y, (e_x_min, e_x_max)) => {
                y_min <= *y && y_max >= *y && (x_min <= *e_x_max && *e_x_min <= x_max)
            }
            Self::Y(x, (e_y_min, e_y_max)) => {
                x_min <= *x && x_max >= *x && (y_min <= *e_y_max && *e_y_min <= y_max)
            }
        }
    }
}

fn construct_shell(out_corners: Vec<Pos>) -> Vec<Edge> {
    let mut shell = Vec::new();
    for i in 0..out_corners.len() {
        let current = out_corners[i];
        let after = out_corners[if i == out_corners.len() - 1 { 0 } else { i + 1 }];

        if current.x == after.x {
            let min_y = current.y.min(after.y);
            let max_y = current.y.max(after.y);
            shell.push(Edge::Y(current.x, (min_y, max_y)));
        } else {
            // y must be same
            let min_x = current.x.min(after.x);
            let max_x = current.x.max(after.x);
            shell.push(Edge::X(current.y, (min_x, max_x)));
        }
    }

    shell
}

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let positions: Vec<Pos> = input.lines().map(Pos::parse).collect();

    let shape = Shape::construct(positions.clone());

    let shell = construct_shell(shape.shift_out());

    let mut largest = 0;
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let size = positions[i].rectangle_size(&positions[j]);
            if is_encased(positions[i], positions[j], &shell) && size > largest {
                largest = size;
            }
        }
    }

    println!("Largest possible {largest}");
}

// fn find_inside(shape: &Shape) -> HashMap<Pos, bool> {
//     let mut res = HashMap::new();
//
//     let min_x = shape
//         .nodes
//         .iter()
//         .min_by(|n1, n2| n1.0.x.cmp(&n2.0.x))
//         .unwrap()
//         .0
//         .x;
//     let max_x = shape
//         .nodes
//         .iter()
//         .max_by(|n1, n2| n1.0.x.cmp(&n2.0.x))
//         .unwrap()
//         .0
//         .x;
//     let min_y = shape
//         .nodes
//         .iter()
//         .min_by(|n1, n2| n1.0.y.cmp(&n2.0.y))
//         .unwrap()
//         .0
//         .y;
//     let max_y = shape
//         .nodes
//         .iter()
//         .max_by(|n1, n2| n1.0.y.cmp(&n2.0.y))
//         .unwrap()
//         .0
//         .y;
//
//     for i in 0..shape.nodes.len() {
//         println!("Mapping node {i}");
//         let before = shape.nodes[if i == 0 { shape.nodes.len() - 1 } else { i - 1 }];
//         let currect = shape.nodes[i];
//         let after = shape.nodes[if i == shape.nodes.len() - 1 { 0 } else { i + 1 }];
//
//         let mut x_range = before.0.x.min(after.0.x)..=before.0.x.max(after.0.x);
//         let mut y_range = before.0.y.min(after.0.y)..=before.0.y.max(after.0.y);
//
//         let add = match currect.1 {
//             NodeType::StraightX => true,
//             NodeType::StraightY => true,
//             NodeType::TopRight(is_outer) => {
//                 if !is_outer {
//                     x_range = before.0.x.min(after.0.x)..=before.0.x.max(after.0.x) - 1;
//                     y_range = before.0.y.min(after.0.y) + 1..=before.0.y.max(after.0.y);
//                 }
//                 is_outer
//             }
//             NodeType::BottomRight(is_outer) => {
//                 if !is_outer {
//                     x_range = before.0.x.min(after.0.x)..=before.0.x.max(after.0.x) - 1;
//                     y_range = before.0.y.min(after.0.y)..=before.0.y.max(after.0.y) - 1;
//                 }
//                 is_outer
//             }
//             NodeType::TopLeft(is_outer) => {
//                 if !is_outer {
//                     x_range = before.0.x.min(after.0.x) + 1..=before.0.x.max(after.0.x);
//                     y_range = before.0.y.min(after.0.y) + 1..=before.0.y.max(after.0.y);
//                 }
//                 is_outer
//             }
//             NodeType::BottomLeft(is_outer) => {
//                 if !is_outer {
//                     x_range = before.0.x.min(after.0.x) + 1..=before.0.x.max(after.0.x);
//                     y_range = before.0.y.min(after.0.y)..=before.0.y.max(after.0.y) - 1;
//                 }
//                 is_outer
//             }
//         };
//         // Should filter straight - doesn't cut enough.
//         // Adjecent inner corners cut on inclusive/non-inclusive.
//         for x in x_range {
//             for y in y_range.clone() {
//                 res.insert(Pos { x, y }, add);
//             }
//         }
//     }
//
//     // for y in min_y..=max_y {
//     //     for x in min_x..=max_x {
//     //         print!(
//     //             "{}",
//     //             if *res.get(&Pos { x, y }).unwrap() {
//     //                 "#"
//     //             } else {
//     //                 "."
//     //             }
//     //         );
//     //     }
//     //     print!("\n");
//     // }
//     // stdout().flush().unwrap();
//
//     res
//
//     // closest.iter().all(|n| {
//     //     let corner_pos = n.0;
//     //     match n.1 {
//     //         NodeType::StraightX => panic!("Should be filtered"),
//     //         NodeType::StraightY => panic!("Should be filtered"),
//     //
//     //         NodeType::TopRight(is_outside) => {
//     //             if is_outside {
//     //                 p.x <= corner_pos.x && p.y >= corner_pos.y
//     //             } else {
//     //                 p.x >= corner_pos.x || p.y <= corner_pos.y
//     //             }
//     //         }
//     //
//     //         NodeType::BottomRight(is_outside) => {
//     //             if is_outside {
//     //                 p.x <= corner_pos.x && p.y <= corner_pos.y
//     //             } else {
//     //                 p.x >= corner_pos.x || p.y >= corner_pos.y
//     //             }
//     //         }
//     //
//     //         NodeType::TopLeft(is_outside) => {
//     //             if is_outside {
//     //                 p.x >= corner_pos.x && p.y >= corner_pos.y
//     //             } else {
//     //                 p.x <= corner_pos.x || p.y <= corner_pos.y
//     //             }
//     //         }
//     //
//     //         NodeType::BottomLeft(is_outside) => {
//     //             if is_outside {
//     //                 p.x >= corner_pos.x && p.y <= corner_pos.y
//     //             } else {
//     //                 p.x <= corner_pos.x || p.y >= corner_pos.y
//     //             }
//     //         }
//     //     }
//     // })
// }

// fn is_inside(p: &Pos, inside: &HashMap<Pos, bool>) -> bool {
//     if let Some(v) = inside.get(p) {
//         *v
//     } else {
//         false
//     }
// }

fn is_encased(p1: Pos, p2: Pos, shell: &Vec<Edge>) -> bool {
    let x_max = p1.x.max(p2.x);
    let x_min = p1.x.min(p2.x);

    let y_max = p1.y.max(p2.y);
    let y_min = p1.y.min(p2.y);

    let mut res = true;

    for e in shell {
        if e.crosses(x_min, x_max, y_min, y_max) {
            res = false;
            break;
        }
    }

    res
}
