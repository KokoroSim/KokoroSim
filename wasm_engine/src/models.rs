pub mod severi;
pub mod inada;
pub mod courtemanche;
pub mod tentusscher;

// Aqui ficará o gerenciador de estado global que orquestra as 4 células
pub struct HeartSystem {
    pub sa_node: severi::SeveriCell,
    // pub av_node: inada::InadaCell,
    // pub atrium: courtemanche::AtriumCell,
    // pub ventricle: tentusscher::VentricleCell,
    pub time: f64,
}

impl HeartSystem {
    pub fn new() -> Self {
        Self {
            sa_node: severi::SeveriCell::new(),
            time: 0.0,
        }
    }
}
