use mathrs::polynomials;
use mathrs::mstructs;

fn main() {
    polynomials::closed_forms::deg_two(
        mstructs::Expression {
            vars: vec!["x"],
            formula: "3x^2+4x+1"
        }
    );
}