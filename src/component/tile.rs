use crate::model::{Tile as TileModel, TileLayer};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Tile {
    tile: TileModel,
    layer: TileLayer,
}

pub enum Msg {}

#[derive(Default, PartialEq, Clone)]
pub struct Prop {
    pub tile: TileModel,
    pub layer: TileLayer,
}

impl Component for Tile {
    type Message = Msg;
    type Properties = Prop;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Tile {
            tile: prop.tile,
            layer: prop.layer,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let c = self.tile != props.tile;
        self.tile = props.tile;
        c
    }
}

impl Renderable<Tile> for Tile {
    fn view(&self) -> Html<Self> {
        html! {
            <span class="remap-tile", style={&format!("background-image: url({})",&self.layer.tile_url(&self.tile))}, />
        }
    }
}
