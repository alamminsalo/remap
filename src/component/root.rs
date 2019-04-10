use super::Grid;
use crate::model::BoundingBox;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

const STORE_KEY: &'static str = "state.v1";

pub struct Root {
    vw: BoundingBox,
    zoom: u8,
}

pub enum Msg {}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Root {
            vw: BoundingBox {
                lon_min: 29.59,
                lon_max: 29.98,
                lat_min: 62.56,
                lat_max: 62.65,
            },
            zoom: 7,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true,
        }
    }
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <Grid: vw=self.vw.clone(), zoom=self.zoom, />
            </div>
        }
    }
}
