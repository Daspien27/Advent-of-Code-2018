use regex::{Regex};
use std::collections::HashMap;
use std::cmp::Reverse;

const PRINT_DEBUG : bool = false;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum DamageType
{
    Radiation,
    Cold,
    Bludgeoning,
    Fire,
    Slashing,
}

impl From<&str> for DamageType
{
    fn from(s : &str) -> DamageType
    {
        match s
        {
            "radiation" => DamageType::Radiation,
            "cold" => DamageType::Cold,
            "bludgeoning" => DamageType::Bludgeoning,
            "fire" => DamageType::Fire,
            "slashing" => DamageType::Slashing,
            _ => panic!("Unrecognized damage type!")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResistanceLevel
{
    Normal,
    Weak,
    Immune,
}

fn parsed_resistances(resistances: Option<&str>) -> HashMap<DamageType, ResistanceLevel>
{
    let mut map = HashMap::new();

    lazy_static! {
        static ref RESISTANCE_RE : Regex = Regex::new(r"(?P<resist_type>weak|immune) to (?P<types>\w+(?:, \w+)*)").unwrap();
        static ref LIST_RE : Regex = Regex::new(r"(\w+)").unwrap();
    }
    
    
        resistances
            .iter()
            .for_each(|r|{
                RESISTANCE_RE.captures_iter(r)
                .for_each(|cap|{
                    let resist_type = match cap.name("resist_type").unwrap().as_str()
                    {
                        "weak" => ResistanceLevel::Weak,
                        "immune" => ResistanceLevel::Immune,
                        _ => panic!("Unexpected resistance!")
                    };

                    LIST_RE
                        .captures_iter(cap.name("types").unwrap().as_str())
                        .for_each(|x|{
                            map.insert(DamageType::from(x.get(0).unwrap().as_str()), resist_type);
                        })
                })
        });
    
    map
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Attack
{
    value: u64,
    damage_type: DamageType
}

impl Attack
{
    fn parse(damage: u64, damage_type: &str) -> Attack
    {
        Attack{value: damage, damage_type: DamageType::from(damage_type)}
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Squad
{
    id: usize,
    count: u64,
    hitpoints: u64,
    resistances : HashMap<DamageType, ResistanceLevel>,
    attack: Attack,
    initiative: u64
}



impl Squad
{
    fn from_parse(id: usize, units: u64, hp: u64, resistances: Option<&str>, damage: u64, damage_type: &str, initiative: u64) -> Squad
    {
        Squad{id: id,
              count: units,
              hitpoints : hp,
              resistances: parsed_resistances(resistances),
              attack: Attack::parse(damage, damage_type),
              initiative: initiative
             }
    }

    fn effective_power(&self) -> u64
    {
        self.count * self.attack.value
    }

    fn damage(&mut self, amount : u64) -> u64 {
        let dead_units = amount / self.hitpoints;

        if self.count < dead_units {
            let original_count = self.count;
            self.count = 0;
            original_count
            
        }
        else {
            self.count -= dead_units;
            dead_units
        }
    }
    
    fn attacked_by(&mut self, enemy : &Squad) -> u64 {
        match self.resistances.get(&enemy.attack.damage_type).unwrap_or(&ResistanceLevel::Normal)
        {
            ResistanceLevel::Immune => 0,
            ResistanceLevel::Weak => {
                self.damage(2 * enemy.effective_power())
            },
            ResistanceLevel::Normal => {
                self.damage(enemy.effective_power())
            }
        }
    }
}

fn parse_squads(squads_str: &str) -> Vec<Squad>
{
    lazy_static! {
        static ref SQUAD_RE : Regex = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points (\((?P<resistances>(?:(?:weak|immune) to \w+(?:, \w+)*(?:; )?)+)\) )?with an attack that does (?P<damage>\d+) (?P<damage_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
    }

    SQUAD_RE
        .captures_iter(squads_str)
        .enumerate()
        .map(|(id, cap) | {
            Squad::from_parse(id + 1, cap.name("units").unwrap().as_str().parse().unwrap(),
                            cap.name("hp").unwrap().as_str().parse().unwrap(),
                            cap.name("resistances").map(|r| r.as_str()),
                            cap.name("damage").unwrap().as_str().parse().unwrap(),
                            cap.name("damage_type").unwrap().as_str(),
                            cap.name("initiative").unwrap().as_str().parse().unwrap()
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Army{
    name: String,
    packs: Vec<Squad>,
}

impl Army{
    fn army_size(&self) -> u64{
        self.packs.iter().map(|s| s.count).sum()
    }

    fn boost(&mut self, amount : u64){
        self.packs.iter_mut().for_each(|p| p.attack.value += amount);
    }

}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Army>
{
    lazy_static! {
        static ref ARMY_RE : Regex = Regex::new(r"(?m)(?:^(?P<name>^.+):$\n(?P<squads>(?:^.+$\n?)+))*").unwrap();
    }

    ARMY_RE
        .captures_iter(input)
        .map(|cap|{
            Army{name: String::from(cap.name("name").unwrap().as_str()),
                 packs: parse_squads(cap.name("squads").unwrap().as_str())
                }
        })
        .collect()
}


fn print_group_count(battle: (&Army, &Army))
{
    let print_army = |a : &Army|{
        println!("{}:", a.name);

        if a.packs.len() > 0 {
            a.packs.iter().for_each(|s|{
                println!("Group {} contains {} units", s.id, s.count);
            });
        }
        else {
            println!("No groups remain.");
        }
    };

    print_army(&battle.0);
    print_army(&battle.1);
}


fn predict_damage(attacking_group: &Squad, defending_group: &Squad) -> u64
{
    match defending_group.resistances.get(&attacking_group.attack.damage_type).unwrap_or(&ResistanceLevel::Normal)
    {
        ResistanceLevel::Immune => 0,
        ResistanceLevel::Weak   => 2 * attacking_group.effective_power(),
        ResistanceLevel::Normal => attacking_group.effective_power()
    }
}

#[derive(Debug)]
enum ParentArmy
{
    Army1(usize),
    Army2(usize),
}

impl ParentArmy
{
    fn army1(&self) -> Option<usize>{
        match self {
            ParentArmy::Army1(id) => Some(*id),
            _ => None
        }
    }

    fn army2(&self) -> Option<usize>{
        match self {
            ParentArmy::Army2(id) => Some(*id),
            _ => None
        }
    }
}


fn attacking_army_targets (attacking_army : &Army, defending_army : &Army) -> Vec<(usize, usize)> {
    let mut defending_army_targets : Vec<Option<usize>> = vec![None; defending_army.packs.len()];

    let mut attacking_army_groups : Vec<_> = attacking_army.packs
                                                        .iter()
                                                        .enumerate()
                                                        .collect();
    
    attacking_army_groups.sort_by_cached_key(|(_, g)| {
        (Reverse(g.effective_power()), Reverse(g.initiative))
    });

    attacking_army_groups
            .iter()
            .for_each(|(attacking_group_idx, attacking_group)|{
                let mut targets : Vec<_> = defending_army.packs
                        .iter()
                        .enumerate()
                        .filter(|(defending_group_idx, _)| defending_army_targets[*defending_group_idx].is_none())
                        .map(|(defending_group_idx, defending_group)|{
                            let predicted_damage = predict_damage(&attacking_group, &defending_group);
                            if PRINT_DEBUG {println!("{} group {} would deal defending group {} {} damage", attacking_army.name, attacking_group.id, defending_group.id, predict_damage(&attacking_group, &defending_group));}
                            (defending_group, defending_group_idx, predicted_damage)
                        })
                        .filter(|(_,_, predicted_damage)| *predicted_damage > 0)
                        .collect();
                
                targets.sort_by_cached_key(|(defending_group, _, predicted_damage)|{
                    (Reverse(*predicted_damage), Reverse(defending_group.effective_power()), Reverse(defending_group.initiative))
                });

                let group_to_attack = targets.first();

                if let Some((_, defending_group_idx, _)) = group_to_attack
                {
                   defending_army_targets[*defending_group_idx] = Some(*attacking_group_idx);
                }
            });

    defending_army.packs
        .iter()
        .enumerate()
        .zip(defending_army_targets.iter())
        .filter(|(_, attacking_group)| attacking_group.is_some())
        .map(|((defending_group_id, _), attacking_group)| (defending_group_id, attacking_group.unwrap()))
        .collect()
}

fn target_selection(army1: &Army, army2: &Army) -> Vec<(ParentArmy, ParentArmy)>
{
    let army1_targets = attacking_army_targets(&army1, &army2);
    let army2_targets = attacking_army_targets(&army2, &army1);

    let mut chained_decisions : Vec<_> = army1_targets
                                               .iter()
                                               .map(|(defender, attacker)| (ParentArmy::Army2(*defender), ParentArmy::Army1(*attacker)))
                                        .chain(
                                            army2_targets
                                               .iter()
                                               .map(|(defender, attacker)| (ParentArmy::Army1(*defender), ParentArmy::Army2(*attacker)))
                                        )
                                        .collect();

    chained_decisions.sort_by_key(|(_, attacking_group)| {
        match attacking_group {
            ParentArmy::Army1(id) => Reverse(army1.packs[*id].initiative),
            ParentArmy::Army2(id) => Reverse(army2.packs[*id].initiative),
    }});

    chained_decisions
}

fn battle(armies: (&mut Army, &mut Army)) -> Option<(String, u64)> {
    loop
    {
        
        if PRINT_DEBUG {
            print_group_count((armies.0, armies.1));
            println!("");
        }

        let army_0_count : u64 = armies.0.army_size();
        let army_1_count : u64 = armies.1.army_size();
        if army_0_count == 0 { return Some((armies.1.name.clone(), army_1_count)); }
        if army_1_count == 0 { return Some((armies.0.name.clone(), army_0_count)); }

        let schedule = target_selection(&armies.0, &armies.1);

        if PRINT_DEBUG {println!("");}

        let total_killed : u64 = schedule.iter().map(|(defender, attacker)|{
            match defender{
                ParentArmy::Army1(idx) => {
                    let defending_group = &mut armies.0.packs[*idx];
                    let attacking_group = &armies.1.packs[attacker.army2().unwrap()];
                    let killed_units = defending_group.attacked_by(&attacking_group);
                    if PRINT_DEBUG {println!("{} group {} attacks defending group {}, killing {} units", armies.1.name, attacking_group.id, defending_group.id, killed_units);}
                    killed_units

                },
                ParentArmy::Army2(idx) => {
                    let defending_group = &mut armies.1.packs[*idx];
                    let attacking_group = &armies.0.packs[attacker.army1().unwrap()];
                    let killed_units = defending_group.attacked_by(&attacking_group);
                    if PRINT_DEBUG {println!("{} group {} attacks defending group {}, killing {} units", armies.0.name, attacking_group.id, defending_group.id, killed_units);}
                    killed_units
                },
            }
        })
        .sum();

        if total_killed == 0 {
            return None; //there is a stand off
        }

        armies.0.packs.retain(|x| x.count > 0);
        armies.1.packs.retain(|x| x.count > 0);

        if PRINT_DEBUG {println!("");}
    }
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Vec<Army>) -> u64 {
    
    assert_eq!(input.len(), 2);

    let mut immune_system = input[0].clone();
    let mut infection = input[1].clone();
    battle((&mut immune_system, &mut infection)).unwrap().1
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Vec<Army>) -> u64 {
    
    assert_eq!(input.len(), 2);

    let mut boost_seed = 1;
    loop {
        let mut immune_system = input[0].clone();
        immune_system.boost(boost_seed);
        
        let mut infection = input[1].clone();

        let battle_result = battle((&mut immune_system, &mut infection));


        if let Some(result) = battle_result
        {
            if PRINT_DEBUG {
                println!("boost at {}: {:?}", boost_seed, result);
            }

            if result.0 == immune_system.name {
                return result.1;
            }
        }

        boost_seed += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squad_parse_example_1() {

        let squads = parse_squads("6638 units each with 2292 hit points (weak to radiation) with an attack that does 3 cold damage at initiative 18");
        
        assert_eq!(squads.len(), 1);

        let expected_squad = Squad{
            id: 1,
            count: 6638,
            hitpoints: 2292,
            resistances: [(DamageType::Radiation, ResistanceLevel::Weak)].iter().cloned().collect(),
            attack: Attack{value: 3, damage_type: DamageType::Cold},
            initiative: 18,
        };

        assert_eq!(*squads.first().unwrap(), expected_squad);
    }

    #[test]
    fn squad_parse_example_2() {

        let squads = parse_squads("20 units each with 1333 hit points (immune to radiation, slashing; weak to bludgeoning) with an attack that does 508 fire damage at initiative 3");
        
        assert_eq!(squads.len(), 1);

        let expected_squad = Squad{
            id: 1,
            count: 20,
            hitpoints: 1333,
            resistances: [(DamageType::Radiation, ResistanceLevel::Immune), 
                          (DamageType::Slashing, ResistanceLevel::Immune), 
                          (DamageType::Bludgeoning, ResistanceLevel::Weak)                          
                         ].iter().cloned().collect(),
            attack: Attack{value: 508, damage_type: DamageType::Fire},
            initiative: 3,
        };

        assert_eq!(*squads.first().unwrap(), expected_squad);
    }

    #[test]
    fn squad_parse_example_3() {

        let squads = parse_squads("1017 units each with 10287 hit points with an attack that does 88 cold damage at initiative 1");
        
        assert_eq!(squads.len(), 1);

        let expected_squad = Squad{
            id: 1,
            count: 1017,
            hitpoints: 10287,
            resistances: [                       
                         ].iter().cloned().collect(),
            attack: Attack{value: 88, damage_type: DamageType::Cold},
            initiative: 1,
        };

        assert_eq!(*squads.first().unwrap(), expected_squad);
    }
}