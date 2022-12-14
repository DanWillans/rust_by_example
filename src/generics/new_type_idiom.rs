struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

pub fn new_type_idiom() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    // println!("Old enough {}", old_enough(5));
    // ^ Won't compile because incorrect type. Implicit construction won't work
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));

    let years = Years(42);
    let _years_as_primitive_1: i64 = years.0;
    let Years(_years_as_primitive_2) = years;
}
