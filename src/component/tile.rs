use crate::model::Tile as TileModel;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Tile {
    tile: TileModel,
    offset: (i32, i32),
    on_clicked: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone)]
pub struct Prop {
    pub tile: TileModel,
    pub offset: (i32, i32),
    pub on_clicked: Option<Callback<()>>,
}

impl Default for Prop {
    fn default() -> Self {
        Self {
            tile: TileModel::default(),
            offset: (0, 0),
            on_clicked: None,
        }
    }
}

impl Component for Tile {
    type Message = Msg;
    type Properties = Prop;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Tile {
            tile: props.tile,
            offset: props.offset,
            on_clicked: props.on_clicked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut callback) = self.on_clicked {
                    callback.emit(());
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let c = self.tile != props.tile || self.offset != props.offset;
        self.tile = props.tile;
        self.offset = props.offset;
        c
    }
}

impl Renderable<Tile> for Tile {
    fn view(&self) -> Html<Self> {
        // make url
        let url = format!(
            "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
            z = self.tile.z,
            x = self.tile.x,
            y = self.tile.y
        );

        let offset_style = format!("left: {}px; top: {}px;", &self.offset.0, &self.offset.1);

        html! {
            <img class="re-tile", src={&url}, style={&offset_style},/>
        }
    }
}
