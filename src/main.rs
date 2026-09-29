#![allow(dead_code, unused_variables)]

/// Entry point for Chapter 6.2: The `match` Control Flow Construct
///
/// `match` compares a value against a series of patterns. Every possible
/// value must be handled, which lets the compiler help us avoid forgetting a
/// case.
fn main() {
    matching_literals();
    binding_values();
    matching_options();
    catch_all_patterns();
}

/// A direction a character can move in the game world.
enum Direction {
    North,
    South,
    East,
    West,
}

/// A dice roll used when the player finds an item.
// `Copy` and `Clone` are traits that the DiceRoll implements. These mark the
// enum as clonable and copyable so this DiceRoll now behaves more like the
// primary scalar types. We will cover traits in more detail later during the
// course.
#[derive(Clone, Copy)]
enum DiceRoll {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

/// An action a player can take. Variants can hold different shapes of data
/// like we learned previously.
enum PlayerAction {
    Move(Direction),
    Attack { target: String, damage: u32 },
    Say(String),
    Quit,
}

/// # Matching Literal Patterns
/// - A `match` expression compares a value with the patterns in its arms.
/// - The code after `=>` runs for the first matching pattern.
/// - Each arm must be separated with a comma.
/// - `match` is exhaustive: every possible `Direction` variant needs an arm.
fn matching_literals() {
    println!("\n{:=>80}", "");
    println!("matching_literals()\n");

    let direction = Direction::West;

    match direction {
        Direction::North => println!("The hero walks north."),
        Direction::South => println!("The hero walks south."),
        Direction::East => println!("The hero walks east."),
        Direction::West => println!("The hero walks west."),
    }

    // Removing an arm causes a compiler error because the missing direction
    // could otherwise be left unhandled.
}

/// # Binding Values in Match Arms
/// - A pattern can bind data stored in an enum variant to a variable.
/// - The bound name is available in the arm's expression or block.
/// - Arms may contain a single expression or a block of statements.
fn binding_values() {
    println!("\n{:=>80}", "");
    println!("binding_values()\n");

    let action = PlayerAction::Attack {
        target: String::from("Spooderman"),
        damage: 12,
    };

    // `match` tries the arms from top to bottom. First, each pattern checks
    // whether `action` has that variant. A name in a matching pattern then
    // binds the data carried by that variant for use in that arm only.
    match action {
        // This arm runs only for `PlayerAction::Move`. If it matched,
        // `direction` would be bound to the direction stored inside `Move`.
        PlayerAction::Move(direction) => {
            println!("The hero moves {}.", direction_name(direction));
        }
        // `action` is `Attack`, so this is the matching arm. The named fields
        // are destructured: `target` and `damage` bind to this attack's data.
        PlayerAction::Attack { target, damage } => {
            println!("The hero attacks {target} for {damage} damage.");
        }
        // This pattern binds the string stored in `Say` to `message`.
        PlayerAction::Say(message) => println!("The hero says: \"{message}\""),
        PlayerAction::Quit => println!("The hero leaves the game."),
    }
}

/// # Matching `Option<T>`
/// - `Option<T>` represents either `Some(value)` or `None`.
/// - Matching `Some(value)` both checks the variant and binds its inner value.
/// - Because `Option<T>` has two variants, both must be handled.
fn matching_options() {
    println!("\n{:=>80}", "");
    println!("matching_options()\n");

    let potion_healing = Some(25_u32);
    let empty_chest: Option<u32> = None;

    println!("Potion result: {}", describe_healing_potion(potion_healing));
    println!("Chest result: {}", describe_healing_potion(empty_chest));
}

/// Returns a message for a potion amount or an empty inventory slot.
fn describe_healing_potion(amount: Option<u32>) -> String {
    match amount {
        Some(points) => format!("The potion restores {points} health."),
        None => String::from("There is no potion here."),
    }
}

/// # Catch-All Patterns
/// - Use `_` when the remaining values should all have the same behavior.
/// - `_` matches anything and does not bind the matched value.
/// - Catch-all arms must come last because earlier matching arms are chosen.
fn catch_all_patterns() {
    println!("\n{:=>80}", "");
    println!("catch_all_patterns()\n");

    let roll = DiceRoll::Three;

    match roll {
        DiceRoll::Five => println!("You found a rare treasure!"),
        DiceRoll::Six => println!("You found a legendary treasure!"),
        _ => println!("Nothing special happens."),
    }

    // An exhaustive `match` can also return a value.
    let score = match roll {
        DiceRoll::Six => 100,
        DiceRoll::Five => 50,
        _ => 10,
    };
    println!("The roll is worth {score} points.");
}

/// Returns the lowercase name of a direction.
fn direction_name(direction: Direction) -> &'static str {
    match direction {
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
    }
}
