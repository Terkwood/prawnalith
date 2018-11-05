/// Note that fq_topic is a fully qualified topic, i.e. `projects/{project_id}/topics/{topic_name}`
pub struct PubSubContext {
    pub fq_topic: String,
    pub client: PubSubClient,
}

/// Note that this pub sub client specifically uses the
/// "service account access" credentials strategy, and
/// *not* an OAuth2 credentials strategy.
///
/// google_pubsub1::PubSub is capable of supporting an
/// OAuth credentials strategy, but we don't need it
/// here, for this simple backend application.
pub type PubSubClient =
    google_pubsub1::Pubsub<hyper::Client, yup_oauth2::ServiceAccountAccess<hyper::Client>>;
