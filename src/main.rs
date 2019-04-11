use remap::component::Map;
use yew::App;

fn main() {
    yew::initialize();
    App::<Map>::new().mount_to_body();
    yew::run_loop();
}
