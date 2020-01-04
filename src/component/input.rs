use crate::model::Px;
use crate::state::{inertia, panning};
use stdweb::js;
use stdweb::unstable::TryInto;
use stdweb::web::event::{ITouchEvent, TouchEnd, TouchMove, TouchStart};
use stdweb::web::{document, EventListenerHandle, IEventTarget, INonElementParentNode};
use yew::events::IMouseEvent;
use yew::services::render::{RenderService, RenderTask};
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

pub enum InputEvent {
    Click,
    DoubleClick,
    MoveBegin,
    Move,
    MoveEnd,
}

pub struct Input {
    id: String,
    link: ComponentLink<Self>,
    // event handlers, using relative pixels to parent object
    oninput: Option<Callback<(Px, InputEvent)>>,
    // state handlers
    inertia: inertia::State,
    panning: panning::State,
    // render service vars
    render: RenderService,
    render_task: Option<RenderTask>,
    // event listener handles
    handles: Vec<EventListenerHandle>,
}

pub enum Msg {
    Init,
    Click(f64, f64),
    DoubleClick(f64, f64),
    Move(f64, f64),
    MoveBegin(f64, f64),
    MoveRelease,
    Decelerate(f64, f64),
    Stop,
}

#[derive(Properties, Default, PartialEq, Clone)]
pub struct Prop {
    pub oninput: Option<Callback<(Px, InputEvent)>>,
}

impl Input {
    // function to send back events
    fn notify(&self, pos: Px, ev: InputEvent) {
        if let Some(ref cb) = self.oninput {
            cb.emit((pos, ev));
        }
    }
}

impl Component for Input {
    type Message = Msg;
    type Properties = Prop;

    fn create(prop: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_self(Msg::Init);
        Self {
            id: String::from("remap_input"),
            link: link,
            oninput: prop.oninput,
            panning: Default::default(),
            inertia: Default::default(),
            render: RenderService::new(),
            render_task: None,
            handles: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Init => {
                if let Some(el) = document().get_element_by_id(&self.id) {
                    // touch start
                    let cb = self.link.send_back(|e: TouchStart| {
                        match e.target_touches().iter().next() {
                            Some(touch) => Msg::MoveBegin(touch.screen_x(), touch.screen_y()),
                            _ => Msg::MoveRelease, // may end panning if no touches found
                        }
                    });
                    self.handles
                        .push(el.add_event_listener(move |e: TouchStart| cb.emit(e)));

                    // touch end
                    let cb = self.link.send_back(|_| Msg::MoveRelease);
                    self.handles
                        .push(el.add_event_listener(move |_: TouchEnd| cb.emit(())));

                    // touch move
                    let cb = self.link.send_back(|e: TouchMove| {
                        match e.target_touches().iter().next() {
                            Some(touch) => Msg::Move(touch.screen_x(), touch.screen_y()),
                            _ => Msg::MoveRelease, // may end panning if no touches found
                        }
                    });
                    self.handles
                        .push(el.add_event_listener(move |e: TouchMove| cb.emit(e)));
                }
            }
            Msg::Click(x, y) => {
                self.notify((x, y).into(), InputEvent::Click);
            }
            Msg::DoubleClick(x, y) => {
                self.notify((x, y).into(), InputEvent::DoubleClick);
            }
            Msg::Move(x, y) => {
                if self.panning.status() == panning::Status::Panning {
                    self.panning.set_position((x, y));
                    self.notify(self.panning.offset().into(), InputEvent::Move);
                }
            }
            Msg::MoveBegin(x, y) => {
                if self.panning.status() != panning::Status::Idle {
                    self.notify((0, 0).into(), InputEvent::MoveEnd);
                }
                self.panning.begin((x, y));
                self.notify((0, 0).into(), InputEvent::MoveBegin);
            }
            Msg::MoveRelease => {
                if self.panning.status() == panning::Status::Panning {
                    self.inertia = inertia::State::begin(self.panning.release());
                    let perfnow: f64 = js! { return performance.now(); }.try_into().unwrap_or(0.0);
                    self.link.send_self(Msg::Decelerate(perfnow, perfnow));
                }
            }
            Msg::Decelerate(t1, t0) => {
                if self.panning.status() == panning::Status::Free {
                    let dt = t1 - t0;
                    // console!(log, "decelerate", &dt);
                    self.panning.add_relative(self.inertia.tick(dt / 1e6));
                    self.notify(self.panning.offset().into(), InputEvent::Move);
                    match self.inertia.status() {
                        inertia::Status::InProgress => {
                            self.render_task = Some(self.render.request_animation_frame(
                                self.link.send_back(move |t2| Msg::Decelerate(t2, t1)),
                            ));
                        }
                        inertia::Status::Ended => {
                            self.render_task = None;
                            self.link.send_self(Msg::Stop);
                        }
                    }
                }
            }
            Msg::Stop => {
                self.panning.end();
                self.notify((0, 0).into(), InputEvent::MoveEnd);
            }
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.oninput = props.oninput;
        false
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div id={&self.id}, class="remap-input",
                onclick=|e| Msg::Click(e.offset_x(), e.offset_y()),
                ondoubleclick=|e| Msg::DoubleClick(e.offset_x(), e.offset_y()),
                onmousedown=|e| Msg::MoveBegin(e.screen_x() as f64, e.screen_y() as f64),
                onmouseup=|_| Msg::MoveRelease,
                onmouseleave=|_| Msg::MoveRelease,
                onmousemove=|e| Msg::Move(e.screen_x() as f64, e.screen_y() as f64),>
            </div>
        }
    }
}
