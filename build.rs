use libadwaita::gio::compile_resources;

fn main() {
    compile_resources(
        "content",
        "content/app.gresource.xml",
        "gtk-rss-reader.gresource",
    );
}
