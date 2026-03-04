use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, NumSpaces, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_usoone(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<(Vec<Vec<Option<bool>>>, Vec<Vec<Option<bool>>>)> {
    let (h, w) = util::infer_shape(clues);
    let mut solver = Solver::new();

    let rooms = graph::borders_to_rooms(borders);

    let is_black = &solver.bool_var_2d((h, w));
    solver.add_expr(!is_black.conv2d_and((1, 2)));
    solver.add_expr(!is_black.conv2d_and((2, 1)));
    let is_liar = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    solver.add_answer_key_bool(is_liar);
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x)));
                if n < 0 {
                    return None;
                } else {
                    solver.add_expr(
                        !is_liar
                            .at((y, x))
                            .iff(is_black.four_neighbors((y, x)).count_true().eq(n)),
                    );
                }
            } else {
                solver.add_expr(!is_liar.at((y, x)));
            }
        }
    }

    for room in &rooms {
        solver.add_expr(is_liar.select(room).count_true().eq(1));
    }

    graph::active_vertices_connected_2d(&mut solver, !is_black);

    solver
        .irrefutable_facts()
        .map(|f| (f.get(is_black), f.get(is_liar)))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(NumSpaces::new(4, 2)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "usoone",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["usoone"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([[0, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]]),
            vertical: crate::util::tests::to_bool_2d([[0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0]]),
        };

        let clues = vec![
            vec![Some(2), None, None, Some(1)],
            vec![Some(2), None, None, None],
            vec![None, None, None, Some(1)],
            vec![Some(0), Some(0), None, Some(1)],
        ];

        (borders, clues)
    }

    #[test]
    fn test_usoone_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_usoone(&borders, &clues);
        assert!(ans.is_some());
        let (shaded, _) = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 0, 0, 0],
            [0, 1, 0, 1],
            [1, 0, 0, 0],
            [0, 0, 0, 0],
        ]);
        assert_eq!(shaded, expected);
    }

    #[test]
    fn test_usoone_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?usoone/4/4/94g1g0c1cj1051";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
