#![allow(dead_code)]

pub enum Type {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    Tour,
}

pub enum EdgeWeightType {
    Explicit,
    Euc2D,
    Euc3D,
    Max2D,
    Max3D,
    Man2D,
    Man3D,
    Ceil2D,
    Geo,
    Att,
    XRay1,
    XRay2,
    Special,
}

pub enum EdgeWeightFormat {
    Function,
    FullMatrix,
    UpperRow,
    LowerRow,
    UpperDiagRow,
    LowerDiagRow,
    UpperCol,
    LowerCol,
    UpperDiagCol,
    LowerDiagCol,
}

pub enum EdgeDataFormat {
    EdgeList,
    AdjList,
}

pub enum NodeCoordType {
    TwoDCoord,
    ThreeDCoord,
    NoCoords,
}

impl Default for NodeCoordType {
    fn default() -> Self { NodeCoordType::NoCoords }
}

pub enum DisplayDataType {
    CoordDisplay,
    TwoDDisplay,
    NoDisplay,
}

impl Default for DisplayDataType {
    fn default() -> Self { DisplayDataType::CoordDisplay }
}

pub struct TwoDCoord {
    node_num: usize,
    x: f64,
    y: f64,
}

pub struct ThreeDCoord {
    node_num: usize,
    x: f64,
    y: f64,
    z: f64,
}

pub enum NodeCoord {
    TwoDCoord,
    ThreeDCoord,
}

pub struct NodeCoordSection(Vec<NodeCoord>);

pub struct TSPLib {
    name: String,
    r#type: Type,
    comment: String,
    dimension: usize,
    capacity: usize,
    edge_weight_type: EdgeWeightType,
    edge_weight_format: EdgeWeightFormat,
    edge_data_format: EdgeDataFormat,
    node_coord_type: NodeCoordType,
    display_data_type: DisplayDataType,
    node_coord_section: NodeCoordSection,

}
