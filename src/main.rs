use jp_deinflector::deinflect;

fn main() {
    let deinflections = deinflect("待った");
    for deinflection in deinflections {
        println!("{}", deinflection);
    }
}