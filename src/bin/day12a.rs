use petgraph::{
    algo::dijkstra,
    graph::{Graph, NodeIndex},
};
use std::fs;

fn compute(content: String) -> String {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let heightmap: Vec<Vec<u32>> = content
        .trim_end()
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, height)| match height {
                    'S' => {
                        start = (i, j);
                        1
                    }
                    'E' => {
                        end = (i, j);
                        26
                    }
                    height => height as u32 - 'a' as u32 + 1,
                })
                .collect()
        })
        .collect();

    let mut edges = vec![];

    let map_height = heightmap.len();
    let map_width = heightmap[0].len();

    let convert_coordinate_to_u32 = |x: usize, y: usize| (x * map_width + y) as u32;
    let convert_coordinates_to_edge = |x1: usize, y1: usize, x2: usize, y2: usize| {
        (
            convert_coordinate_to_u32(x1, y1),
            convert_coordinate_to_u32(x2, y2),
        )
    };

    for i in 0..map_height {
        for j in 0..map_width {
            if i < map_height - 1 {
                if heightmap[i][j] >= heightmap[i + 1][j] - 1 {
                    edges.push(convert_coordinates_to_edge(i, j, i + 1, j));
                }
                if heightmap[i][j] <= heightmap[i + 1][j] + 1 {
                    edges.push(convert_coordinates_to_edge(i + 1, j, i, j));
                }
            }
            if j < map_width - 1 {
                if heightmap[i][j] >= heightmap[i][j + 1] - 1 {
                    edges.push(convert_coordinates_to_edge(i, j, i, j + 1));
                }
                if heightmap[i][j] <= heightmap[i][j + 1] + 1 {
                    edges.push(convert_coordinates_to_edge(i, j + 1, i, j));
                }
            }
        }
    }

    let graph = Graph::<(), ()>::from_edges(&edges);
    let start = convert_coordinate_to_u32(start.0, start.1);
    let end = convert_coordinate_to_u32(end.0, end.1);
    let node_map = dijkstra(&graph, start.into(), Some(end.into()), |_| 1);

    node_map
        .get(&NodeIndex::new(end as usize))
        .unwrap()
        .to_string()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let result = compute(content);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn examples() {
        let content = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
        let expected = "31";
        assert_eq!(compute(content.to_string()), expected);
    }
}
