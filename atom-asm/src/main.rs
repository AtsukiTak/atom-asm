mod macho;
mod num;

fn main() {
    let macho = macho::gen_demo();
    let bytes = macho::serialize(&macho);
    // write into file
}
