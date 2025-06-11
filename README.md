# regex_macro

Allows for named groups in regular expressions to be generated from special files ending in `.re`.

---

## Usage

### Initialization

- Call the `load_regex_files!` macro on the directory that contains regex files.
- Each file is loaded with the `m` and `x` flags
  - `m` (multiline): `^` and `$` match the start and end of lines, not just the entire input.
  - `x` (extended): allows whitespace and comments starting with `#` inside the regex.

### Each regex file is a structure with 4 main implementations

1. `from_str(text: &str)` Finds a single match within the given string.
2. `from_file(filename: &str)` Finds a single match after opening the given filename.
3. `vec_from_str(text: &str)` Extracts all the matches in a given string.
4. `vec_from_file(filename: &str)` Extracts all the matches after opening the given filename.

The abstract structure for matches within the regex_macro library contains `start_pos`, `end_pos`, and public fields for each named group inside of the `.re` file. Each field then contains a `start_pos`, `end_pos`, and `val` field.

### Tips

- Use struct style naming convention for `.re` files. i.e. `Event.re` rather than `event.re`

### Benefits

- Files load automatically into precompiled code with macros which allows rust-analyzer to use provide type hints.
- Regular expressions can be split over multiple lines and use comments to improve readability for regular expressions.

---

## The events example

### event.rs

```rust
use regex_macro::load_regex_files;

fn main() {
    load_regex_files!("tests/regex");
    let events = Event::vec_from_file("tests/samples/events.txt").unwrap();
    println!("{}", serde_json::to_string_pretty(&events).unwrap());
}
```

### events.txt

```text
🧠 Focused Dev + Growth Schedule

Time Activity
7:30–8:00 AM Wake up + hydrate + light stretch
8:00–8:45 AM Gym (weights or intervals)
9:00–9:30 AM Breakfast + cooldown + planning
9:30–12:00PM Deep coding (core project)
12:00–12:45PM Lunch + decompress
12:45–2:30PM Coding or meetings if needed
2:30–3:00 PM Break or walk
3:00–5:00 PM Learning block (Rust, AI, etc)
5:00–6:00 PM Chill / side project / cleanup
6:00–7:00 PM Dinner + relax
7:00–9:00 PM Optional: build/test/hack ideas
9:00–10:00 PM Wind down (read, steam room, etc)
10:00–10:30PM Journal + sleep prep
```

### Event.re

```text
(?<start>\d{1,2}:\d{1,2})
.
(?<end>\d{1,2}:\d{1,2})
\s{0,2}
(?<ampm>[AMPM]{2})
.
(?<activity>.+)
```

### Parsed items

