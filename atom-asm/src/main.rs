mod macho;

fn main() {
    let macho = macho::gen_demo();
    let bytes = macho::serialize(&macho);
    // write into file
}
