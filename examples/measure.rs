#![allow(dead_code)]

use std::env;

use libosmium::handler::Handler;
use libosmium::Node;

#[derive(Debug)]
struct BoundingBox {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            min_lat: f64::INFINITY,
            max_lat: f64::NEG_INFINITY,
            min_lon: f64::INFINITY,
            max_lon: f64::NEG_INFINITY,
        }
    }
}

impl Handler for BoundingBox {
    fn node(&mut self, node: &Node) {
        let loc = node.location();
        if loc.is_valid() {
            let lon = loc.lon();
            let lat = loc.lat();
            if self.max_lon < lon {
                self.max_lon = lon;
            }
            if self.min_lon > lon {
                self.min_lon = lon;
            }
            if self.max_lat < lat {
                self.max_lat = lat;
            }
            if self.min_lat > lat {
                self.min_lat = lat;
            }
        }
    }
}

#[derive(Debug)]
struct Center {
    lat: f64,
    lon: f64,
}

impl From<BoundingBox> for Center {
    fn from(bbox: BoundingBox) -> Self {
        Center {
            lat: (bbox.min_lat + bbox.max_lat) / 2.0,
            lon: (bbox.min_lon + bbox.max_lon) / 2.0,
        }
    }
}

fn main() -> Result<(), String> {
    let file = env::args()
        .skip(1)
        .next()
        .ok_or("Missing file".to_string())?;

    let mut handler = BoundingBox::default();
    handler
        .apply(&file)
        .map_err(|cstr| cstr.to_string_lossy().to_string())?;

    println!("{handler:?}");
    println!("{:?}", Center::from(handler));

    Ok(())
}
