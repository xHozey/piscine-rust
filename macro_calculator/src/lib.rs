use json::*;

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut calcs = 0.;
    let mut carbs = 0.;
    let mut proteins = 0.;
    let mut fats = 0.;
    for food in foods {
        calcs += food.calories.1[0..food.calories.1.len()-4].parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions
    }
    let mut res = JsonValue::new_object();
    res["calcs"] = format!("{:.2}", calcs).into();
    res["carbs"] = format!("{:.2}", carbs).into();
    res["proteins"] = format!("{:.2}",proteins).into();
    res["fats"] = format!("{:.2}",fats).into();
    res
}