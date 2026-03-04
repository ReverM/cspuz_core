use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::usoone;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let (borders, clues) = usoone::deserialize_problem(url).ok_or("invalid url")?;
    let ans = usoone::solve_usoone(&borders, &clues);

    let height = clues.len();
    let width = clues[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    board.add_borders(&borders, "black");

    for y in 0..height {
        for x in 0..width {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    board.push(Item::cell(y, x, "black", ItemKind::Num(n)));
                } else {
                    board.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
                if let Some((_, liar)) = &ans {
                    if let Some(l) = liar[y][x] {
                        board.push(Item::cell(
                            y,
                            x,
                            "green",
                            if l { ItemKind::Cross } else { ItemKind::Circle },
                        ));
                    }
                }
            } else if let Some((shaded, _)) = &ans {
                if let Some(b) = shaded[y][x] {
                    board.push(Item::cell(
                        y,
                        x,
                        "green",
                        if b { ItemKind::Block } else { ItemKind::Dot },
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
            solve("https://puzz.link/p?usoone/4/4/94g1g0c1cj105b"),
            Board {
                kind: BoardKind::Grid,
                height: 4,
                width: 4,
                data: vec![
                    Item { y: 1, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 3, color: "black", kind: ItemKind::BoldWall },                    
                    Item { y: 3, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::Num(2) }, 
                    Item { y: 1, x: 1, color: "green", kind: ItemKind::Cross }, 
                    Item { y: 1, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 1, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 1, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::Num(2) },
                    Item { y: 3, x: 1, color: "green", kind: ItemKind::Circle },
                    Item { y: 3, x: 3, color: "green", kind: ItemKind::Block },
                    Item { y: 3, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 3, x: 7, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 1, color: "green", kind: ItemKind::Block },
                    Item { y: 5, x: 3, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 5, x: 7, color: "green", kind: ItemKind::Circle },
                    Item { y: 7, x: 1, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 7, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Num(0) },
                    Item { y: 7, x: 3, color: "green", kind: ItemKind::Circle },
                    Item { y: 7, x: 5, color: "green", kind: ItemKind::Dot },
                    Item { y: 7, x: 7, color: "black", kind: ItemKind::Num(1) },
                    Item { y: 7, x: 7, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
