# lignin-schema Changelog

<!-- markdownlint-disable no-trailing-punctuation -->

## next

TODO: Date

* **Breaking:**
  * All functions now accept only a single parameter indicating whether there is any content.
  * All functions now only return the element's tag name as `&'static str`.

## 0.0.4

2021-01-30

* **Breaking:**
  * Upgraded `lignin` dependency from 0.0.3 to 0.0.5
    > to support fallible allocation/bump object initialisation downstream.

## 0.0.3

2021-01-03

* **Breaking:**
  * Upgraded `lignin` dependency to 0.0.3

## 0.0.2

2020-11-30

* **Breaking:**
  * Removed "remnants" feature (always enabled now)
  * Upgraded `lignin` dependency to 0.0.2

## 0.0.1

2020-10-03

Initial unstable release
