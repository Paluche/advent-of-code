fn get_value(prefix: &str, line: &str) -> isize {
    line.strip_prefix(prefix)
        .unwrap()
        .parse::<isize>()
        .expect("")
}

fn parse_input(input: &str) -> (isize, isize, isize) {
    let mut lines = input.lines();
    let hit_points = get_value("Hit Points: ", lines.next().unwrap());
    let damage = get_value("Damage: ", lines.next().unwrap());
    let armor = get_value("Armor: ", lines.next().unwrap());

    (hit_points, damage, armor)
}

fn damage_done(attacker_damage: isize, defender_armor: isize) -> isize {
    let ret = attacker_damage - defender_armor;

    if ret <= 0 {
        1
    } else {
        ret
    }
}

fn fight(
    player_hit_points: isize,
    player_damage: isize,
    player_armor: isize,
    boss_hit_points: isize,
    boss_damage: isize,
    boss_armor: isize,
) -> bool {
    // Could be improved....

    let mut player_hit_points = player_hit_points;
    let mut boss_hit_points = boss_hit_points;
    let mut player_turn = true;

    let player_damage = damage_done(player_damage, boss_armor);
    let boss_damage = damage_done(boss_damage, player_armor);

    while player_hit_points > 0 && boss_hit_points > 0 {
        if player_turn {
            boss_hit_points -= player_damage;
        } else {
            player_hit_points -= boss_damage;
        }

        player_turn = !player_turn;
    }

    player_hit_points > 0
}

// The items are sorted by cost.
const WEAPONS: [(usize, isize, isize); 5] = [
    (8, 4, 0),  // Dagger
    (10, 5, 0), // Shortsword
    (25, 6, 0), // Warhammer
    (40, 7, 0), // Longsword
    (74, 8, 0), // Greataxe
];

const ARMORS: [Option<(usize, isize, isize)>; 6] = [
    None,
    Some((13, 0, 1)),  // Leather
    Some((31, 0, 2)),  // Chainmail
    Some((53, 0, 3)),  // Splintmail
    Some((75, 0, 4)),  // Bandedmail
    Some((102, 0, 5)), // Platemail
];

const RINGS: [Option<(usize, isize, isize)>; 7] = [
    None,
    Some((20, 0, 1)),  // Defense +1
    Some((25, 1, 0)),  // Damage +1
    Some((40, 0, 2)),  // Defense +2
    Some((50, 2, 0)),  // Damage +2
    Some((80, 0, 3)),  // Defense +3
    Some((100, 3, 0)), // Damage +3
];

const PLAYER_HIT_POINTS: isize = 100;

fn buy(
    cost: &mut usize,
    (item_cost, item_damage, item_armor): (usize, isize, isize),
    player_damage: &mut isize,
    player_armor: &mut isize,
) {
    *cost += item_cost;
    *player_damage += item_damage;
    *player_armor += item_armor;
}

fn run(input: &str, part2: bool) -> usize {
    let (boss_hit_points, boss_damage, boss_armor) = parse_input(input);

    let mut ret = if part2 { usize::MIN } else { usize::MAX };

    // Get one weapon.
    for weapon in WEAPONS {
        let mut cost = 0;
        let mut player_damage = 0;
        let mut player_armor = 0;

        buy(&mut cost, weapon, &mut player_damage, &mut player_armor);

        // Get zero or one armor
        for armor in ARMORS {
            let mut cost = cost;
            let mut player_damage = player_damage;
            let mut player_armor = player_armor;

            if let Some(armor) = armor {
                buy(&mut cost, armor, &mut player_damage, &mut player_armor);
            }

            // Get zero one or two rings
            for ring_1 in RINGS {
                let mut cost = cost;
                let mut player_damage = player_damage;
                let mut player_armor = player_armor;

                if let Some(ring_1) = ring_1 {
                    buy(
                        &mut cost,
                        ring_1,
                        &mut player_damage,
                        &mut player_armor,
                    );
                }

                for ring_2 in RINGS {
                    let mut cost = cost;
                    let mut player_damage = player_damage;
                    let mut player_armor = player_armor;

                    if let Some(ring_2) = ring_2 {
                        let do_buy = if let Some(ring_1) = ring_1 {
                            ring_1 != ring_2
                        } else {
                            true
                        };

                        if do_buy {
                            buy(
                                &mut cost,
                                ring_2,
                                &mut player_damage,
                                &mut player_armor,
                            );
                        }
                    }
                    let res = fight(
                        PLAYER_HIT_POINTS,
                        player_damage,
                        player_armor,
                        boss_hit_points,
                        boss_damage,
                        boss_armor,
                    );

                    if part2 {
                        if !res {
                            ret = ret.max(cost);
                        }
                    } else if res {
                        ret = ret.min(cost);
                    }
                }
            }
        }
    }
    ret
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    run(input, false)
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    run(input, true)
}
