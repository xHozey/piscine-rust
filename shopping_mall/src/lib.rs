mod mall;
use std::collections::HashMap;
use mall::*;

pub fn biggest_store(mall: &Mall) ->  (&String, &Store) {
    let mut res: Option<(&String, &Store)> = None;
    let mut area = 0;
    for (_, floor) in &mall.floors {
        for (name, store) in &floor.stores {
               if store.square_meters > area {
                res = Some((name, store));
                area = store.square_meters;
               }
        }
    }
    res.unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String,&Employee)> {
    let mut res = Vec::new();
    let mut highest_salary = 0.;
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (_, em) in &store.employees {
                if em.salary > highest_salary {
                    highest_salary = em.salary
                }
            }
        }
    }
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (name, em) in &store.employees {
                if em.salary == highest_salary {
                    res.push((name, em))
                }
            }
        }
    }
    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut res = mall.guards.len();
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            res += store.employees.len()
        }
    }
    res
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total_size = 0;
    for (_, floor) in &mall.floors {
        total_size += floor.size_limit
    }
    let needed_guards = (total_size as f64 / 200.).ceil();
    if mall.guards.len() < needed_guards as usize {
       for (name, guard) in guards {
        mall.hire_guard(name, guard);
        if mall.guards.len() == needed_guards as usize {
            break
        }        
       } 
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in &mut mall.floors {
        for (_, store) in &mut floor.stores {
            for (_, em) in &mut store.employees {
                if (em.working_hours.1 - em.working_hours.0) >= 10 {
                    em.raise(10.*em.salary/100.);
                } else {
                    em.cut(10.*em.salary/100.);
                }
            }
        }
    }
}