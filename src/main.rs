use remap::component::Map;
use stdweb::web::{document, INonElementParentNode};
use yew::App;

fn main() {
    yew::initialize();
    let app = App::<Map>::new();
    if let Some(el) = document().get_element_by_id("map-root") {
        app.mount(el);
    }
    yew::run_loop();
}
