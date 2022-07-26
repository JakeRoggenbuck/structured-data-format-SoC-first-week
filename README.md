# structured-data-format-SoC-first-week
Summer Of Code (SoC) First Week - Structured Data Format

## This week's challenge
Build a new and unique structured data format similar to JSON or TOML with at least two intentional design decisions that improve upon some aspect of an existing data structure format.

## How to participate
- Fork this repo and start your work in your fork of the project!
- Projects are due on Sunday the 31st at 9pm PDT.

## Requirements
- Write two paragraphs about your new data format (including two intentional design decisions and why)
- Give three examples of this data format being used
- Make a basic interpreter for this new file format and a library in any language with at least two functions ([loads](https://www.geeksforgeeks.org/json-loads-in-python/), [dumps](https://www.geeksforgeeks.org/json-dumps-in-python/)).
  - Optionally add [load](https://www.geeksforgeeks.org/json-load-in-python/) and [dump](https://www.geeksforgeeks.org/json-dump-in-python/) (More info below)
- Have fun and be creative, make this a real thing that you will use in the future

```py
# Loads string of your new format into a native object
obj = myformat.loads("text")

# Write a string of your format from native object
str = myformat.dumps(obj)
```

## Resources
- https://toml.io/en/
- https://yaml.org/
- https://www.json.org/json-en.html
- https://www.geeksforgeeks.org/json-load-in-python/
- https://www.geeksforgeeks.org/json-dump-in-python/
- https://www.geeksforgeeks.org/json-loads-in-python/
- https://www.geeksforgeeks.org/json-dumps-in-python/

### What is Summer of Code?
Summer of Code is an initiative to encourage people to build cool and useful software over the summer. And most importantly to learn and have fun!!
