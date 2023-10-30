# agent pikafish

```rust
use pikagent;

fn main() {
    let bin = "/Users/atopx/opensource/chesscc/Pikafish/src/pikafish";
    let mut engine = pikagent::Engine::new(bin).unwrap();
    println!("{}", engine.uci());
    engine.new_game(6, 32).unwrap();
    let (best, ponter) = engine.search(None, Some(10), None);
    println!("best {:?}", best);
    println!("ponter {:?}", ponter);
    println!("fen {}", engine.to_fen());

    let (best, ponter) = engine.move_search(&best.unwrap(), 20, 2000);
    println!("best {:?}", best);
    println!("ponter {:?}", ponter);
    println!("fen {}", engine.to_fen());
    engine.close().unwrap();
}
```
