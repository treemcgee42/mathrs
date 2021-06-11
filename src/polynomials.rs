use crate::mstructs;
use std::collections::HashMap;

pub mod closed_forms {
    use super::*;

    // Convert the string into something usable by equations.
    // For now, we only support polynomial coefficients that 
    // support parsing into floats, but in the future may add
    // support for constant variables.
    fn mathify(s: &str) -> f64 {
        s.parse::<f64>().unwrap()
    }

    // convert something like ax^2 to a
    fn remove_var_and_deg(s: &str) -> f64 {
        let lm2 = s.len()-3;
        let result: f64;

        if lm2==0 {
            result = mathify("1");
        } else {
            result = mathify(&s[0..lm2]);
        }
        
        result
    }

    // convert something like bx to b
    fn remove_var(s: &str) -> f64 {
        let lm1 = s.len()-1;
        let result: f64;

        if lm1==0 {
            result = mathify("1");
        } else {
            result = mathify(&s[0..lm1]);
        }

        result
    }

    // returns a coefficient map representing the equation. For instance,
    // ax^2+bx+c would have a map {0:c, 1:b, 2:a}
    fn parse_polynomial(expr: mstructs::Expression) -> HashMap<u32,f64> {
        let mut coefficient_map = HashMap::new();

        let var_of = expr.vars[0].chars().last();

        for s in expr.formula.split('+') {
            if s.len()>1 {
                if s.chars().nth(s.len()-2)==Some('^') {
                    let n = s.chars().last().unwrap().to_digit(10).unwrap();
                    coefficient_map.insert(n,remove_var_and_deg(s));
                } else if s.chars().last()==var_of {
                    coefficient_map.insert(1_u32,remove_var(s));
                } else {
                    coefficient_map.insert(0_u32,mathify(s));
                }
            } else {
                coefficient_map.insert(0_u32,mathify(s));
            }
        }

        coefficient_map
    }

    // takes Expression of form
    // ax^2 + bx^2 + c
    pub fn deg_two(expr: mstructs::Expression) -> (f64,f64) {
        let cm = parse_polynomial(expr);

        let mut coeffs: [f64; 3] = [0.0,0.0,0.0];

        for n in 0..3 {
            match cm.get(&n) {
                Some(a) => coeffs[n as usize]=*a,
                None => (),
            }
        }

        let r1 = (-1.0 * coeffs[1] + (coeffs[1].powi(2)-4.0*coeffs[0]*coeffs[2]).sqrt())
            / (2.0*coeffs[2]);
        let r2 = (-1.0 * coeffs[1] - (coeffs[1].powi(2)-4.0*coeffs[0]*coeffs[2]).sqrt())
        / (2.0*coeffs[2]);

        (r1,r2)
    }

    mod tests {
        use super::*;

        #[test]
        fn parse_polynomial_test() {
            // check simplest case, all addition, explicit coefficients
            let expr1 = mstructs::Expression {
                vars: vec!["x"],
                formula: "3x^2+4x+1",
            };
            let coefficient_map1 = closed_forms::parse_polynomial(expr1);

            assert_eq!(coefficient_map1.get(&0), Some(&1.0_f64));
            assert_eq!(coefficient_map1.get(&1), Some(&4.0_f64));
            assert_eq!(coefficient_map1.get(&2), Some(&3.0_f64));

            // no degree 1 term, no coefficient for degree 2 term
            let expr2 = mstructs::Expression {
                vars: vec!["x"],
                formula: "x^2+4",
            };

            let coefficient_map2 = closed_forms::parse_polynomial(expr2);

            assert_eq!(coefficient_map2.get(&0), Some(&4.0_f64));
            assert_eq!(coefficient_map2.get(&2), Some(&1.0_f64));

            // check explicit negative coefficients
            let expr3 = mstructs::Expression {
                vars: vec!["x"],
                formula: "-3x^2+-4x+-1",
            };
            let coefficient_map3 = closed_forms::parse_polynomial(expr3);

            assert_eq!(coefficient_map3.get(&0), Some(&-1.0_f64));
            assert_eq!(coefficient_map3.get(&1), Some(&-4.0_f64));
            assert_eq!(coefficient_map3.get(&2), Some(&-3.0_f64));
        }

        #[test]
        fn two_deg_test() {
            let expr_2_m2 = mstructs::Expression {
                vars: vec!["x"],
                formula: "x^2+-4",
            };

            let sols_2_m2 = closed_forms::deg_two(expr_2_m2);

            assert_eq!(sols_2_m2.0, 2.0_f64);
            assert_eq!(sols_2_m2.1, -2.0_f64);
        }
    }
}