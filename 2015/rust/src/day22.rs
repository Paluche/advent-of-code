use pathfinding::directed::dijkstra::dijkstra;

fn get_value(prefix: &str, line: &str) -> isize {
    line.strip_prefix(prefix)
        .unwrap()
        .parse::<isize>()
        .expect("")
}

fn parse_input(input: &str) -> (isize, isize) {
    let mut lines = input.lines();
    let hit_points = get_value("Hit Points: ", lines.next().unwrap());
    let damage = get_value("Damage: ", lines.next().unwrap());

    (hit_points, damage)
}

const PLAYER_HIT_POINTS: isize = 50;
const PLAYER_MANA_POINTS: isize = 500;

enum Spells {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spells {
    fn mana_cost(&self) -> isize {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    fn cast(&self) -> Effect {
        match self {
            Self::MagicMissile => Effect::MagicMissile,
            Self::Drain => Effect::Drain,
            Self::Shield => Effect::Shield(6),
            Self::Poison => Effect::Poison(6),
            Self::Recharge => Effect::Recharge(5),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Effect {
    MagicMissile,
    Drain,
    Shield(usize),
    Poison(usize),
    Recharge(usize),
}

impl Effect {
    fn apply(
        &self,
        player_hit_points: &mut isize,
        player_mana_points: &mut isize,
        player_armor: &mut isize,
        boss_hit_points: &mut isize,
    ) -> Option<Self> {
        match self {
            Self::MagicMissile => {
                *boss_hit_points -= 4;
                None
            }
            Self::Drain => {
                *boss_hit_points -= 2;
                *player_hit_points += 2;
                None
            }
            Self::Shield(x) => {
                *player_armor += 7;

                if *x == 1 {
                    None
                } else {
                    Some(Self::Shield(*x - 1))
                }
            }
            Self::Poison(x) => {
                *boss_hit_points -= 3;

                if *x == 1 {
                    None
                } else {
                    Some(Self::Poison(*x - 1))
                }
            }
            Self::Recharge(x) => {
                *player_mana_points += 101;
                if *x == 1 {
                    None
                } else {
                    Some(Self::Recharge(*x - 1))
                }
            }
        }
    }
}

fn damage_done(attacker_damage: isize, defender_armor: isize) -> isize {
    let ret = attacker_damage - defender_armor;

    if ret <= 0 {
        1
    } else {
        ret
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct FightContext {
    turn: usize,
    cost: isize,
    player_hit_points: isize,
    player_mana_points: isize,
    boss_hit_points: isize,
    boss_damage: isize,
    effects: Vec<Effect>,
}

impl FightContext {
    fn new(
        player_hit_points: isize,
        player_mana_points: isize,
        boss_hit_points: isize,
        boss_damage: isize,
    ) -> Self {
        Self {
            turn: 0,
            cost: 0,
            player_hit_points,
            player_mana_points,
            boss_hit_points,
            boss_damage,
            effects: Vec::new(),
        }
    }

    fn apply_cast(&mut self) -> isize {
        let mut player_armor = 0;
        self.effects = self
            .effects
            .iter()
            .filter_map(|effect| {
                effect.apply(
                    &mut self.player_hit_points,
                    &mut self.player_mana_points,
                    &mut player_armor,
                    &mut self.boss_hit_points,
                )
            })
            .collect();

        player_armor
    }

    fn effects_contains(&self, a: &Effect) -> bool {
        self.effects.iter().any(|b| match b {
            Effect::Shield(_) => matches!(a, Effect::Shield(_)),
            Effect::Poison(_) => matches!(a, Effect::Poison(_)),
            Effect::Recharge(_) => matches!(a, Effect::Recharge(_)),
            _ => a == b,
        })
    }

    fn successors(&self, hard: bool) -> Vec<(Self, isize)> {
        let mut fight_context = self.clone();

        if fight_context.turn > 0 {
            // Boss turns attacks.
            fight_context.turn += 1;
            let player_armor = fight_context.apply_cast();

            if fight_context.boss_hit_points <= 0 {
                // Boss died from the cast applied.
                return vec![(fight_context, 0)];
            }

            // Boss attacks.
            fight_context.player_hit_points -=
                damage_done(fight_context.boss_damage, player_armor);

            if fight_context.player_hit_points <= 0 {
                // Player died from the boss attack.
                return Vec::new();
            }
        }

        // Player turn
        fight_context.turn += 1;

        if hard {
            // At the start of each player turn (before any other effects
            // apply), you lose 1 hit point. If this brings you to or below 0
            // hit points, you lose.
            fight_context.player_hit_points -= 1;

            if fight_context.player_hit_points <= 0 {
                // Player died from the hard mode handicap.
                return Vec::new();
            }
        }

        fight_context.apply_cast();

        if fight_context.boss_hit_points <= 0 {
            // Boss died from the cast applied.
            vec![(fight_context, 0)]
        } else {
            // Cast a spell.
            [
                Spells::MagicMissile,
                Spells::Drain,
                Spells::Shield,
                Spells::Poison,
                Spells::Recharge,
            ]
            .iter()
            .filter_map(|spell| {
                let mana_cost = spell.mana_cost();
                if mana_cost > fight_context.player_mana_points {
                    // Not enough mana available to cast that spell.
                    None
                } else {
                    let effect = spell.cast();

                    if fight_context.effects_contains(&effect) {
                        // Cannot cast a spell that would start an effect which
                        // is already active.
                        None
                    } else {
                        let mut ret = fight_context.clone();
                        ret.effects.push(spell.cast());
                        ret.cost += mana_cost;
                        ret.player_mana_points -= mana_cost;

                        Some((ret, mana_cost))
                    }
                }
            })
            .collect()
        }
    }

    fn success(&self) -> bool {
        self.boss_hit_points <= 0
    }
}

fn run(input: &str, hard: bool) -> isize {
    let (boss_hit_points, boss_damage) = parse_input(input);

    dijkstra(
        &FightContext::new(
            PLAYER_HIT_POINTS,
            PLAYER_MANA_POINTS,
            boss_hit_points,
            boss_damage,
        ),
        |c| c.successors(hard),
        |c| c.success(),
    )
    .unwrap()
    .1
}

#[aoc(day22, part1)]
fn part1(input: &str) -> isize {
    run(input, false)
}

#[aoc(day22, part2)]
fn part2(input: &str) -> isize {
    run(input, true)
}
