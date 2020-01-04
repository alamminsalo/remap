use crate::component::Tile;
use crate::model::position::Px;
use crate::model::{Tile as TileModel, TileLayer, Viewport};
use itertools::Itertools;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

/// OSM Raster tile grid
pub struct Grid {
    // inner visible viewport
    vw: Viewport,
    // outer viewport, used to fetch tiles
    vw_outer: Viewport,
    // layers to draw
    layers: Vec<TileLayer>,
}

impl Grid {
    fn tile_rows(vw: &Viewport) -> Vec<Vec<TileModel>> {
        // group by rows
        let mut tile_rows = vec![];
        for (_, group) in &vw.tiles().group_by(|t| t.y) {
            tile_rows.push(group.collect::<Vec<TileModel>>());
        }
        tile_rows
    }

    // returns pixel offset between viewports
    fn tile_offset(vw: &Viewport, vw_outer: &Viewport) -> Px {
        // take nw tile of outer viewport
        let tile = TileModel::from_lonlat(vw_outer.lon_min, vw_outer.lat_max, vw.z);
        // return pixel offset from inner viewport
        vw.pixel_offset(&tile)
    }
}

pub enum Msg {}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct Prop {
    pub vw: Viewport,
    pub vw_outer: Viewport,
    pub layers: Vec<TileLayer>,
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Prop;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Grid {
            vw: prop.vw,
            vw_outer: prop.vw_outer,
            layers: prop.layers,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        let mut changed = false;
        if self.vw != prop.vw {
            self.vw = prop.vw;
            self.vw_outer = prop.vw_outer;
            changed = true;
        }
        if self.layers != prop.layers {
            self.layers = prop.layers;
            changed = true;
        }
        changed
    }

    fn view(&self) -> Html<Self> {
        let tile_rows = Grid::tile_rows(&self.vw_outer);
        let tile_offset = Grid::tile_offset(&self.vw, &self.vw_outer);
        html! {
            <div class="remap-tile-grid remap-noselect", draggable="false",
                style={format!("transform: translate({}px, {}px)", &tile_offset.x, &tile_offset.y)},>
                    { for self.layers.iter().map(|l| tile_layer(l, &tile_rows)) }
            </div>
        }
    }
}

// draws tile layer
fn tile_layer(layer: &TileLayer, tile_rows: &[Vec<TileModel>]) -> Html<Grid> {
    html! {
        <div class="remap-tile-layer",>
            { for tile_rows.into_iter().map(|tr| tile_row(tr, layer)) }
        </div>
    }
}

// draws tile row
fn tile_row(tiles: &[TileModel], layer: &TileLayer) -> Html<Grid> {
    html! {
        <div class="remap-tile-row",>
            { for tiles.iter().map(|t| tile(t.clone(), layer)) }
        </div>
    }
}

// draws tile
fn tile(tile: TileModel, layer: &TileLayer) -> Html<Grid> {
    html! {
        <Tile: tile=tile, layer=layer.clone(),/>
    }
}
