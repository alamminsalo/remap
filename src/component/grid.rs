use crate::component::Tile;
use crate::model::{Tile as TileModel, Viewport};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Grid {
    zoom: u8,
    vw: Viewport,
}

pub enum Msg {}

#[derive(PartialEq, Clone, Default)]
pub struct Prop {
    pub vw: Viewport,
    pub zoom: u8,
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Prop;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Grid {
            vw: prop.vw,
            zoom: prop.zoom,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        let changed = self.vw != prop.vw || self.zoom != prop.zoom;
        self.vw = prop.vw;
        self.zoom = prop.zoom;
        changed
    }
}

impl Renderable<Grid> for Grid {
    fn view(&self) -> Html<Self> {
        let tiles = self.vw.tiles(self.zoom);
        html! {
            <div uk-grid="", class="uk-grid-collapse uk-child-width-auto",>
                { for tiles.iter().map(view_tile) }
            </div>
        }
    }
}

fn view_tile(tile: &TileModel) -> Html<Grid> {
    html! {
        <Tile: tile=tile, />
    }
}
