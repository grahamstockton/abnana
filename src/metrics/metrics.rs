use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{counter::Counter, family::Family},
};

#[derive(Debug)]
pub struct Metrics {
    pub triggers: Family<Labels, Counter>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    pub experiment_id: i64,
    pub treatment_id: String,
}

impl Metrics {
    pub fn record_trigger(&self, experiment_id: i64, treatment_id: &str) {
        self.triggers
            .get_or_create(&Labels {
                experiment_id,
                treatment_id: treatment_id.to_string(),
            })
            .inc();
    }
}
