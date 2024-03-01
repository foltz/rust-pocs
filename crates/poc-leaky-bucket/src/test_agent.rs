use crate::test_client::TestClient;

pub struct TestAgent<'a> {
    client: &'a TestClient<'a>,
    agent_id: i32,
}

impl<'a> TestAgent<'a> {
    pub fn new(client: &'a TestClient, agent_id: i32) -> Self {
        Self {
            client,
            agent_id,
        }
    }

    pub async fn run(&mut self) {
        // - TODO: run client.get - over and over....
        // tokio::spawn(async move {
        //     let micros = self.client.get().await;
            loop {
                let micros = self.client.get().await;
                println!("agent: {} - {micros}", self.agent_id,);
            }
        // });

    }
}