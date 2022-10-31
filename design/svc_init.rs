use progenitor::inst::{Factory, Store, Server};

use foo_proj::common::{
    model::{Foo, Bar},
    effects::complex_common_foo_update
};

use super::complex_isolated_bar_update;

// 
pub struct SceneStores {
    foo: Store<Foo>,
    bar: Store<Bar>
}

pub struct Scene {
    stores: SceneStores,
    comm: 
}

fn main() {
    let factory = progenitor::inst::Factory::from_env();

    let scene = Scene{

    }
}