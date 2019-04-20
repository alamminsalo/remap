use super::{Grid, Input, InputEvent};
use crate::model::position::{LonLat, Px};
use crate::model::{TileLayer, Viewport};
use crate::state::{inertia, layer, panning};
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
    // element width in pixels
    width: i32,
    // element height in pixels
    height: i32,
    // if set to some, indicates a map move in progress
    movement: Option<Px>,

    // state handlers
    panning: panning::State,
    inertia: inertia::State,
    layers: layer::State,

    // dom callback handles
    handles: Vec<EventListenerHandle>,
}

pub enum Msg {
    Init,
    Resize,
    Goto(Px, i8), // centers immediately to point with given zoom
    Zoom(i8),
    Input(Px, InputEvent),
}

impl Map {
    /// Returns translated viewport based on offset
    fn panned_viewport(&self, offset: &Px) -> Viewport {
        // calc new center
        let center = self
            .center
            .px(self.zoom)
            .translate(&offset.neg())
            .lonlat(self.zoom);
        // make new viewport from center
        Viewport::new(&center, (self.width, self.height), self.zoom)
    }
    fn finish_panning(&mut self) {
        // end movement
        if let Some(offset) = self.movement.take() {
            self.center = self
                .center
                .px(self.zoom)
                .translate(&offset.neg())
                .lonlat(self.zoom);
        }
    }
    /// Calculates map grid viewports
    fn calc_viewports(&self) -> (Viewport, Viewport) {
        // TODO: investigate if this impacts performance to do so many calculations on the view
        // function
        if let Some(ref offset) = self.movement {
            // make new viewport from center
            let vw = self.panned_viewport(offset);
            // resize outer viewport accordingly
            let mut adjust_amt = offset.neg().normalize(512);
            // multiply if panning fast, on steps of 35 vel
            let (vx, vy) = self.panning.velocity;
            adjust_amt.x *= (1.0 + vx.abs() / 35.0) as i64;
            adjust_amt.y *= (1.0 + vy.abs() / 35.0) as i64;
            (vw, vw.resize_keep_min_bounds(adjust_amt))
        } else {
            // make viewports
            let vw = Viewport::new(&self.center, (self.width, self.height), self.zoom);
            (vw, vw.clone())
        }
    }
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
            movement: None,
            zoom: 4,
            panning: Default::default(),
            inertia: Default::default(),
            // add single raster layer as default
            // TODO: parametrize
            layers: layer::State::new(vec![TileLayer::new(
                "https://tile.thunderforest.com/neighbourhood",
                ".png?apikey=9d61ff3f272b4bbaa7d9c0f63ad34177",
            )]),
            handles: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Init => {
                // make resize event handler
                let cb = self.link.send_back(|_| Msg::Resize);
                self.handles
                    .push(window().add_event_listener(move |_: ResizeEvent| cb.emit(())));
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
                // console!(log, &(px.x as i32), &(px.y as i32));
                let vw = Viewport::new(&self.center, (self.width, self.height), self.zoom);
                self.center = vw.pixels().translate(&px).lonlat(self.zoom);
                self.link.send_self(Msg::Zoom(z));
                true
            }
            Msg::Input(pos, e) => {
                match e {
                    InputEvent::Click => {
                        // TODO
                    }
                    InputEvent::DoubleClick => {
                        self.link.send_self(Msg::Goto(pos, self.zoom as i8 + 1));
                    }
                    InputEvent::MoveBegin => {
                        self.finish_panning();
                    }
                    InputEvent::Move => {
                        self.movement = Some(pos);
                    }
                    InputEvent::MoveEnd => {
                        self.finish_panning();
                    }
                }
                true
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
        // calc viewports
        let (vw, vw_outer) = self.calc_viewports();
        // zoomlevel
        let z = self.zoom as i8;
        // visible layers
        let visible_layers = self.layers.layers_by_visibility(true);

        html! {
            <div id={&self.id}, class="remap-map",>
                // TODO: abstract
                <div class="remap-zoom-controls",>
                    <i class="remap-control remap-control-zoom-in", onclick=|_| Msg::Zoom(z + 1),></i>
                    <i class="remap-control remap-control-zoom-out", onclick=|_| Msg::Zoom(z - 1),></i>
                </div>
                <div class="remap-viewport",>
                    // tile grid
                    <Grid: vw=vw, vw_outer=vw_outer, layers=visible_layers, />
                </div>
                // input handling component
                <Input: oninput=|(px,e)| Msg::Input(px,e), />
            </div>
        }
    }
}
