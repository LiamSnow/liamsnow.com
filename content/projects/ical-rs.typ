#metadata((
  title: "ical-rs",
  desc: "A flexible and typed iCalendar (RFC 5545) library ",
  date: "2024-11-10",
  homepage: true
)) <page>

#import "/_shared/template.typ": post
#show: post

*NOTE:* This is one of my earlier Rust projects. I'm planning a rewrite that uses code generation in `build.rs` instead of macros for more comprehensive type coverage and better ergonomics.

= Motivation

For a while I've used Python CLI tools, #link("https://github.com/pimutils/todoman")[todoman] and #link("https://github.com/pimutils/khal")[khal], to manage my CalDAV calendar and to-do lists. While functional, todoman had limitations: no TUI, cumbersome multi-command workflows for editing, no integration with status bars like waybar, no notification system, and limited customization.

These tools work with iCalendar (`.ics`) files stored in directories and require #link("https://github.com/pimutils/vdirsyncer")[vdirsyncer] (now #link("https://pimsync.whynothugo.nl/")[pimsync]) as a middleman to sync with CalDAV servers. I wanted to build a single Rust tool that handles todos, calendar events, and talks directly to CalDAV servers without the syncing middleman.

To build this, I needed a Rust iCalendar library capable of editing files while preserving all existing data.

= The Editing Challenge

Creating iCalendar files is straightforward, but editing them presents challenges. When you modify a file, you must preserve all existing data, including properties and parameters your implementation doesn't understand. Apple Calendar, for instance, uses extensive X-properties for their features built on top of normal iCalendar ones. In most cases you need to parse a file, modify one field, and write it back with everything else remaining intact.

The iCalendar type system adds complexity. RFC 5545 defines 14 value types (DATE, DATE-TIME, TEXT, INTEGER, etc.). Each property has a default type, but the VALUE parameter can override it. For example, DTSTART defaults to DATE-TIME, but VALUE=DATE makes it a date. A proper library must parse parameters and infer the correct type automatically.

The RFC is extensive, with multiple extensions, which means handling many edge cases.

= Existing Rust Libraries

Two Rust iCalendar libraries existed when I started:
 - #link("https://github.com/Peltoche/ical-rs")[ical-rs]: solid for creating files, but archived in 2024 and not designed for editing
 - #link("https://github.com/hoodie/icalendar")[icalendar]: well-maintained and good for creating files, but doesn't handle type inference from parameters automatically

With these libraries, users must handle the type system and parameter parsing themselves. I needed something that parsed parameters and provided a typed API automatically.

= My Approach

I built a library that parses iCalendar parameters and infers types automatically, providing typed methods for each property while preserving unknown properties and parameters. The implementation uses Rust macros to generate methods for iCalendar properties, though this approach has limitations. A future rewrite using `build.rs` would enable more comprehensive type generation.

The library focuses on making editing straightforward: parse a file, modify what you need, serialize it back with everything preserved.

= Usage

== Modify Existing

```rust
let ics_str = "BEGIN:VCALENDAR...";
let mut vcal = ICalComponent::from_ics(ics_str)?;
let vtodo = vcal.expect_vtodo();
vtodo.summary("New Summary".to_string());
let new_ics_str = vcal.to_ics();
```

== Create New
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

== X & IANA Properties

```rust
let in_ics = r#"BEGIN:VCALENDAR
X-EXAMPLE:19921217T123456
END:VCALENDAR"#;
let mut vcal = ICalComponent::from_ics(&in_ics)?;
let x_example = vcal.get_prop("X-EXAMPLE")?
    .convert_value::<ICalDateTime>()?;
```

Read later:
```rust
let value = vcal.get_prop("X-EXAMPLE")?
    .get_as::<ICalDateTime>()?;
println!("{}", value); // 1992-12-17 12:34:56
```
