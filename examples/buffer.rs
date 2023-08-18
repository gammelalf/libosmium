use std::env;

use libosmium::{Handler, ItemBuffer, ItemRef, Node};

struct NodeBuffer {
    buffer: ItemBuffer,
}

impl Handler for NodeBuffer {
    fn node(&mut self, node: &Node) {
        self.buffer.push(node);
    }
}

fn main() -> Result<(), String> {
    let file = env::args()
        .skip(1)
        .next()
        .ok_or("Missing file".to_string())?;

    let mut handler = NodeBuffer {
        buffer: ItemBuffer::new(),
    };

    handler
        .apply(&file)
        .map_err(|cstr| cstr.to_string_lossy().to_string())?;

    for node in handler.buffer.iter() {
        if let Some(ItemRef::Node(node)) = node.cast() {
            if !node.tags().is_empty() {
                println!("{:?}", node.tags())
            }
        } else {
            unreachable!("The buffer was only populated with nodes");
        }
    }

    Ok(())
}
