[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua)

# dijkstra-suite

[![Crates.io Version](https://img.shields.io/crates/v/dijkstra-suite)](<https://crates.io/crates/dijkstra-suite/>)
[![docs.rs (with version)](https://img.shields.io/docsrs/dijkstra-suite/latest?label=docs.rs)](https://docs.rs/dijkstra-suite/latest/dijkstra_suite/index.html)
[![Crates.io License](https://img.shields.io/crates/l/dijkstra-suite)](https://github.com/TheShooter89/dijkstra-suite/blob/master/LICENSE)

A Dijkstra's algorithm implementation that aims to be simple to use and fast to run

> [!CAUTION]
> WIP: this crate is WORK IN PROGRESS and not ready to be used in production

## Documentation

Check the documentation with example on [docs.rs](https://docs.rs/dijkstra-suite).

## Quick Start

Add `dijkstra-suite` to your project, running

```sh
cargo add dijkstra-suite
```

or add it to your `Cargo.toml` file

```toml
[dependencies]
dijkstra-suite = "0.1.0-beta.1"
```

## Usage

Create a `Graph` and run it through `dijkstra_path()` function providing a starting and an ending node

```rust
use dijkstra_suite::dijkstra::dijkstra_path;
use dijkstra_suite::graph::Graph;
use dijkstra_suite::path::Path;
use dijkstra_suite::node::{Node, NodeConnection};

let mut graph: Graph<&str, f32> = Graph::default();
let node_a = Node {
    id: "A",
    weight: 0.0,
    neighbours: vec![
        NodeConnection {
            from: "A",
            to: "B",
            weight: 7.0,
        },
        NodeConnection {
            from: "A",
            to: "E",
            weight: 1.0,
        },
    ],
};

let node_b = Node {
    id: "B",
    weight: 0.0,
    neighbours: vec![
        NodeConnection {
            from: "B",
            to: "A",
            weight: 7.0,
        },
        NodeConnection {
            from: "B",
            to: "C",
            weight: 3.0,
        },
        NodeConnection {
            from: "B",
            to: "E",
            weight: 8.0,
        },
    ],
};

let node_c = Node {
    id: "C",
    weight: 0.0,
    neighbours: vec![
        NodeConnection {
            from: "C",
            to: "B",
            weight: 3.0,
        },
        NodeConnection {
            from: "C",
            to: "D",
            weight: 6.0,
        },
        NodeConnection {
            from: "C",
            to: "E",
            weight: 2.0,
        },
    ],
};

let node_d = Node {
    id: "D",
    weight: 0.0,
    neighbours: vec![
        NodeConnection {
            from: "D",
            to: "C",
            weight: 6.0,
        },
        NodeConnection {
            from: "D",
            to: "E",
            weight: 7.0,
        },
    ],
};

let node_e = Node {
    id: "E",
    weight: 0.0,
    neighbours: vec![
        NodeConnection {
            from: "E",
            to: "A",
            weight: 1.0,
        },
        NodeConnection {
            from: "E",
            to: "B",
            weight: 8.0,
        },
        NodeConnection {
            from: "E",
            to: "C",
            weight: 2.0,
        },
        NodeConnection {
            from: "E",
            to: "D",
            weight: 7.0,
        },
    ],
};

graph.insert(node_a.id, node_a);
graph.insert(node_b.id, node_b);
graph.insert(node_c.id, node_c);
graph.insert(node_d.id, node_d);
graph.insert(node_e.id, node_e);

let result = dijkstra_path(&graph, "B", "D");

let expected_path: Path<&str, f32> = Path {
    weight: 9.0,
    steps: vec!["B", "C", "D"],
};

assert_eq!(result.unwrap(), expected_path)

```

## License

[GNU GPL-3](https://choosealicense.com/licenses/gpl-3.0/)

## Authors

written with 💛️💙️ by Tanque

- [@TheShooter89](https://www.github.com/TheShooter89)

## `tanque` Stands With Ukraine 🇺🇦️

    "Freedom doesn't come cheap"

`tanque` stands with people of Ukraine in their fight against the brutal russian aggression and unrightful occupation of their homeland

`tanque` stands with people of Ukraine in their fight for **Freedom**, for **Peace**, for **Self-Determination**, for **Happiness**

`tanque` stands with this generation of young ukrainians robbed away of their youth by the war, who will have to find the strength to get up once again and rebuild from the rubbles

####

*By your side, for as long as it takes* 💪️

        Slava Ukraini 🇺🇦️

### Donate

Please contribute and donate through official government channels or globally-know remarkable institutions:

- **UNITED24**: Institutinal fundraising, charity and media platform of Ukrainian Government. It's possible to donate for food, medicine, medical assistance, refugees support and more
  
    [U24 official site](https://u24.gov.ua/)

- **Medecins Sans Frontieres**: Life-saving medical assistance both in war and peace time, all over the world
  
    [MSF official site](https://www.msf.org/ukraine)

- **Protect A Volunteer**: Independent matching platform to support a Volunteer on the frontline
  
    [Protect a Volunteer site](https://protectavolunteer.com/)

Or use below badge:

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua)

---

    humans die, but IDEAS are bulletproof
    🇺🇦️ ️🇪🇺️ 🏳️‍🌈️
