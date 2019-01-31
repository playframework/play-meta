---
name: "🚢 Play release"
about: Tracking issue for a Play release
title: Release Play x.y.z
labels: release
assignees: ''

---

## Communicate the release

Better to do this in advance:

- [ ] Make sure that other teams inside Lightbend are aware of the upcoming release, even if it is a minor/patch one
  - [ ] [Lightbend Telemetry](https://developer.lightbend.com/docs/telemetry/current/home.html) Team
  - [ ] [Lightbend Platform](https://www.lightbend.com/lightbend-platform) Team
  - [ ] Akka Team

## Do the Release <https://github.com/lightbend/play-meta/blob/master/releasing/play.md>

- [ ] [Release projects that Play depends on][]
  - [ ] release [play-json][]
  - [ ] release [play-ws][]
  - [ ] release [twirl][]

- [ ] [Release Play itself][]

- [ ] [Release external modules][]
  - [ ] release [play-slick][]
  - [ ] release [play-ebean][]
  - [ ] release [scalatestplus-play][]

- [ ] [Release omnidoc][]

- [ ] [Update playframework templates and seeds][]
- [ ] [Update Example Code Service][]
- [ ] [Update playframework.com][]

- [ ] [Announce][]
  - [ ] Write a blog post on <https://playframework.ghost.io/ghost/24> about the release.
  - [ ] Write a topic on <https://discuss.lightbend.com/>
  - [ ] Write a release on <https://github.com/playframework/playframework/releases>
  - [ ] Send an internal email to eng-updates
  - [ ] Tweet about the new release.

[Release projects that Play depends on]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-0---release-projects-that-play-depends-on-play-json-play-ws-twirl
[Release Play itself]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-1---release-play-itself
[Release external modules]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-2---release-external-modules-play-slick-play-ebean-scalatestplus-play
[Release omnidoc]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-3---release-omnidoc
[Update playframework templates and seeds]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-4---update-playframework-templates-and-seeds
[Update Example Code Service]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-5---update-example-code-service
[Update playframework.com]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-6---update-playframeworkcom
[Announce]: https://github.com/lightbend/play-meta/blob/master/releasing/play.md#step-7---announce

[play-ebean]: https://github.com/playframework/play-ebean
[play-json]: https://github.com/playframework/play-json
[play-slick]: https://github.com/playframework/play-slick
[play-ws]: https://github.com/playframework/play-ws
[scalatestplus-play]: https://github.com/playframework/scalatestplus-play
[twirl]: https://github.com/playframework/twirl
