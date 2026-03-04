use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::inabas;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = inabas::deserialize_problem(url).ok_or("invalid url")?;
    let ans = inabas::solve_inabas(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));
    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                board.push(Item::cell(y, x, "black", ItemKind::Circle));
                if clue > 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                }
            } else if let Some(ans) = &ans {
                if let Some(a) = ans[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if a { ItemKind::Fill } else { ItemKind::Dot },
                    ));
                }
            }
        }
    }

    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::*;
    use crate::compare_board_and_check_no_solution_case;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board_and_check_no_solution_case!(
            solve("https://pzprxs.vercel.app/p?island/5/5/1g6m2i3h4m"),
            Board {
                kind: BoardKind::Grid,
                height: 5,
                width: 5,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Num(6) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Fill },
                    Item { y: 1, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 9, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Fill },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 5, x: 9, color: "black", kind: ItemKind::Num(3) },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Fill },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 5, color: "black", kind: ItemKind::Num(4) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 9, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 1, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 7, color: "green", kind: ItemKind::Dot },
                    Item { y: 9, x: 9, color: "green", kind: ItemKind::Dot },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
