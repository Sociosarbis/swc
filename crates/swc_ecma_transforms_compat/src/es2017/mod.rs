pub use self::async_to_generator::{async_to_generator, Config as AsyncToGeneratorConfig};
use serde::Deserialize;
use swc_common::Mark;
use swc_ecma_visit::Fold;

mod async_to_generator;

pub fn es2017(global_mark: Mark, c: Config) -> impl Fold {
    async_to_generator(c.async_to_generator, global_mark)
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub async_to_generator: AsyncToGeneratorConfig,
}
