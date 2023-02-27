pub mod google {
  pub mod rpc {
    tonic::include_proto!("google.rpc");
  }
}

pub mod auth {
  tonic::include_proto!("auth");
}

pub mod cache_client {
  tonic::include_proto!("cache_client");

  pub mod pubsub {
    tonic::include_proto!("cache_client.pubsub");
  }
}

pub mod control_client {
  tonic::include_proto!("control_client");
}
