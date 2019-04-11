use super::Grid;
use crate::model::Viewport;
use geo::Coordinate;
use stdweb::unstable::TryInto;
use stdweb::Value;
use uuid::Uuid;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

const STORE_KEY: &'static str = "state.v1";

pub struct Map {
    id: String,
    center: Coordinate<f64>,
    zoom: u8,
    link: ComponentLink<Self>,
    // pixel width, height
    width: i32,
    height: i32,
}

pub enum Msg {
    Init,
    Refresh,
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
        }
    }
}

impl Renderable<Map> for Map {
    fn view(&self) -> Html<Self> {
        let vw = Viewport::new(&self.center, (self.width, self.height), self.zoom);
        html! {
            <div id={&self.id}, class="remap-map",>
                <Grid: vw=vw, />
            </div>
        }
    }
}
