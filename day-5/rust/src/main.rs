mod letters;
pub mod types;

fn main() {
    let glyphs = letters::define_glyphs();
    println!("{:?}", glyphs);
}