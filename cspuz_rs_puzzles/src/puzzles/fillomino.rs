use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_pzprxs, url_to_problem, Choice, Choice2, Combinator, ContextBasedGrid, Dict,
    Grid, HexInt, Optionalize, Rooms, Size, Spaces, Tuple3,
};
use cspuz_rs::solver::{bool_constant, Config, GraphDivisionMode, Solver};

pub fn solve_fillomino(
    max3: bool,
    clues: &[Vec<Option<i32>>],
    default_borders: &Option<graph::InnerGridEdges<Vec<Vec<bool>>>>,
) -> Option<(
    Vec<Vec<Option<i32>>>,
    graph::BoolInnerGridEdgesIrrefutableFacts,
)> {
    let (h, w) = util::infer_shape(clues);

    let mut config = Config::default();
    config.graph_division_mode = GraphDivisionMode::Rust;

    let mut solver = Solver::with_config(config);
    let mut ranges = vec![];
    let mut max = (h * w) as i32;
    if max3 {
        max = 3;
    }
    for y in 0..h {
        let mut row = vec![];
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n < 0 {
                    row.push((1, max));
                } else {
                    row.push((n, n));
                }
            } else {
                row.push((1, max));
            }
        }
        ranges.push(row);
    }
    let num = &solver.int_var_2d_from_ranges((h, w), &ranges);
    solver.add_answer_key_int(num);

    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);
    solver.add_expr(
        num.slice((.., ..(w - 1)))
            .ne(num.slice((.., 1..)))
            .iff(&is_border.vertical),
    );
    solver.add_expr(
        num.slice((..(h - 1), ..))
            .ne(num.slice((1.., ..)))
            .iff(&is_border.horizontal),
    );

    if let Some(default_borders) = default_borders {
        for y in 0..h {
            for x in 0..(w - 1) {
                if default_borders.vertical[y][x] {
                    solver.add_expr(is_border.vertical.at((y, x)));
                }
            }
        }
        for y in 0..(h - 1) {
            for x in 0..w {
                if default_borders.horizontal[y][x] {
                    solver.add_expr(is_border.horizontal.at((y, x)));
                }
            }
        }
    }

    graph::graph_division_2d(&mut solver, num, &is_border);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    if n > 3 && max3 {
                        solver.add_expr(bool_constant(false));
                    } else {
                        solver.add_expr(num.at((y, x)).eq(n));
                    }
                }
            }
        }
    }

    solver
        .irrefutable_facts()
        .map(|f| (f.get(num), f.get(&is_border)))
}

type Problem = (
    bool,
    Vec<Vec<Option<i32>>>,
    Option<graph::InnerGridEdges<Vec<Vec<bool>>>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "t/")),
            Box::new(Dict::new(false, "")),
        ]),
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
        Choice::new(vec![
            Box::new(Optionalize::new(Rooms)),
            Box::new(Dict::new(None, "")),
        ]),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url_pzprxs(combinator(), "fillomino", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["fillomino"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        (
            false,
            vec![
                vec![None, Some(1), None, None, None],
                vec![None, None, Some(3), Some(4), None],
                vec![Some(2), None, None, Some(5), None],
                vec![None, Some(4), None, None, None],
                vec![None, None, None, None, None],
            ],
            None,
        )
    }

    fn problem_for_tests2() -> Problem {
        (
            true,
            vec![
                vec![None, Some(2), Some(1), None],
                vec![None, None, None, None],
                vec![None, None, None, None],
                vec![Some(2), Some(1), None, None],
            ],
            None,
        )
    }

    fn problem_for_tests3() -> Problem {
        (
            false,
            vec![vec![None, None], vec![None, Some(2)], vec![None, None]],
            Some(graph::InnerGridEdges {
                horizontal: crate::util::tests::to_bool_2d([[1, 0], [0, 0]]),
                vertical: crate::util::tests::to_bool_2d([[1], [1], [0]]),
            }),
        )
    }

    #[test]
    fn test_fillomino_problem1() {
        let (max3, problem, borders) = problem_for_tests1();
        let ans = solve_fillomino(max3, &problem, &borders);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_2d([
            [6, 1, 3, 3, 4],
            [6, 6, 3, 4, 4],
            [2, 6, 6, 5, 4],
            [2, 4, 6, 5, 5],
            [4, 4, 4, 5, 5],
        ]);
        assert_eq!(ans.0, expected);
    }

    #[test]
    fn test_fillomino_problem2() {
        let (max3, problem, borders) = problem_for_tests2();
        let ans = solve_fillomino(max3, &problem, &borders);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_2d([
            [2, 2, 1, 2],
            [1, 3, 3, 2],
            [2, 3, 1, 3],
            [2, 1, 3, 3],
        ]);
        assert_eq!(ans.0, expected);
    }

    #[test]
    fn test_fillomino_problem3() {
        let (max3, problem, borders) = problem_for_tests3();
        let ans = solve_fillomino(max3, &problem, &borders);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_2d([[1, 2], [3, 2], [3, 3]]);
        assert_eq!(ans.0, expected);
    }

    #[test]
    fn test_fillomino_serializer() {
        {
            let problem = problem_for_tests1();
            let url = "https://pzprxs.vercel.app/p?fillomino/5/5/g1k34g2h5h4n";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
        /*
        {
            let problem = problem_for_tests2();
            let url = "https://pzprxs.vercel.app/p?fillomino/t/4/4/g21o21h";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests3();
            let url = "https://pzprxs.vercel.app/p?fillomino/2/3/i2hog";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }*/
    }
}
