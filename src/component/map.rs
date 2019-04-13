use super::Grid;
use crate::model::Viewport;
use crate::state::movement;
use geo::Coordinate;
use stdweb::unstable::TryInto;
use uuid::Uuid;
use yew::events::IMouseEvent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Map {
    id: String,
    center: Coordinate<f64>,
    zoom: u8,
    link: ComponentLink<Self>,
    // pixel width, height
    width: i32,
    height: i32,
    move_state: movement::State,
}

pub enum Msg {
    Init,
    Refresh,
    Move(i32, i32),
    MoveBegin,
    MoveEnd,
    Zoom(i8),
}

impl Component for Map {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_self(Msg::Init);
        Map {
            id: Uuid::new_v4().to_simple().to_string(),
            center: Coordinate { x: 29.8, y: 62.6 },
            height: 256,
            width: 256,
            zoom: 13,
            move_state: movement::State::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Init => {
                let cb = self.link.send_back(|_| Msg::Refresh);
                let closure = move || cb.emit(());
                js! {
                    var _cb = @{closure};
                    var callback = function(){
                        _cb();
                    };
                    window.addEventListener("resize", callback);
                    var el = document.getElementById(@{self.id.clone()});
                    el.rs_closures = [_cb];
                };
                self.link.send_self(Msg::Refresh);
                false
            }
            Msg::Refresh => {
                // Get size of el
                let size: Vec<i32> = js! {
                    var self = document.getElementById(@{self.id.clone()});
                    return [ self.clientWidth, self.clientHeight ];
                }
                .try_into()
                .unwrap_or(vec![0, 0]);

                // set width, height
                self.width = size[0];
                self.height = size[1];

                true
            }
            Msg::Move(x, y) => {
                if self.move_state.is_moving() {
                    self.move_state.add_movement((x, y));
                    let pos = self.move_state.position;
                    console!(log, "move", &pos.0, &pos.1);
                }
                true
            }
            Msg::MoveBegin => {
                console!(log, "move begin");
                self.move_state.begin();
                false
            }
            Msg::MoveEnd => {
                if self.move_state.is_moving() {
                    console!(log, "move end");
                    let offset = self.move_state.end();
                    let vw = Viewport::new(&self.center, (self.width, self.height), self.zoom);
                    self.center = vw.translate(offset).center();
                    true
                } else {
                    false
                }
            }
            Msg::Zoom(z) => {
                console!(log, "zoom");
                if z >= 1 && z <= 18 {
                    self.zoom = z as u8;
                }
                true
            }
        }
    }
}

impl Renderable<Map> for Map {
    fn view(&self) -> Html<Self> {
        let mut vw = Viewport::new(&self.center, (self.width, self.height), self.zoom);

        if self.move_state.is_moving() {
            // apply transform
            vw = vw.translate(self.move_state.position);
        }

        // zoomlevel
        let z = self.zoom as i8;

        html! {
            <div id={&self.id}, class="remap-map",>
                <div class="remap-zoom-controls",>
                    <div>{&format!("zoom: {}", &z)}</div>
                    <div><button onclick=|_| Msg::Zoom(z + 1),>{"+"}</button></div>
                    <div><button onclick=|_| Msg::Zoom(z - 1),>{"-"}</button></div>
                </div>
                <div class="remap-viewport",
                    onpointerdown=|_| Msg::MoveBegin,
                    onpointerup=|_| Msg::MoveEnd,
                    onpointerleave=|_| Msg::MoveEnd,
                    onpointermove=|e| Msg::Move(e.movement_x(), e.movement_y()),>
                    // tile grid
                    <Grid: vw=vw, />
                </div>
            </div>
        }
    }
}
