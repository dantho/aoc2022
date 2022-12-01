/// https://adventofcode.com/2021/day/21
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day21)]
pub fn gen1(_input: &str) -> () {
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day21, part1)]
pub fn part1(_input: &()) -> usize {
    let my_puzzle_input = (7,3);
    deterministic_play (100,my_puzzle_input)
    // 906093 is the answer!
}

#[aoc(day21, part2)]
pub fn part2(_input: &()) -> usize {
    let starting_positions = (7,3);
    let win_count = dirac_play(starting_positions);
    let ans = win_count[0].max(win_count[1]);
    ans

}

pub fn dirac_play(starting_positions: (usize, usize)) -> [usize;2] {
    const WINNING_SCORE: usize = 21;
    const BOARD_SIZE: usize = 10;
    // Total move distance and universe count from 3 rolls of the Dirac Dice
    let rollx3 = [[3,4,5,6,7,8,9], // 7 unique roll totals
                  [1,3,6,7,6,3,1]]; // universe count per roll total (sums to 27)
    // player postions (x2), universe count per player, player scores
    let initial_game_state = [[starting_positions.0, starting_positions.1], [1, 0], [0, 0]];
    let mut win_count = [0,0];
    let mut game_state = vec![initial_game_state];
    while !game_state.is_empty() {
        for player in 0..=1 {
            if game_state.len() < 50 { 
                for game in &game_state {
                    println!("{:?}",game);
                }
                println!();
            }
            game_state = game_state.into_iter()
            .map(|[pos, [universes,_], score]| {
                (0..rollx3[0].len())
                .map(|roll_ndx| {
                    let (mut new_pos, mut new_univ, mut new_score)
                        = (pos,universes,score);
                    new_pos[player] = (pos[player] + rollx3[0][roll_ndx] - 1)%BOARD_SIZE+1;
                    new_univ = universes * rollx3[1][roll_ndx];
                    new_score[player] = score[player] + new_pos[player];

                    [new_pos,[new_univ,0],new_score]
                }).collect::<Vec<_>>()
            }).flatten().collect();
            // count winning games
            win_count[player] += game_state.iter()
            .fold(0,|winners,[_,[universes,_],score]| {
                if score[player] >= WINNING_SCORE {
                    winners + universes // A winner in each universe!
                } else {
                    winners // no new winners
                }
            });
            // remove winning games
            game_state = game_state.into_iter()
            .filter(|[_,_,score]| {
                assert!(score[1-player]<WINNING_SCORE);
                score[player] < WINNING_SCORE
            }).collect();
        }
    }
    win_count
}

pub fn deterministic_play(max_roll: usize, starting_pos: (usize, usize)) -> usize {
    const WINNING_SCORE: usize = 1000;
    const BOARD_SIZE: usize = 10;
    let mut player_pos = [starting_pos.0, starting_pos.1];
    let mut scores = [0; 2];
    let mut die = 0usize; // Not valid roll - initialization only
    let mut roll_count = 0usize;
    'game_loop:
    loop {
        for player in 0..=1 {
            let roll_x3: usize = ((0..3).into_iter().map(|_|{
                roll_count += 1;
                die = die % max_roll + 1;
                die
            }).sum::<usize>() - 1) % max_roll + 1;
            let pos = &mut player_pos[player];
            *pos = ((*pos + roll_x3) - 1) % BOARD_SIZE + 1;
            scores[player] += *pos;
            println!("Player {} die-modulo-rolls {} and moves to space {} for a total score of {}.", player+1, roll_x3, *pos, scores[player]); 
            if scores[player] >= WINNING_SCORE {break 'game_loop};
        }
    }
    let winner = if scores[0] >= WINNING_SCORE {0} else {1};
    let loser = 1-winner;
    assert!(scores[winner] >= WINNING_SCORE && scores[loser]<WINNING_SCORE);
    println!{"WINNER --> Player {} with a score of {}.",winner+1,scores[winner]}; 
    println!{"Loser ---> Player {} had a score of {}.",loser+1,scores[loser]}; 
    println!("The die was rolled {} times", roll_count);

    roll_count * scores[loser]
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_deterministic_play() {
        let res = deterministic_play(100,(4,8));
        assert_eq!(res, 739785);
    }

    #[test]
    fn test_ex1_dirac_play() {
        let res = dirac_play((4,8));
        assert_eq!(res, [444356092776315, 341960390180808]);
    }

}