```json
[
  {
    "start_pos": 50,
    "end_pos": 98,
    "start": {
      "start_pos": 50,
      "end_pos": 54,
      "val": "7:30"
    },
    "end": {
      "start_pos": 57,
      "end_pos": 61,
      "val": "8:00"
    },
    "ampm": {
      "start_pos": 62,
      "end_pos": 64,
      "val": "AM"
    },
    "activity": {
      "start_pos": 65,
      "end_pos": 98,
      "val": "Wake up + hydrate + light stretch"
    }
  },
  {
    "start_pos": 99,
    "end_pos": 140,
    "start": {
      "start_pos": 99,
      "end_pos": 103,
      "val": "8:00"
    },
    "end": {
      "start_pos": 106,
      "end_pos": 110,
      "val": "8:45"
    },
    "ampm": {
      "start_pos": 111,
      "end_pos": 113,
      "val": "AM"
    },
    "activity": {
      "start_pos": 114,
      "end_pos": 140,
      "val": "Gym (weights or intervals)"
    }
  },
  {
    "start_pos": 141,
    "end_pos": 187,
    "start": {
      "start_pos": 141,
      "end_pos": 145,
      "val": "9:00"
    },
    "end": {
      "start_pos": 148,
      "end_pos": 152,
      "val": "9:30"
    },
    "ampm": {
      "start_pos": 153,
      "end_pos": 155,
      "val": "AM"
    },
    "activity": {
      "start_pos": 156,
      "end_pos": 187,
      "val": "Breakfast + cooldown + planning"
    }
  },
  {
    "start_pos": 188,
    "end_pos": 229,
    "start": {
      "start_pos": 188,
      "end_pos": 192,
      "val": "9:30"
    },
    "end": {
      "start_pos": 195,
      "end_pos": 200,
      "val": "12:00"
    },
    "ampm": {
      "start_pos": 200,
      "end_pos": 202,
      "val": "PM"
    },
    "activity": {
      "start_pos": 203,
      "end_pos": 229,
      "val": "Deep coding (core project)"
    }
  },
  {
    "start_pos": 230,
    "end_pos": 264,
    "start": {
      "start_pos": 230,
      "end_pos": 235,
      "val": "12:00"
    },
    "end": {
      "start_pos": 238,
      "end_pos": 243,
      "val": "12:45"
    },
    "ampm": {
      "start_pos": 243,
      "end_pos": 245,
      "val": "PM"
    },
    "activity": {
      "start_pos": 246,
      "end_pos": 264,
      "val": "Lunch + decompress"
    }
  },
  {
    "start_pos": 265,
    "end_pos": 308,
    "start": {
      "start_pos": 265,
      "end_pos": 270,
      "val": "12:45"
    },
    "end": {
      "start_pos": 273,
      "end_pos": 277,
      "val": "2:30"
    },
    "ampm": {
      "start_pos": 277,
      "end_pos": 279,
      "val": "PM"
    },
    "activity": {
      "start_pos": 280,
      "end_pos": 308,
      "val": "Coding or meetings if needed"
    }
  },
  {
    "start_pos": 309,
    "end_pos": 337,
    "start": {
      "start_pos": 309,
      "end_pos": 313,
      "val": "2:30"
    },
    "end": {
      "start_pos": 316,
      "end_pos": 320,
      "val": "3:00"
    },
    "ampm": {
      "start_pos": 321,
      "end_pos": 323,
      "val": "PM"
    },
    "activity": {
      "start_pos": 324,
      "end_pos": 337,
      "val": "Break or walk"
    }
  },
  {
    "start_pos": 338,
    "end_pos": 383,
    "start": {
      "start_pos": 338,
      "end_pos": 342,
      "val": "3:00"
    },
    "end": {
      "start_pos": 345,
      "end_pos": 349,
      "val": "5:00"
    },
    "ampm": {
      "start_pos": 350,
      "end_pos": 352,
      "val": "PM"
    },
    "activity": {
      "start_pos": 353,
      "end_pos": 383,
      "val": "Learning block (Rust, AI, etc)"
    }
  },
  {
    "start_pos": 384,
    "end_pos": 429,
    "start": {
      "start_pos": 384,
      "end_pos": 388,
      "val": "5:00"
    },
    "end": {
      "start_pos": 391,
      "end_pos": 395,
      "val": "6:00"
    },
    "ampm": {
      "start_pos": 396,
      "end_pos": 398,
      "val": "PM"
    },
    "activity": {
      "start_pos": 399,
      "end_pos": 429,
      "val": "Chill / side project / cleanup"
    }
  },
  {
    "start_pos": 430,
    "end_pos": 459,
    "start": {
      "start_pos": 430,
      "end_pos": 434,
      "val": "6:00"
    },
    "end": {
      "start_pos": 437,
      "end_pos": 441,
      "val": "7:00"
    },
    "ampm": {
      "start_pos": 442,
      "end_pos": 444,
      "val": "PM"
    },
    "activity": {
      "start_pos": 445,
      "end_pos": 459,
      "val": "Dinner + relax"
    }
  },
  {
    "start_pos": 460,
    "end_pos": 506,
    "start": {
      "start_pos": 460,
      "end_pos": 464,
      "val": "7:00"
    },
    "end": {
      "start_pos": 467,
      "end_pos": 471,
      "val": "9:00"
    },
    "ampm": {
      "start_pos": 472,
      "end_pos": 474,
      "val": "PM"
    },
    "activity": {
      "start_pos": 475,
      "end_pos": 506,
      "val": "Optional: build/test/hack ideas"
    }
  },
  {
    "start_pos": 507,
    "end_pos": 556,
    "start": {
      "start_pos": 507,
      "end_pos": 511,
      "val": "9:00"
    },
    "end": {
      "start_pos": 514,
      "end_pos": 519,
      "val": "10:00"
    },
    "ampm": {
      "start_pos": 520,
      "end_pos": 522,
      "val": "PM"
    },
    "activity": {
      "start_pos": 523,
      "end_pos": 556,
      "val": "Wind down (read, steam room, etc)"
    }
  },
  {
    "start_pos": 557,
    "end_pos": 593,
    "start": {
      "start_pos": 557,
      "end_pos": 562,
      "val": "10:00"
    },
    "end": {
      "start_pos": 565,
      "end_pos": 570,
      "val": "10:30"
    },
    "ampm": {
      "start_pos": 570,
      "end_pos": 572,
      "val": "PM"
    },
    "activity": {
      "start_pos": 573,
      "end_pos": 593,
      "val": "Journal + sleep prep"
    }
  }
]
```
