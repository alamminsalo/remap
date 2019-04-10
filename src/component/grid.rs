use crate::component::Tile;
use crate::model::{Tile as TileModel, Viewport};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Grid {
    vw: Viewport,
}

pub enum Msg {}

#[derive(PartialEq, Clone, Default)]
pub struct Prop {
    pub vw: Viewport,
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Prop;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Grid { vw: prop.vw }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        let changed = self.vw != prop.vw;
        self.vw = prop.vw;
        changed
    }
}

impl Renderable<Grid> for Grid {
    fn view(&self) -> Html<Self> {
        let view_tiles = self
            .vw
            .tiles()
            .map(|t| view_tile(&t, self.vw.pixel_offset(&t)));
        html! {
            <div class="re-viewport",>
            { for view_tiles }
            </div>
        }
    }
}

fn view_tile(tile: &TileModel, offset: (i32, i32)) -> Html<Grid> {
    html! {
        <Tile: tile=tile, offset=offset,/>
    }
}
