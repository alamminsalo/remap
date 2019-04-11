use crate::component::Tile;
use crate::model::{Tile as TileModel, Viewport};
use itertools::Itertools;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

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
        // group by rows
        let mut tile_rows = vec![];
        for (_, group) in &self.vw.tiles().group_by(|t| t.y) {
            tile_rows.push(group.collect::<Vec<TileModel>>());
        }
        // inner div xy-offset
        let first_tile = TileModel::from_lonlat(self.vw.lon_min, self.vw.lat_max, self.vw.z);
        let offset = self.vw.pixel_offset(&first_tile);
        html! {
            <div class="remap-tile-grid", style={format!("left: {}px; top: {}px", &offset.0, &offset.1)},>
                { for tile_rows.into_iter().map(tile_row) }
            </div>
        }
    }
}

fn tile_row(tiles: Vec<TileModel>) -> Html<Grid> {
    html! {
        <div class="remap-tile-row",>
        { for tiles.into_iter().map(view_tile) }
        </div>
    }
}

fn view_tile(tile: TileModel) -> Html<Grid> {
    html! {
        <Tile: tile=tile,/>
    }
}
