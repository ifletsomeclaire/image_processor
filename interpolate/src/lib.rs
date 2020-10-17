pub mod dainargs;

pub fn run_dain(args: &dainargs::DainArgs) {
    std::process::Command::new("E:/DAIN Interpolation/DAIN_APP Alpha/DAINAPP")
        .args(args.to_arguments())
        .spawn()
        .expect("Error spawning DAINAPP");
}
pub fn run_dain_and_wait(args: &dainargs::DainArgs) {
    std::process::Command::new("E:/DAIN Interpolation/DAIN_APP Alpha/DAINAPP")
        .args(args.to_arguments())
        .output()
        .expect("Error outputting DAINAPP");
}
