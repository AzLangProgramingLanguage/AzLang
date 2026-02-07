trait Handler {
    fn handle(&self, request: i32) -> Option<String>;
}

struct LevelOneHandler;
struct LevelTwoHandler;
struct LevelThreeHandler;

impl Handler for LevelOneHandler {
    fn handle(&self, request: i32) -> Option<String> {
        if request < 10 {
            Some(format!("Level 1 handled: {}", request))
        } else {
            None
        }
    }
}

impl Handler for LevelTwoHandler {
    fn handle(&self, request: i32) -> Option<String> {
        if request >= 10 && request < 20 {
            Some(format!("Level 2 handled: {}", request))
        } else {
            None
        }
    }
}

impl Handler for LevelThreeHandler {
    fn handle(&self, request: i32) -> Option<String> {
        Some(format!("Level 3 handled (default): {}", request))
    }
}

struct Chain<H, N> {
    current: H,
    next: N,
}

impl<H, N> Handler for Chain<H, N>
where
    H: Handler,
    N: Handler,
{
    fn handle(&self, request: i32) -> Option<String> {
        self.current
            .handle(request)
            .or_else(|| self.next.handle(request))
    }
}

struct EmptyHandler;

impl Handler for EmptyHandler {
    fn handle(&self, _request: i32) -> Option<String> {
        None
    }
}

// Chain builder
struct ChainBuilder<H> {
    handler: H,
}

impl ChainBuilder<EmptyHandler> {
    fn new() -> Self {
        ChainBuilder {
            handler: EmptyHandler,
        }
    }
}

impl<H> ChainBuilder<H>
where
    H: Handler,
{
    fn link<NH: Handler>(self, next_handler: NH) -> ChainBuilder<Chain<NH, H>> {
        ChainBuilder {
            handler: Chain {
                current: next_handler,
                next: self.handler,
            },
        }
    }

    fn build(self) -> H {
        self.handler
    }
}

fn main() {
    // Chain-i qur
    let my_chain = ChainBuilder::new()
        .link(LevelThreeHandler) // Sonuncu (default)
        .link(LevelTwoHandler) // Ortada
        .link(LevelOneHandler) // İlk
        .build();

    let requests = vec![5, 15, 25];

    for req in requests {
        if let Some(result) = my_chain.handle(req) {
            println!("{}", result);
        }
    }
}
