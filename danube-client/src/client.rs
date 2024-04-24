use crate::errors::Result;
use crate::lookup_service::{LookupResult, LookupService};
use crate::producer::ProducerBuilder;
use proto::danube_client;

pub mod proto {
    include!("../../proto/danube.rs");
}

#[derive(Debug, Default)]
pub struct DanubeClient {
    url: String,
    cnx_manager: ConnectionManager,
    lookup_service: LookupSerice,
}

impl DanubeClient {
    pub fn new() -> Self {
        DanubeClient {
            url: Default::default(),
            lookup_service: LookupService::new(),
        }
    }

    //creates a Client Builder
    pub fn builder() -> DanubeClientBuilder {
        DanubeClientBuilder::default()
    }

    /// creates a Producer Builder
    pub fn new_producer(&self) -> ProducerBuilder {
        ProducerBuilder::new()
    }

    /// gets the address of a broker handling the topic
    pub async fn lookup_topic<S: Into<String>>(&self, topic: S) -> Result<LookupResult> {
        self.lookup_service
            .lookup_topic(topic)
            .await
            .map_err(|e| e.into())
    }

    pub async fn connect(&self) -> Result<()> {
        let mut client = danube_client::DanubeClient::connect(String::from(&self.url)).await?;

        let req = proto::ProducerRequest {
            request_id: 1,
            producer_id: 2,
            producer_name: "hello_producer".to_string(),
            topic: "hello_topic".to_string(),
            schema: Some(proto::Schema {
                name: "schema_name".to_string(),
                schema_data: "1".as_bytes().to_vec(),
                type_schema: 0,
            }),
        };

        let request = tonic::Request::new(req);
        let response = client.create_producer(request).await?;

        println!("Response: {:?}", response.get_ref().request_id);

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DanubeClientBuilder {
    url: String,
}

impl DanubeClientBuilder {
    pub fn service_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();

        self
    }
    pub fn build(self) -> DanubeClient {
        DanubeClient { url: self.url }
    }
}
