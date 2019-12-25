use z3::ast::Ast;
use z3::*;

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let (solver, digits) = part_1(264793, 803935, &ctx);
    println!(
        "Part 1 has {} possible combinations",
        find_all_solns(&solver, &digits, &ctx).len()
    );
    let (solver, digits) = part_2(264793, 803935, &ctx);
    println!(
        "Part 2 has {} possible combinations",
        find_all_solns(&solver, &digits, &ctx).len()
    );
}

fn part_1(low_range: i64, high_range: i64, ctx: &Context) -> (Solver, Vec<ast::Int>) {
    const NUM_DIGITS: usize = 6;
    let mut digits = Vec::new();

    let solver = Solver::new(&ctx);
    //Each digit is 0 < d < 9
    for x in 0..NUM_DIGITS {
        let digit = ast::Int::new_const(&ctx, format!("digit_{}", x));
        solver.assert(&digit.ge(&ast::Int::from_i64(&ctx, 0)));
        solver.assert(&digit.le(&ast::Int::from_i64(&ctx, 9)));
        digits.push(digit);
    }

    let mut eq_next_assertions = Vec::new();
    // Each digit is less than or equal to the next one
    for x in 0..NUM_DIGITS - 1 {
        solver.assert(&digits[x].le(&digits[x + 1]));
        eq_next_assertions.push(digits[x]._eq(&digits[x + 1]));
    }

    // There is at least one repeating digit (ie: 0*11*345)
    let eq_refs: Vec<_> = eq_next_assertions.iter().collect();
    let at_least_one_double = (&eq_next_assertions[0]).or(&eq_refs[1..]);
    solver.assert(&at_least_one_double);

    // Apply range
    // Get total of digits
    let total = digits
        .iter()
        .fold(ast::Int::from_i64(&ctx, 0), |ast, digit| {
            ast.mul(&[&ast::Int::from_i64(&ctx, 10)]).add(&[&digit])
        });
    solver.assert(&total.ge(&ast::Int::from_i64(&ctx, low_range)));
    solver.assert(&total.le(&ast::Int::from_i64(&ctx, high_range)));
    (solver, digits)
}

fn part_2(low_range: i64, high_range: i64, ctx: &Context) -> (Solver, Vec<ast::Int>) {
    const NUM_DIGITS: usize = 6;
    let mut digits = Vec::new();

    let solver = Solver::new(&ctx);
    //Each digit is 0 < d < 9
    for x in 0..NUM_DIGITS {
        let digit = ast::Int::new_const(&ctx, format!("digit_{}", x));
        solver.assert(&digit.ge(&ast::Int::from_i64(&ctx, 0)));
        solver.assert(&digit.le(&ast::Int::from_i64(&ctx, 9)));
        digits.push(digit);
    }

    let mut eq_next_assertions = Vec::new();

    for x in 0..NUM_DIGITS - 1 {
        // Each digit is less than or equal to the next one
        solver.assert(&digits[x].le(&digits[x + 1]));
        // Next digit is the same
        // The digit two after is not the same
        // Used later to assert max of 2 in a row
        let mut assertion = digits[x]._eq(&digits[x + 1]);
        if let Some(next_next_digit) = digits.get(x + 2) {
            assertion = assertion.and(&[&digits[x]._eq(&next_next_digit).not()]);
        }
        // Also ensure the one before it (if it's not the first digit) is not the same
        if x > 0 {
            assertion = assertion.and(&[&digits[x]._eq(&digits[x - 1]).not()]);
        }
        eq_next_assertions.push(assertion);
    }

    // There is at least one repeating digit (ie: 0*11*345)
    let eq_refs: Vec<_> = eq_next_assertions.iter().collect();
    let at_least_one_double = (&eq_next_assertions[0]).or(&eq_refs[1..]);
    solver.assert(&at_least_one_double);

    // Apply range
    // Get total of digits
    let total = digits
        .iter()
        .fold(ast::Int::from_i64(&ctx, 0), |ast, digit| {
            ast.mul(&[&ast::Int::from_i64(&ctx, 10)]).add(&[&digit])
        });
    solver.assert(&total.ge(&ast::Int::from_i64(&ctx, low_range)));
    solver.assert(&total.le(&ast::Int::from_i64(&ctx, high_range)));
    (solver, digits)
}

fn find_all_solns(solver: &Solver, digits: &Vec<ast::Int>, ctx: &Context) -> Vec<i64> {
    let solver = solver.clone();
    let mut results = Vec::new();
    let mut next_result = solver.check();
    while next_result == SatResult::Sat {
        let model = solver.get_model();
        let mut digit_assertions = Vec::new();
        // Save result for printing
        let mut result_as_num = 0;
        for digit in digits.iter() {
            let digit_result = model.eval(digit).unwrap().as_i64().unwrap();
            result_as_num *= 10;
            result_as_num += digit_result;
            digit_assertions.push(digit._eq(&ast::Int::from_i64(ctx, digit_result)));
        }
        results.push(result_as_num);

        // Assert so we don't get a repeat solution
        let refs: Vec<_> = digit_assertions.iter().collect();
        solver.assert(&digit_assertions[0].and(&refs[1..]).not());
        next_result = solver.check();
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn z3_test() {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let x = ast::Int::new_const(&ctx, "x");
        let y = ast::Int::new_const(&ctx, "y");

        let solver = Solver::new(&ctx);
        solver.assert(&x.gt(&y));
        solver.check();
        let mdl = solver.get_model();
        println!("Model: {}", mdl);
        assert_eq!(solver.check(), SatResult::Sat);
    }
}
