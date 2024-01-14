# leblad

A rust library providing a list of Algerian administrative areas with many useful APIs, based on dzcode-io/leblad.

## Usage

```sh
cargo add leblad
```

# Example

```rs
use leblad::get_wilaya_list;

fn main() {
    let wilayas = get_wilaya_list();
    assert_eq!(wilayas[0].name, "Adrar");
}
```
