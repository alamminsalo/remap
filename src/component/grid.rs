use crate::component::Tile;
use crate::model::position::Px;
use crate::model::{Tile as TileModel, TileLayer, Viewport};
use itertools::Itertools;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

/// OSM Raster tile grid
pub struct Grid {
    // viewport
    vw: Viewport,
    center: Px,
    height: i32,
    width: i32,
    // rows of tiles
    tile_rows: Vec<Vec<TileModel>>,
    // dom offset from parent
    tile_offset: Px,
    // layers to draw
    layers: Vec<TileLayer>,
}

impl Grid {
    /// recalculates rows and offset for self
    fn recalculate(&mut self, vw: &Viewport) {
        self.vw = vw.clone();
        self.tile_rows = Self::tile_rows(&vw);
        self.tile_offset = Self::tile_offset(&vw, &self.center, self.height, self.width);
    }

    fn tile_rows(vw: &Viewport) -> Vec<Vec<TileModel>> {
        // group by rows
        let mut tile_rows = vec![];
        for (_, group) in &vw.tiles().group_by(|t| t.y) {
            tile_rows.push(group.collect::<Vec<TileModel>>());
        }
        tile_rows
    }

    // returns pixel offset from viewport
    fn tile_offset(vw: &Viewport, center: &Px, width: i32, height: i32) -> Px {
        // take nw tile of viewport
        let tile_px = TileModel::from_lonlat(vw.lon_min, vw.lat_max, vw.z).pixels();
        // take distance to actual visible viewport
        tile_px.distance(&center.translate(&(-width / 2, -height / 2).into()))
    }
}

pub enum Msg {}

#[derive(PartialEq, Clone, Default)]
pub struct Prop {
    pub vw: Viewport,
    pub layers: Vec<TileLayer>,
    pub center: Px,
    pub height: i32,
    pub width: i32,
}

impl Component for Grid {
    type Message = Msg;
    type Properties = Prop;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Grid {
            tile_rows: Grid::tile_rows(&prop.vw),
            tile_offset: Grid::tile_offset(&prop.vw, &prop.center, prop.height, prop.width),
            vw: prop.vw,
            center: prop.center,
            height: prop.height,
            width: prop.width,
            layers: prop.layers,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
        let mut changed = false;
        if self.vw != prop.vw {
            self.center = prop.center;
            self.width = prop.width;
            self.height = prop.height;
            self.recalculate(&prop.vw);
            changed = true;
        }
        if self.layers != prop.layers {
            self.layers = prop.layers;
            changed = true;
        }
        changed
    }
}

impl Renderable<Grid> for Grid {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="remap-tile-grid remap-noselect", draggable="false",
                style={format!("transform: translate({}px, {}px)", &self.tile_offset.x, &self.tile_offset.y)},>
                    { for self.layers.iter().map(|l| tile_layer(l, &self.tile_rows)) }
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
