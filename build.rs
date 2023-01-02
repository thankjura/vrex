fn main() {
    glib_build_tools::compile_resources(
        "data/resources/",
        "data/resources/vrex.gresource.xml",
        "vrex.gresource",
    );
}
