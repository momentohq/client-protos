pub mod protosocket {
    pub mod common {
        include!("protosocket/common.rs");
    }

    pub mod cache {
        include!("protosocket/cache.rs");
    }

    pub mod message_impls {
        include!("protosocket/message_impls.rs");
    }
}

pub mod permission_messages {
  include!("permission_messages.rs");
}

pub mod common {
  include!("common.rs");
}

pub mod auth {
  include!("auth.rs");
}

pub mod token {
  include!("token.rs");
}

pub mod cache_client {
  include!("cache_client.rs");

  pub mod pubsub {
    include!("cache_client.pubsub.rs");
  }
}

pub mod control_client {
  include!("control_client.rs");
}

pub mod leaderboard {
  include!("leaderboard.rs");
}

pub mod function_types {
  include!("function_types.rs");
}

pub mod function {
  include!("function.rs");
}
