use super::Grid;
use crate::model::Viewport;
use crate::state::movement;
use geo::Coordinate;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ITouchEvent, ResizeEvent, TouchEnd, TouchMove, TouchStart};
use stdweb::web::{
    document, window, Element, EventListenerHandle, HtmlElement, IEventTarget, IHtmlElement,
    INonElementParentNode,
};
use uuid::Uuid;
use yew::events::IMouseEvent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Map {
    link: ComponentLink<Self>,

    // inner state variables
    id: String,
    center: Coordinate<f64>,
    zoom: u8,
    width: i32,  // pixels
    height: i32, // pixels

    // state handlers
    move_state: movement::State,

    // dom callback handles
    resize_handle: Option<EventListenerHandle>,
    touchend_handle: Option<EventListenerHandle>,
    touchstart_handle: Option<EventListenerHandle>,
    touchmove_handle: Option<EventListenerHandle>,
}

pub enum Msg {
    Init,
    Resize,
    Move(i32, i32),
    MoveBegin(i32, i32),
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
            link: link,
            // handlers empty at first
            resize_handle: None,
            touchend_handle: None,
            touchstart_handle: None,
            touchmove_handle: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Init => {
                // make resize event handler
                let cb = self.link.send_back(|_| Msg::Resize);
                self.resize_handle =
                    Some(window().add_event_listener(move |_: ResizeEvent| cb.emit(())));

                // touch start
                let cb = self.link.send_back(|e: TouchStart| {
                    match e.target_touches().iter().next() {
                        Some(touch) => {
                            Msg::MoveBegin(touch.screen_x() as i32, touch.screen_y() as i32)
                        }
                        _ => Msg::MoveEnd, // may end movement if no touches found
                    }
                });
                self.touchstart_handle =
                    Some(window().add_event_listener(move |e: TouchStart| cb.emit(e)));

                // touch end
                let cb = self.link.send_back(|_| Msg::MoveEnd);
                self.touchend_handle =
                    Some(window().add_event_listener(move |_: TouchEnd| cb.emit(())));

                // touch move
                let cb = self.link.send_back(|e: TouchMove| {
                    match e.target_touches().iter().next() {
                        Some(touch) => Msg::Move(touch.screen_x() as i32, touch.screen_y() as i32),
                        _ => Msg::MoveEnd, // may end movement if no touches found
                    }
                });
                self.touchmove_handle =
                    Some(window().add_event_listener(move |e: TouchMove| cb.emit(e)));

                // send initial resize event
                self.link.send_self(Msg::Resize);
                // no need for immediate redraw
                false
            }
            Msg::Resize => {
                // get element
                document()
                    .get_element_by_id(&self.id)
                    .map(|el: Element| {
                        // try into html element, which has the rect methods
                        el.try_into()
                            .map(|html_el: HtmlElement| {
                                // set width, height from rect object
                                let r = html_el.get_bounding_client_rect();
                                self.width = r.get_width() as i32;
                                self.height = r.get_height() as i32;
                            })
                            .ok()
                    })
                    .is_some()
            }
            Msg::Move(x, y) => {
                if self.move_state.is_moving() {
                    self.move_state.set_position((x, y));
                    //let pos = self.move_state.offset();
                    //console!(log, "move", &pos.0, &pos.1);
                }
                true
            }
            Msg::MoveBegin(x, y) => {
                //console!(log, "move begin");
                self.move_state.begin((x, y));
                false
            }
            Msg::MoveEnd => {
                if self.move_state.is_moving() {
                    //console!(log, "move end");
                    let offset = self.move_state.end();
                    let vw = Viewport::new(&self.center, (self.width, self.height), self.zoom);
                    self.center = vw.translate(offset).center();
                    true
                } else {
                    false
                }
            }
            Msg::Zoom(z) => {
                //console!(log, "zoom");
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
            vw = vw.translate(self.move_state.offset());
        }

        // zoomlevel
        let z = self.zoom as i8;

        html! {
            <div id={&self.id}, class="remap-map",>
                <div class="remap-zoom-controls",>
                    <div class="remap-control",><button onclick=|_| Msg::Zoom(z + 1),>{"+"}</button></div>
                    <div class="remap-control",><button onclick=|_| Msg::Zoom(z - 1),>{"-"}</button></div>
                </div>
                <div class="remap-viewport",
                    onmousedown=|e| Msg::MoveBegin(e.screen_x(), e.screen_y()),
                    onmouseup=|_| Msg::MoveEnd,
                    onmouseleave=|_| Msg::MoveEnd,
                    onmousemove=|e| Msg::Move(e.screen_x(), e.screen_y()),>
                    // tile grid
                    <Grid: vw=vw, />
                </div>
            </div>
        }
    }
}
