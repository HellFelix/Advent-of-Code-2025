use std::{f32::INFINITY, fs::File, io::Read};

#[derive(Clone, Copy, Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}
impl Position {
    pub fn distance(&self, rhs: &Self) -> f32 {
        return ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2) + (self.z - rhs.z).powi(2))
            .sqrt();
    }
}

#[derive(Clone, Debug)]
struct Circuit {
    points: Vec<Position>,
    connections: Vec<(usize, usize)>,
}
impl Circuit {
    pub fn new(pos: Position) -> Self {
        Self {
            points: vec![pos],
            connections: Vec::new(),
        }
    }
    pub fn connect_circuit(&mut self, mut rhs: Circuit, conn_self: usize, conn_other: usize) {
        let conn_len = self.points.len();

        self.points.append(&mut rhs.points);

        self.connections.append(
            &mut rhs
                .connections
                .iter()
                .map(|c| (c.0 + conn_len, c.1 + conn_len))
                .collect(),
        );

        self.connections.push((conn_self, conn_other + conn_len));
    }
}

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let mut points: Vec<_> = input
        .lines()
        .map(|l| {
            let mut coords = l.split(",");
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            let z = coords.next().unwrap().parse().unwrap();
            Circuit::new(Position { x, y, z })
        })
        .collect();

    // let mut connected = Vec::new();
    // for _ in 0..=10 {
    //     let mut shortest = INFINITY;
    //     let (mut m, mut n) = (0, 0);
    //     for i in 0..points.len() {
    //         for j in i + 1..points.len() {
    //             let distance = points[i].points[0].distance(&points[j].points[0]);
    //             if distance < shortest
    //                 && !connected.contains(&(i, j))
    //                 && !connected.contains(&(j, i))
    //             {
    //                 m = i;
    //                 n = j;
    //                 shortest = distance;
    //             }
    //         }
    //     }
    //
    //     connected.push((m, n));
    //     println!(
    //         "Shortest is {:?} to {:?} - {shortest}",
    //         points[m].points[0], points[n].points[0]
    //     )
    // }

    let mut conn_x1 = 0.;
    let mut conn_x2 = 0.;
    while points.len() > 1 {
        let (is_connected, c1, c2, conn1, conn2) = find_shortest(&mut points);
        conn_x1 = points[c1].points[conn1].x;
        conn_x2 = points[c2].points[conn2].x;

        if !is_connected {
            let rhs = points.remove(c2);
            points[c1].connect_circuit(rhs, conn1, conn2);
        }

        println!("{}", points.len());
        // println!("Cycle finished");
        // for c in &points {
        //     println!("{c:?}");
        // }
    }

    println!("{conn_x1}, {conn_x2}");
    println!("x mul yields: {}", conn_x1 as i64 * conn_x2 as i64);

    // for c in &points {
    //     println!("{}", c.points.len());
    // }
    // let mut accum = 1;
    //
    // for _ in 0..3 {
    //     let ref_points = points.clone();
    //     let (i, max) = ref_points
    //         .iter()
    //         .enumerate()
    //         .max_by(|a, b| a.1.points.len().cmp(&b.1.points.len()))
    //         .unwrap();
    //
    //     println!("{}", max.points.len());
    //     points.remove(i);
    //     accum *= max.points.len();
    // }
    //
    // println!("Total: {accum}");
}

fn find_shortest(points: &mut Vec<Circuit>) -> (bool, usize, usize, usize, usize) {
    let mut shortest = INFINITY;
    let mut is_connected = false;
    let (mut c1, mut c2) = (0, 0);
    let (mut conn1, mut conn2) = (0, 0);

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for (index1, p1) in points[i].points.iter().enumerate() {
                for (index2, p2) in points[j].points.iter().enumerate() {
                    let distance = p1.distance(&p2);
                    if distance < shortest {
                        shortest = distance;
                        c1 = i;
                        c2 = j;

                        conn1 = index1;
                        conn2 = index2;
                    }
                }
            }
        }
    }

    for (i, circuit) in points.iter().enumerate() {
        for j in 0..circuit.points.len() {
            for k in j + 1..circuit.points.len() {
                let distance = circuit.points[j].distance(&circuit.points[k]);
                if distance < shortest
                    && !circuit.connections.contains(&(j, k))
                    && !circuit.connections.contains(&(k, j))
                {
                    shortest = distance;
                    is_connected = true;
                    c1 = i;
                    c2 = i;

                    conn1 = j;
                    conn2 = k;
                }
            }
        }
    }

    if is_connected {
        //println!("Shortest is internal");
        points[c1].connections.push((conn1, conn2));
    }

    // println!(
    //     "shortest is between {:?}, and {:?} - {shortest}",
    //     points[c1].points[conn1], points[c2].points[conn2]
    // );

    (is_connected, c1, c2, conn1, conn2)
}
