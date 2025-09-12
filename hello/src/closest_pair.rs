fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    if points.len() < 2 {
        panic!("At least two points are required");
    }

    // 对点按x坐标排序
    let mut points_x: Vec<(f64, f64)> = points.to_vec();
    points_x.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // 对点按y坐标排序
    let mut points_y: Vec<(f64, f64)> = points.to_vec();
    points_y.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    closest_pair_rec(&points_x, &points_y).1
}

fn closest_pair_rec(points_x: &[(f64, f64)], points_y: &[(f64, f64)]) -> (f64, ((f64, f64), (f64, f64))) {
    let n = points_x.len();

    // 基本情况：当点数少于等于3时，直接计算所有对的距离
    if n <= 3 {
        return brute_force_closest(points_x);
    }

    // 分割点
    let mid = n / 2;
    let midpoint = points_x[mid];

    // 将点分为左右两部分（基于x坐标）
    let (points_x_left, points_x_right) = points_x.split_at(mid);

    // 根据中线分割y排序的点（更简洁的方法）
    let (points_y_left, points_y_right): (Vec<_>, Vec<_>) = points_y
        .iter()
        .partition(|&&(x, _)| x < midpoint.0);

    // 处理x坐标恰好等于midpoint.0的点
    let points_on_line: Vec<(f64, f64)> = points_y.iter()
        .filter(|&&(x, _)| x == midpoint.0)
        .cloned()
        .collect();

    // 将在中线上的点分配给左或右，这里简单地分配给左侧
    let mut points_y_left: Vec<(f64, f64)> = points_y_left.into_iter().cloned().collect();
    points_y_left.extend(points_on_line);

    let points_y_right: Vec<(f64, f64)> = points_y_right.into_iter().cloned().collect();

    // 递归求解左右两部分的最近点对
    let (delta_left, pair_left) = closest_pair_rec(points_x_left, &points_y_left);
    let (delta_right, pair_right) = closest_pair_rec(points_x_right, &points_y_right);

    // 找出较小的距离和对应的点对
    let (delta, pair) = if delta_left < delta_right {
        (delta_left, pair_left)
    } else {
        (delta_right, pair_right)
    };

    // 构建带状区域中的点
    let strip: Vec<(f64, f64)> = points_y.iter()
        .filter(|&&(x, _)| (x - midpoint.0).abs() < delta)
        .cloned()
        .collect();

    // 检查带状区域中是否存在更近的点对
    let (strip_delta, strip_pair) = closest_in_strip(&strip, delta, pair);

    if strip_delta < delta {
        (strip_delta, strip_pair)
    } else {
        (delta, pair)
    }
}

fn brute_force_closest(points: &[(f64, f64)]) -> (f64, ((f64, f64), (f64, f64))) {
    let mut min_distance = f64::INFINITY;
    let mut closest_pair = ((0.0, 0.0), (0.0, 0.0));

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = euclidean_distance(points[i], points[j]);
            if distance < min_distance {
                min_distance = distance;
                closest_pair = (points[i], points[j]);
            }
        }
    }

    (min_distance, closest_pair)
}

fn euclidean_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn closest_in_strip(strip: &[(f64, f64)], delta: f64, best_pair: ((f64, f64), (f64, f64))) -> (f64, ((f64, f64), (f64, f64))) {
    let mut min_distance = delta;
    let mut closest_pair = best_pair;

    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() && (strip[j].1 - strip[i].1) < min_distance {
            let distance = euclidean_distance(strip[i], strip[j]);
            if distance < min_distance {
                min_distance = distance;
                closest_pair = (strip[i], strip[j]);
            }
            j += 1;
        }
    }

    (min_distance, closest_pair)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let points = vec![
            (2.0, 2.0), // A
            (2.0, 8.0), // B
            (5.0, 5.0), // C
            (6.0, 3.0), // D
            (6.0, 7.0), // E
            (7.0, 4.0), // F
            (7.0, 9.0)  // G
        ];

        let result = closest_pair(&points);
        let expected1 = ((6.0, 3.0), (7.0, 4.0));
        let expected2 = ((7.0, 4.0), (6.0, 3.0));

        let distance_result = euclidean_distance(result.0, result.1);
        let distance_expected = euclidean_distance(expected1.0, expected1.1);

        assert!((distance_result - distance_expected).abs() < 1e-9);
        assert!(result == expected1 || result == expected2);
    }

    #[test]
    fn test_simple_case() {
        let points = vec![(0.0, 0.0), (1.0, 1.0)];
        let result = closest_pair(&points);
        assert!(result == ((0.0, 0.0), (1.0, 1.0)) || result == ((1.0, 1.0), (0.0, 0.0)));
    }

    #[test]
    fn test_vertical_line() {
        let points = vec![(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)];
        let result = closest_pair(&points);
        let distance = euclidean_distance(result.0, result.1);
        assert!((distance - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_horizontal_line() {
        let points = vec![(1.0, 1.0), (3.0, 1.0), (5.0, 1.0)];
        let result = closest_pair(&points);
        let distance = euclidean_distance(result.0, result.1);
        assert!((distance - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_square() {
        let points = vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)];
        let result = closest_pair(&points);
        let distance = euclidean_distance(result.0, result.1);
        assert!((distance - 1.0).abs() < 1e-9);
    }
}

fn main() {
    let points = vec![
        (2.0, 2.0), // A
        (2.0, 8.0), // B
        (5.0, 5.0), // C
        (6.0, 3.0), // D
        (6.0, 7.0), // E
        (7.0, 4.0), // F
        (7.0, 9.0)  // G
    ];

    let (p1, p2) = closest_pair(&points);
    let distance = euclidean_distance(p1, p2);

    println!("最近的点对是: {:?} 和 {:?}", p1, p2);
    println!("它们之间的距离是: {}", distance);
}