# Prawnalith Heads-Up Display

It's a small view of statuses for all the tanks.

## Example code

Return value example
```javascript
[
     {
           "tank": { "id": 1, "name": "The Mothership" },
           "temp": { "f": 82.18, "c": 27.88 },
           "ph": { "val": 8.11, "mv": 500.15 }
     },
     {
           "tank": { "id": 2, "name": "The Pond" },
           "temp": { "f": 83.1, "c": 28.39 },
           "ph": { "val": 7.98, "mv": 488.33 }
     }
]
```

Rust struct example
```rust


/// A struct to hold some data from the HTTP request
/// for temp/ph info.
#[derive(Debug, Serialize, Deserialize)]
pub struct TankStatus {
   pub tank: Tank,
   pub temp: Temp,
   pub ph: Ph,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tank {
   pub id: i32,
   pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {
   pub f: f32,
   pub c: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ph {
   pub val: f32,
   pub mv: f32,
}
```