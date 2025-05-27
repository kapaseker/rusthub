use std::collections::HashMap;

/*fn main() {
    let v_one = vec![1, 2, 3, 4];
    println!("{:?}", v_one);

    let v_two: Vec<i32> = v_one.iter().map(|x| x * 2).collect();
    println!("{:?}", v_one);
    println!("{:?}", v_two);

    let solar_distance = HashMap::from([
        ("Mercury".to_string(), 0.4),
        ("Venus".to_string(), 0.7),
        ("Earth".to_string(), 1.0),
        ("Mars".to_string(), 1.5),
    ]);

    iter_map(&solar_distance);
}

fn iter_map(solars: &HashMap<String, f64>) {
    solars.iter().for_each(|(key, value)| {
        println!("{}", key)
    });

    if let Some(v) = solars.get("Venus") {
        println!("Venus has: {}", v);
    }
}
*/
// 
// struct SliceIter<'s, T> {
//     slice: &'s [T],
//     i: usize,
// }
// 
// impl<'s, T> Iterator for SliceIter<'s, T> {
//     type Item = &'s T;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.i == self.slice.len() {
//             None
//         } else {
//             let next = &self.slice[self.i];
//             self.i += 1;
//             Some(next)
//         }
//     }
// }
// 
// fn main() {
//     let slice = &[2, 4, 6, 8];
//     let iter = SliceIter { slice, i: 0 };
//     for elem in iter {
//         print!("{} ", elem);
//     }
// 
//     println!();
// 
//     let slice = &["ab", "cd", "ef", "app"];
//     let iter = SliceIter { slice, i: 0 };
//     for elem in iter {
//         print!("{} ", elem);
//     }
// 
//     println!();
// 
//     let result: i32 = (1..=10) // 创建一个从 1 到 10 的范围
//         .filter(|x| x % 2 == 0) // 只保留偶数
//         .map(|x| x * x) // 计算平方
//         .sum(); // 求和
// 
//     println!("The sum of squares of even numbers from 1 to 10 is: {}", result);
// 
// 
//     let result = [("apple", 1.0), ("google", 2.0), ("microsoft", 3.0), ("openai", 4.0)];
//     let company_map: HashMap<&str, f32> = result.iter().map(|(k, v)| (*k, *v)).collect();
//     println!("{:?}", company_map);
// }


struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (u32, u32);
    type IntoIter = GridRefIter<'a>;
    fn into_iter(self) -> GridRefIter<'a> {
        GridRefIter { grid: self, i: 0, j: 0 }
    }
}

struct GridRefIter<'a> {
    grid: &'a Grid,
    i: usize,
    j: usize,
}

impl<'a> Iterator for GridRefIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }
    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }
}