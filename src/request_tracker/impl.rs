use crate::*;

impl RequestIdGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&self) -> String {
        let counter: u64 = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let timestamp: u64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        format!("{:x}-{:x}", timestamp, counter)
    }
}

impl RequestContext {
    pub fn new(
        request_id: String,
        method: String,
        path: String,
        remote_addr: OptionString,
        user_agent: OptionString,
    ) -> Self {
        Self {
            request_id,
            method,
            path,
            remote_addr,
            user_agent,
            start_time: std::time::Instant::now(),
        }
    }

    pub fn get_request_id(&self) -> &str {
        &self.request_id
    }

    pub fn get_method(&self) -> &str {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_remote_addr(&self) -> &OptionString {
        &self.remote_addr
    }

    pub fn get_user_agent(&self) -> &OptionString {
        &self.user_agent
    }

    pub fn get_duration(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

static REQUEST_ID_GENERATOR: std::sync::LazyLock<RequestIdGenerator> =
    std::sync::LazyLock::new(|| RequestIdGenerator::new());

tokio::task_local! {
    static REQUEST_CONTEXT: RequestContextRef;
}

pub fn generate_request_id() -> String {
    REQUEST_ID_GENERATOR.generate()
}

pub async fn with_request_context<F, R>(context: RequestContextRef, future: F) -> R
where
    F: std::future::Future<Output = R>,
{
    REQUEST_CONTEXT.scope(context, future).await
}

pub fn get_current_request_context() -> Option<RequestContextRef> {
    REQUEST_CONTEXT.try_with(|ctx| ctx.clone()).ok()
}
