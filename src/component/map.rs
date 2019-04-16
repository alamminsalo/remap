use super::Grid;
use crate::model::position::{LonLat, Px};
use crate::model::Viewport;
use crate::state::{inertia, panning};
use stdweb::unstable::TryInto;
use stdweb::web::event::{ITouchEvent, ResizeEvent, TouchEnd, TouchMove, TouchStart};
use stdweb::web::{
    document, window, Element, EventListenerHandle, HtmlElement, IEventTarget, IHtmlElement,
    INonElementParentNode,
};
use uuid::Uuid;
use yew::events::IMouseEvent;
use yew::services::render::{RenderService, RenderTask};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Map {
    link: ComponentLink<Self>,
    render: RenderService,
    render_task: Option<RenderTask>,

    // inner state variables
    id: String,
    center: LonLat,
    zoom: usize,
    width: i32,  // pixels
    height: i32, // pixels

    // state handlers
    panning: panning::State,
    inertia: Option<inertia::State>,

    // dom callback handles
    resize_handle: Option<EventListenerHandle>,
    touchend_handle: Option<EventListenerHandle>,
    touchstart_handle: Option<EventListenerHandle>,
    touchmove_handle: Option<EventListenerHandle>,
}

impl Map {
    fn finish_panning(&mut self) {
        // end movement
        let offset: Px = self.panning.end().into();
        self.center = self
            .center
            .px(self.zoom)
            .translate(&offset.neg())
            .lonlat(self.zoom);
    }
}

pub enum Msg {
    Init,
    Resize,
    Noop,
    Goto(Px, i8), // centers immediately to point with given zoom
    Pan(f64, f64),
    PanBegin(f64, f64),
    PanRelease,
    MoveEnd,
    Inertia(f64),
    Zoom(i8),
}

impl Component for Map {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_self(Msg::Init);
        Map {
            link: link,
            render: RenderService::new(),
            render_task: None,
            id: Uuid::new_v4().to_simple().to_string(),
            center: LonLat {
                lon: 29.8,
                lat: 62.6,
            },
            height: 256,
            width: 256,
            zoom: 13,
            panning: panning::State::default(),
            inertia: None,
            // handlers empty at first
            resize_handle: None,
            touchend_handle: None,
            touchstart_handle: None,
            touchmove_handle: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Noop => false,
            Msg::Init => {
                // make resize event handler
                let cb = self.link.send_back(|_| Msg::Resize);
                self.resize_handle =
                    Some(window().add_event_listener(move |_: ResizeEvent| cb.emit(())));

                // touch start
                let cb = self.link.send_back(|e: TouchStart| {
                    match e.target_touches().iter().next() {
                        Some(touch) => Msg::PanBegin(touch.screen_x(), touch.screen_y()),
                        _ => Msg::PanRelease, // may end panning if no touches found
                    }
                });
                self.touchstart_handle =
                    Some(window().add_event_listener(move |e: TouchStart| cb.emit(e)));

                // touch end
                let cb = self.link.send_back(|_| Msg::PanRelease);
                self.touchend_handle =
                    Some(window().add_event_listener(move |_: TouchEnd| cb.emit(())));

                // touch move
                let cb = self.link.send_back(|e: TouchMove| {
                    match e.target_touches().iter().next() {
                        Some(touch) => Msg::Pan(touch.screen_x(), touch.screen_y()),
                        _ => Msg::PanRelease, // may end panning if no touches found
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
            Msg::Goto(px, z) => {
                console!(log, &(px.x as i32), &(px.y as i32));
                let vw = Viewport::new(&self.center, (self.width, self.height), self.zoom);
                self.center = vw.pixels().translate(&px).lonlat(self.zoom);
                self.link.send_self(Msg::Zoom(z));
                true
            }
            Msg::Pan(x, y) => {
                if self.panning.status() == panning::Status::Panning {
                    self.panning.set_position((x, y));
                    true
                } else {
                    false
                }
            }
            Msg::PanBegin(x, y) => {
                self.inertia = None;
                if self.panning.status() != panning::Status::Idle {
                    self.finish_panning();
                }
                self.panning.begin((x, y));
                false
            }
            Msg::PanRelease => {
                if self.panning.status() == panning::Status::Panning {
                    self.inertia = Some(inertia::State::begin(self.panning.release()));
                    self.link.send_self(Msg::Inertia(0.0));
                }
                true
            }
            Msg::MoveEnd => {
                self.inertia = None;
                // console!(log, "move end");
                self.finish_panning();
                true
            }
            Msg::Inertia(dt) => {
                if let Some(ref mut inertia) = self.inertia {
                    self.panning.add_relative(inertia.tick(dt));
                    match inertia.status() {
                        inertia::Status::InProgress => {
                            self.render_task = Some(self.render.request_animation_frame(
                                self.link.send_back(|dt| Msg::Inertia(dt / 1e6)),
                            ));
                        }
                        inertia::Status::Ended => {
                            self.render_task = None;
                            self.link.send_self(Msg::MoveEnd);
                        }
                    }
                    true
                } else {
                    false
                }
            }
            Msg::Zoom(z) => {
                //console!(log, "zoom");
                if z >= 1 && z <= 18 {
                    self.zoom = z as usize;
                }
                true
            }
        }
    }
}

impl Renderable<Map> for Map {
    fn view(&self) -> Html<Self> {
        // make viewport
        // apply transform on middle of moving
        let mut c = self.center.px(self.zoom);
        if self.panning.status() != panning::Status::Idle {
            let offset_px: Px = self.panning.offset().into();
            c = c.translate(&offset_px.neg());
        }
        let vw = Viewport::new(&c.lonlat(self.zoom), (self.width, self.height), self.zoom);

        // zoomlevel
        let z = self.zoom as i8;

        html! {
            <div id={&self.id}, class="remap-map",>
                <div class="remap-zoom-controls",>
                    <i class="remap-control remap-control-zoom-in", onclick=|_| Msg::Zoom(z + 1),></i>
                    <i class="remap-control remap-control-zoom-out", onclick=|_| Msg::Zoom(z - 1),></i>
                </div>
                <div class="remap-viewport",
                    onmousedown=|e| Msg::PanBegin(e.screen_x() as f64, e.screen_y() as f64),
                    onmouseup=|_| Msg::PanRelease,
                    onmouseleave=|_| Msg::PanRelease,
                    ondoubleclick=|e| Msg::Goto((e.offset_x(), e.offset_y()).into(), z + 1),
                    onmousemove=|e| Msg::Pan(e.screen_x() as f64, e.screen_y() as f64),
                   //  onmousewheel=|e| {
                   //      if e.delta_y() > 10.0 {
                   //          Msg::Zoom(z + 1)
                   //      } else if e.delta_y() < -10.0 {
                   //          Msg::Zoom(z - 1)
                   //      } else {
                   //          Msg::Noop
                   //      }
                   //  },
                    >
                    // tile grid
                    <Grid: vw=vw, />
                </div>
            </div>
        }
    }
}
