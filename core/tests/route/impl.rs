use super::*;

impl ServerHook for TestRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self {
            data: String::new(),
        }
    }

    async fn handle(mut self, _: &mut Stream, _: &mut Context) -> Status {
        self.data = String::from("test");
        Status::Continue
    }
}
