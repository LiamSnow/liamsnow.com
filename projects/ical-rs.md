---
title: ical-rs
desc: A flexbile and typed iCalendar (RFC 5545) library 
date: 2024-11-10
homepage: true
---

# Background
## Why
For a long time I've used python CLI tools ([todoman](https://github.com/pimutils/todoman)
and [khal](https://github.com/pimutils/khal)) to interact with
my CalDAV calendar and todo lists in Linux. I'm pretty happy with them,
but I could see a lot of room for improvements, especially with todoman.

These tools actually don't interact with CalDAV directory, but instead
work directly on iCalendar (`.ics`) file and require [vdirsyncer](https://github.com/pimutils/vdirsyncer)
(now replaced by [pimsync](https://pimsync.whynothugo.nl/)) to sync a CalDAV
server to local iCalendar files.

Now, I'm not affraid to say that I'm quite a Python hater for big projects.
So I didn't want to go out and rewrite `todoman` in Python again.
I decided to go with my favorite choice here which was Rust.

## Rust iCalendar Libraries
There are two rust libraries that exist:
 1. [ical-rs](https://github.com/Peltoche/ical-rs): cool library
but archived in 2024. Seems to be a great library for making iCalendar
files, but not for reading or editing ones.
 2. [icalendar](https://github.com/hoodie/icalendar): cool library, well maintained but again seems to not focus on editing

Now realistically, I could have been very successful with either library.

## The Real Reason Why
I was super interested in this old format, it was cool to be that it had
a super defined spec (RFC 5545). I was also taking a programming langauges
class, I needed a project for the masters extension that handled parsing.

# Results
After probably rewriting this library 3 times, I can say I'm almost happy
with the result. I taught me a LOT about using Rust macros, and also
their disadvantages. I think having a system like macros in Zig would have
been best here. In my opinion, the ideal implementation of this library relies
on code generation in a `build.rs` (something I didn't realize was possible
at the time).


## Features

 - Full implemenation of every ICalendar type
 - Generated methods for every ICalendar property with all allowed types
 - Support for X & IANA properties and parameters

## Usage

### Modify Existing
I built the library around this idea, I would I say
I definetly achieved it. Here you can see that in 4
lines of code, I can parse an iCalendar string, edit
it and then save it back to a string. 

```rust
let ics_str = "BEGIN:VCALENDAR...";
let mut vcal = ICalComponent::from_ics(ics_str)?;
let vtodo = vcal.expect_vtodo();
vtodo.summary("New Summary".to_string());
let new_ics_str = vcal.to_ics();
```

### Make New
```rust
let dtstamp = Tz::America__New_York.with_ymd_and_hms(1992, 12, 17, 12, 34, 56)?;
let vcal = ICalComponent::vcalendar_with_vtodo(
    ICalComponent::empty()
        .uid_random()
        .dtstamp(dtstamp.into())
        .percent_complete(10)
        .build()
);
let ics_str = vcal.to_ics();
```

### X & IANA Properties

Convert Value:
```rust
let in_ics = r#"BEGIN:VCALENDAR
X-EXAMPLE:19921217T123456
END:VCALENDAR"#;
let mut vcal = ICalComponent::from_ics(&in_ics)?;
let x_example = vcal.get_prop("X-EXAMPLE")?
    .convert_value::<ICalDateTime>()?;
```

Read Later:
```rust
let value = vcal.get_prop("X-EXAMPLE")?
    .get_as::<ICalDateTime>()?);
println!("{}", value); // 1992-12-17 12:34:56
```
