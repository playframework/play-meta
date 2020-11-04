---
name: "🚢 Play release"
about: Tracking issue for a Play release
title: Release Play x.y.z
labels: release
assignees: ''

---

## Preparing the release

Better to do this in advance:

* [ ] Make sure that other teams inside Lightbend are aware of the upcoming release, even if it is a minor/patch one
  * [ ] [Lightbend Telemetry](https://developer.lightbend.com/docs/telemetry/current/home.html) Team
  * [ ] Akka Team
* [ ] [Triage issues][]

## Do the Release ([Full document](https://github.com/playframework/play-meta/blob/master/releasing/play.md))

* [ ] [Release projects that Play depends on][]
  * [ ] release [play-json][]
    * [ ] Look for [PRs][play-json/prs] that should be merged.
    * [ ] Look at [`status:needs-backport`][play-json/backport], [`status:needs-forwardport`][play-json/forwardport] issues/PRs (including closed ones).
    * [ ] Look at issues/PRs tagged [milestone][play-json/milestones] version (including closed ones).
    * [ ] Update any dependencies that are needed. (e.g JDK, Scala)
  * [ ] release [play-ws][]
    * [ ] Look for [PRs][play-ws/prs] that should be merged.
    * [ ] Look at [`status:needs-backport`][play-ws/backport], [`status:needs-forwardport`][play-ws/forwardport] issues/PRs (including closed ones).
    * [ ] Look at issues/PRs tagged [milestone][play-ws/milestones] version (including closed ones).
    * [ ] Update any dependencies that are needed. (e.g JDK, Scala)
    * [ ] Make sure play-ws uses new play-json
  * [ ] release [twirl][]
    * [ ] Look for [PRs][twirl/prs] that should be merged.
    * [ ] Look at [`status:needs-backport`][twirl/backport], [`status:needs-forwardport`][twirl/forwardport], [`status:needs-backport-1.3`][twirl/backport-1.3] issues/PRs (including closed ones).
    * [ ] Look at issues/PRs tagged [milestone][twirl/milestones] version (including closed ones).
    * [ ] Update any dependencies that are needed. (e.g JDK, Scala)

* [ ] [Release Play itself][]
    * [ ] Look for [PRs][play/prs] that should be merged.
    * [ ] Look at [`status:needs-backport`][play/backport], [`status:needs-forwardport`][play/forwardport], [`status:needs-backport-2.6`][play/backport-2.6] issues/PRs (including closed ones).
    * [ ] Look at issues/PRs tagged [milestone][play/milestones] version (including closed ones).
    * [ ] Update any dependencies that are needed. (e.g JDK, Scala)
    * [ ] Make sure play uses new play-json, play-ws, twirl

* [ ] [Release external modules][]
  * [ ] release [play-slick][]
  * [ ] release [scalatestplus-play][]
  * [ ] release [play-grpc][]

* [ ] [Release omnidoc][]

* [ ] [Update the supported modules page][]
* [ ] [Update playframework templates and seeds][]
* [ ] [Update Example Code Service][]

* [ ] [Update playframework.com][]
  * [ ] [Update `.version` in `play-generated-docs`][]
  * [ ] [Update `playReleases.json` and `changelog.md`][]
  * [ ] [Update versions for Example Code Service][]
  * [ ] [Deploy the website changes][]

* [ ] [Announce][]
  * [ ] Write a blog post on <https://playframework.ghost.io/ghost/24> about the release.
  * [ ] Write a topic on <https://discuss.lightbend.com/>
  * [ ] Write a release on <https://github.com/playframework/playframework/releases>
  * [ ] Send an internal email to eng-updates
  * [ ] Tweet about the new release.

* [ ] [Post release tasks][]

[Triage issues]: https://github.com/issues?utf8=%E2%9C%93&q=label%3Atriage+org%3Aplayframework+archived%3Afalse+
[Release projects that Play depends on]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-0---release-projects-that-play-depends-on-play-json-play-ws-twirl
[Release Play itself]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-1---release-play-itself
[Release external modules]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-2---release-external-modules
[Release omnidoc]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-3---release-omnidoc
[Update the supported modules page]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-4---update-playframework-templates-and-seeds
[Update playframework templates and seeds]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-4---update-playframework-templates-and-seeds
[Update Example Code Service]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-5---update-example-code-service
[Update playframework.com]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-6---update-playframeworkcom
[Update `.version` in `play-generated-docs`]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#update-version-in-play-generated-docs
[Update `playReleases.json` and `changelog.md`]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#update-playreleasesjson-and-changelogmd
[Update versions for Example Code Service]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#update-versions-for-example-code-service
[Deploy the website changes]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#deploy-the-website-changes
[Announce]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-7---announce
[Post release tasks]: https://github.com/playframework/play-meta/blob/master/releasing/play.md#step-8---post-release-tasks

[play-grpc]: https://github.com/playframework/play-grpc
[play-json]: https://github.com/playframework/play-json
[play-slick]: https://github.com/playframework/play-slick
[play-ws]: https://github.com/playframework/play-ws
[scalatestplus-play]: https://github.com/playframework/scalatestplus-play
[twirl]: https://github.com/playframework/twirl

[play-json/prs]:         https://github.com/playframework/play-json/pulls
[play-json/backport]:    https://github.com/playframework/play-json/labels/status%3Aneeds-backport
[play-json/forwardport]: https://github.com/playframework/play-json/labels/status%3Aneeds-forwardport
[play-json/milestones]:  https://github.com/playframework/play-json/milestones?direction=asc&sort=due_date

[play-ws/prs]:         https://github.com/playframework/play-ws/pulls
[play-ws/backport]:    https://github.com/playframework/play-ws/labels/status%3Aneeds-backport
[play-ws/forwardport]: https://github.com/playframework/play-ws/labels/status%3Aneeds-forwardport
[play-ws/milestones]:  https://github.com/playframework/play-ws/milestones?direction=asc&sort=due_date

[twirl/prs]:          https://github.com/playframework/twirl/pulls
[twirl/backport]:     https://github.com/playframework/twirl/labels/status%3Aneeds-backport
[twirl/forwardport]:  https://github.com/playframework/twirl/labels/status%3Aneeds-forwardport
[twirl/backport-1.3]: https://github.com/playframework/twirl/labels/status%3Aneeds-backport-1.3
[twirl/milestones]:   https://github.com/playframework/twirl/milestones?direction=asc&sort=due_date

[play/prs]:          https://github.com/playframework/playframework/pulls
[play/backport]:     https://github.com/playframework/playframework/labels/status%3Aneeds-backport
[play/forwardport]:  https://github.com/playframework/playframework/labels/status%3Aneeds-forwardport
[play/backport-2.6]: https://github.com/playframework/playframework/labels/status%3Aneeds-backport-2.6
[play/milestones]:   https://github.com/playframework/playframework/milestones?direction=asc&sort=due_date
