/// Note that fq_topic is a fully qualified topic, i.e. `projects/{project_id}/topics/{topic_name}`
pub struct PubSubContext {
    pub fq_topic: String,
    pub client: PubSubClient,
}

pub type PubSubClient =
    google_pubsub1::Pubsub<hyper::Client, yup_oauth2::ServiceAccountAccess<hyper::Client>>;
