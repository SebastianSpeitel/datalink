use datalink::request::ErasableRequest;
use datalink::{Data, Link, Request, Visitor, link::LinkBuilder};
use std::sync::mpsc;
use std::thread;

/// A Visitor that takes values during a datalink query and sends them over an `mpsc::Sender`.
pub struct SenderVisitor(mpsc::Sender<String>);

impl Visitor for SenderVisitor {
    #[inline]
    fn visit_str(&mut self, value: &str) {
        let _ = self.0.send(value.to_string());
    }

    #[inline]
    fn visit_str_owned(&mut self, value: Box<str>) {
        let _ = self.0.send(value.into_string());
    }
}

/// A Request that yields our `SenderVisitor` to stream values.
pub struct ChannelRequest(mpsc::Sender<String>);

impl Request for ChannelRequest {

    #[inline]
    fn schema(&self) -> impl datalink::schema::Schema {
        datalink::schema::ONLY_LINKS
    }

    #[inline]
    fn visitor(&mut self) -> impl Visitor {
        
    }

    #[inline]
    fn as_erased(&mut self) -> impl ErasableRequest {
        // Return unit `()` which acts as an empty ErasableRequest for this example,
        // since we don't intend to perform type-erased operations here.
        ()
    }

    #[inline]
    fn provide_link_unchecked<L: Link>(&mut self, link: L) {
        // Since we are receiving links (relationships) instead of values,
        // we instruct the link to query its target item into us!
        // This will result in our visitor catching the actual primitive value.
        link.query_target(self);
    }
}

/// Our custom data source that streams multiple strings via Links
pub struct MyStream {
    items: Vec<String>,
}

impl Data for MyStream {
    #[inline(never)]
    fn query(&self, mut request: impl Request) {
        // We provide items as Links, which is the idiomatic way in Datalink
        // to represent items in a sequence. `LinkBuilder::new_ownable`
        // constructs a relationship yielding a reference, or an owned value if requested.
        for item in &self.items {
            request.provide_link_with(|| LinkBuilder::new_ownable(item));
        }
    }
}

fn main() {
    println!("Setting up channel...");
    let (tx, rx) = mpsc::channel();

    // Spawn a background thread that acts as our data producer
    thread::spawn(move || {
        // Imagine this is some complex `Data` object we are querying.
        // Datalink's push-based model shines here because it can freely
        // iterate its internal structure and push to our streaming request.
        let my_data = MyStream {
            items: vec![
                "Hello".to_string(),
                "from".to_string(),
                "another".to_string(),
                "thread".to_string(),
            ],
        };

        println!("Background thread: querying data...");
        // This will traverse the sequence, our Request will receive Links,
        // extract the target string from them, and yield it to the Visitor!
        let req = ChannelRequest(tx);
        my_data.query(req);
        println!("Background thread: finished querying.");
    });

    // Receive the streaming data in the main thread
    for msg in rx {
        println!("Received: {}", msg);
    }
    println!("Stream complete!");
}
