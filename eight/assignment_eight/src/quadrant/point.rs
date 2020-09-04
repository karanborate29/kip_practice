use crate::quadrant::point::Position::{First, Fourth, Second, Third, X, Y};

#[derive(Debug)]
enum Position {
    First,
    Second,
    Third,
    Fourth,
    X,
    Y,
}

pub fn quadrant_matching() {
    let input: [(i32, i32); 5] = [(0, 2), (2, 0), (-2, -3), (5, 6), (4, -4)];

    for element in &input {
        if element.0 > 0 && element.1 > 0 {
            let quadrant: Position = Position::search_quadrant(&Position::First, element);
            println!(
                "{:?}::Coordinate::Abscissa{:?} Coordinate::Ordinate{:?}",
                quadrant, element.0, element.1
            )
        }
        if element.0 < 0 && element.1 > 0 {
            let quadrant: Position = Position::search_quadrant(&Position::Second, element);
            println!(
                "{:?}::Coordinate::Abscissa{:?} Coordinate::Ordinate{:?}",
                quadrant, element.0, element.1
            )
        }
        if element.0 < 0 && element.1 < 0 {
            let quadrant: Position = Position::search_quadrant(&Position::Third, element);
            println!(
                "{:?}::Coordinate::Abscissa{:?} Coordinate::Ordinate{:?}",
                quadrant, element.0, element.1
            )
        }
        if element.0 > 0 && element.1 < 0 {
            let quadrant: Position = Position::search_quadrant(&Position::Fourth, element);
            println!(
                "{:?}::Coordinate::Abscissa{:?} Coordinate::Ordinate{:?}",
                quadrant, element.0, element.1
            )
        }
        if element.0 > 0 && element.1 == 0 {
            let quadrant: Position = Position::search_quadrant(&Position::X, element);
            println!(
                "{:?}::Coordinate::Abscissa{:?} Coordinate::Ordinate{:?}",
                quadrant, element.0, element.1
            )
        }
        if element.0 == 0 && element.1 > 0 {
            let quadrant: Position = Position::search_quadrant(&Position::Y, element);
            println!(
                "{:?}::Coordinate::Abscissa{:?} Coordinate::Ordinate{:?}",
                quadrant, element.0, element.1
            )
        }
    }
    println!();
}

impl Position {
    fn search_quadrant(&self, _element: &(i32, i32)) -> Position {
        match &self {
            Position::First => First,
            Position::Second => Second,
            Position::Third => Third,
            Position::Fourth => Fourth,
            Position::X => X,
            Position::Y => Y,
        }
    }
}
