mod rpn;
fn main() {
    let mut calc = rpn::RpnCalc::new();
    let res = calc.calc("test");
    println!("Result was: \"{res}\".")
}
