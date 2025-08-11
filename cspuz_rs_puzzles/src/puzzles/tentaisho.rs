use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Map, NumSpaces, Size, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_tentaisho(
    stars: &[Vec<bool>],
    cells: &[Vec<Option<i32>>]
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h,w) = util::infer_shape(cells);

    let mut solver = Solver::new();
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));

    let mut star_pos = vec![];
    for y in 0..(2*h + 1) {
        for x in 0..(2*w + 1) {
            if 1 == stars[y][x] {
                star_pos.push((y, x));
            }
        }
    }
    
    let group_id = solver.int_var_2d((h, w), 0, star_pos.len() as i32 - 1);
    solver.add_expr(
        edges.horizontal.iff(
            group_id
                .slice((..(h - 1), ..))
                .ne(group_id.slice((1.., ..))),
        ),
    );
    solver.add_expr(
        edges.vertical.iff(
            group_id
                .slice((.., ..(w - 1)))
                .ne(group_id.slice((.., 1..))),
        ),
    );

    for(i, &(y, x)) in star_pos.iter().enumerate() {
        graph::active_vertices_connected_2d(&mut solver, group_id.eq(i as i32));
        if y%2 == 0 {
            if x%2 == 0 {

            }
            else {

            }
        }
        else {
            if x%2 == 0 {

            }
            else {

            }
        }
    }

}