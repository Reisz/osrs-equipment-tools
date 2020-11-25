use enum_iterator::IntoEnumIterator;

use super::ExprMap;
use crate::{
    bool_expr::{BoolExpr, BoolExprBuilder},
    vars::{Region, RegionCombination},
};

fn freeing_awowogei(c: &RegionCombination) -> bool {
    c[Region::Kandarin]
}

fn freeing_mountain_dwarf(c: &RegionCombination) -> bool {
    c[Region::Asgarnia] && c[Region::Kandarin]
}

fn freeing_goblin_generals(c: &RegionCombination) -> bool {
    c[Region::Asgarnia] && (c[Region::Kandarin] || c[Region::Morytania])
}

fn freeing_pirate_pete(c: &RegionCombination) -> bool {
    c[Region::Kandarin] || c[Region::Morytania]
}

fn freeing_lumbridge_guide(c: &RegionCombination) -> bool {
    c[Region::Asgarnia] && c[Region::Kandarin]
}

fn freeing_evil_dave(c: &RegionCombination) -> bool {
    c[Region::Desert] && (c[Region::Kandarin] || c[Region::Morytania])
}

fn freeing_sir_amik_varze(c: &RegionCombination) -> bool {
    c[Region::Asgarnia] && c[Region::Fremennik] && (c[Region::Kandarin] || c[Region::Morytania])
}

fn freeing_skrach_uglogwee(c: &RegionCombination) -> bool {
    c[Region::Kandarin]
}

fn count_subquests(c: &RegionCombination) -> u8 {
    freeing_awowogei(c) as u8
        + freeing_mountain_dwarf(c) as u8
        + freeing_goblin_generals(c) as u8
        + freeing_pirate_pete(c) as u8
        + freeing_lumbridge_guide(c) as u8
        + freeing_evil_dave(c) as u8
        + freeing_sir_amik_varze(c) as u8
        + freeing_skrach_uglogwee(c) as u8
}

#[derive(Debug)]
struct RfdCompletionLevels([Option<Vec<RegionCombination>>; 8]);

impl RfdCompletionLevels {
    pub fn new() -> Self {
        let mut result = Self(Default::default());

        for level in result.0.iter_mut() {
            *level = Some(Vec::new());
        }

        result
    }

    fn level_mut(&mut self, level: u8) -> &mut Vec<RegionCombination> {
        assert!(level > 0 && level <= 8);
        self.0[(level - 1) as usize].as_mut().unwrap()
    }
}

fn get_completions() -> RfdCompletionLevels {
    let mut result = RfdCompletionLevels::new();

    for n in 1..128 {
        let combo = RegionCombination::from(n);
        for m in 1..=count_subquests(&combo) {
            result.level_mut(m).push(combo.clone());
        }
    }

    result
}

fn filter_supersets(mut levels: RfdCompletionLevels) -> RfdCompletionLevels {
    let mut results = RfdCompletionLevels::new();

    for (level, result) in levels.0.iter_mut().zip(results.0.iter_mut()) {
        let result = result.as_mut().unwrap();
        for combo in level.take().unwrap().into_iter() {
            // Combinations are created using incrementing binary numbers, meaning possible subsets
            // always come before all of their supersets
            if !result.iter().rev().any(|other| combo.is_superset_of(other)) {
                result.push(combo);
            }
        }
    }

    results
}

fn create_expression(level: Vec<RegionCombination>) -> BoolExpr<Region> {
    let mut builder = BoolExprBuilder::new();

    // TODO try to simplify here
    // NOTE build AST and apply transformation rules then convert AST to RPN

    let mut or_count = 0;
    for combo in level {
        let mut and_count = 0;

        for region in Region::into_enum_iter() {
            if combo[region] {
                builder.var(region);
                and_count += 1;
            }
        }

        for _ in 1..and_count {
            builder.and()
        }

        or_count += 1;
    }
    for _ in 1..or_count {
        builder.or()
    }

    builder.finalize().unwrap()
}

fn create_expressions(mut levels: RfdCompletionLevels) -> Vec<BoolExpr<Region>> {
    levels
        .0
        .iter_mut()
        .map(|l| create_expression(l.take().unwrap()))
        .collect()
}

/// Add expressions for the RDF gloves.
pub fn add_gloves(map: &mut ExprMap) {
    let mut expressions = create_expressions(filter_supersets(get_completions()));

    map.insert("Barrows gloves".to_string(), expressions[7].clone());
    map.insert("Dragon gloves".to_string(), expressions.pop().unwrap());
    map.insert("Rune gloves".to_string(), expressions.pop().unwrap());
    map.insert("Adamant gloves".to_string(), expressions.pop().unwrap());
    map.insert("Mithril gloves".to_string(), expressions.pop().unwrap());
    map.insert("Black gloves".to_string(), expressions.pop().unwrap());
    map.insert("Steel gloves".to_string(), expressions.pop().unwrap());
    map.insert("Iron gloves".to_string(), expressions.pop().unwrap());
    map.insert("Bronze gloves".to_string(), expressions.pop().unwrap());

    let mut builder = BoolExprBuilder::new();
    builder.var(Region::Kandarin);
    builder.var(Region::Morytania);
    builder.or();

    map.insert(
        "Hardleather gloves".to_string(),
        builder.finalize().unwrap(),
    );
}
