use super::Grid;
use crate::model::Viewport;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

const STORE_KEY: &'static str = "state.v1";

pub struct Root {
    vw: Viewport,
}

pub enum Msg {}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Root {
            // 37,4423231496
            vw: Viewport {
                lon_min: 29.71,
                lon_max: 29.87539,
                lat_min: 62.557,
                lat_max: 62.631,
                z: 12,
            },
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
            <Grid: vw=self.vw.clone(), />
        }
    }
}
