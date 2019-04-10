use mappr::component::Root;
use yew::App;

fn main() {
    yew::initialize();
    App::<Root>::new().mount_to_body();
    yew::run_loop();
}
