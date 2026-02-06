struct Request {
    user: Option<String>,
    role: Option<String>,
}

trait Handler {
    fn handle(&self, req: &Request);
}

struct Final;

impl Handler for Final {
    fn handle(&self, _req: &Request) {
        println!("🎉 Request successfully processed!");
    }
}

struct Log<N: Handler> {
    next: N,
}

impl<N: Handler> Handler for Log<N> {
    fn handle(&self, req: &Request) {
        println!("📝 Logging request for user: {:?}", req.user);
        self.next.handle(req);
    }
}

struct Role<N: Handler> {
    next: N,
}

struct Auth<N: Handler> {
    next: N,
}

impl<N: Handler> Handler for Auth<N> {
    fn handle(&self, req: &Request) {
        if req.user.is_none() {
            println!("❌ No user found. Stop chain.");
            return;
        }

        println!("✅ Auth passed");
        self.next.handle(req);
    }
}

impl<N: Handler> Handler for Role<N> {
    fn handle(&self, req: &Request) {
        if req.role.as_deref() != Some("admin") {
            println!("❌ Not admin. Stop chain.");
            return;
        }

        println!("✅ Role check passed");
        self.next.handle(req);
    }
}

fn main() {
    let chain = Auth {
        next: Role {
            next: Log { next: Final },
        },
    };

    let req = Request {
        user: Some("Prestgg".into()),
        role: Some("admin".into()),
    };

    chain.handle(&req);
}
