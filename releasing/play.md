# Play Release Procedure

- [Play Release Procedure](#play-release-procedure)
  - [Before the release](#before-the-release)
    - [Internal communication](#internal-communication)
    - [Issues and pull request triage](#issues-and-pull-request-triage)
  - [Release tracking issue](#release-tracking-issue)
  - [Intro](#intro)
  - [Prerequisites](#prerequisites)
  - [If something goes wrong](#if-something-goes-wrong)
  - [Releasing Play](#releasing-play)
    - [Step 0 - release projects that Play depends on (play-json, play-ws, Twirl)](#step-0---release-projects-that-play-depends-on-play-json-play-ws-twirl)
    - [Step 1 - release Play itself](#step-1---release-play-itself)
    - [Step 2 - release external modules](#step-2---release-external-modules)
    - [Step 3 - release omnidoc](#step-3---release-omnidoc)
    - [Step 4 - update playframework templates and seeds](#step-4---update-playframework-templates-and-seeds)
      - [Docs](#docs)
      - [Other](#other)
    - [Step 5 - Update Example Code Service](#step-5---update-example-code-service)
    - [Step 6 - Update playframework.com](#step-6---update-playframeworkcom)
    - [Step 7 - Announce](#step-7---announce)
    - [Step 8 - Post release tasks](#step-8---post-release-tasks)

## Before the release

### Internal communication

Make sure that other teams inside Lightbend are aware of the upcoming release, even if it is a minor/patch one. For example:

1. [Lightbend Telemetry](https://developer.lightbend.com/docs/telemetry/current/home.html) Team
2. [Lightbend Platform](https://www.lightbend.com/lightbend-platform) Team
3. Akka Team

### Issues and pull request triage

See if there are [issues that need triage](https://github.com/issues?utf8=%E2%9C%93&q=label%3Atriage+org%3Aplayframework+archived%3Afalse+) and are possibly related to the upcoming release. This is mainly important if you are doing a minor or major release.

## Release tracking issue

Create a new [release tracking issue][].

[release tracking issue]: https://github.com/playframework/play-meta/issues/new?template=z_play-release.md

## Intro

Play 2.4 introduced a much more modular setup for the Play modules and documentation, where modules now span
multiple projects.  Each project needs to be released individually, and does not necessarily need to be
re-released every time Play itself is released.  When cutting a release of Play, you need to make the following
decisions:

- Does this release of Play require releasing the modules that depend on it, eg play-grpc and play-slick.  For
  a typical minor release, it would not.  For a major release, and often for pre releases of major releases, the
  answer is yes.

- Does this release of Play require updating the activator templates?  If it is the latest stable release that
  you are releasing, then yes.  Specifically, if the current stable release is 2.3.8, and you're releasing
  2.3.9, then you need to publish the templates, but if you're releasing 2.2.7, then you must not publish the
  templates, since doing so will revert the Play seed templates back to 2.2 when they should be on 2.3.

## Prerequisites

You need an account on `vegemite` with sudo access to the `play` account.  Generally, all actions
should be performed by ssh-ing into `vegemite`, starting a `screen` session, then switching to the `play` user.  By
starting a `screen` session, you ensure that your internet connection does not impact the release.

For tagging some releases (e.g. play-ws) you may need to enter your Github username and password. If you have
2FA enabled for Github (which you should have!) then you may need to generate a [personal access token][]..

[personal access token]: https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/

## If something goes wrong

The release process pushes all artifacts to maven central **and** bintray, but doesn't "promote" them until all
publishing is complete and successful.

If the build failed during or after the promotion of either bintray or maven central artifacts, there is no
going back.  Published artifacts are immutable, they find their way into CDNs, into caches on developer
machines, and if there are two different versions of the same artifact out there, this can cause big problems.
Your only option is to fix the problem, and attempt another release of the next version.  Remember, version
numbers are cheap.

If the build failed during or before the publishing of artifacts, but not after either the bintray or maven
central promotion, you can drop the maven central staging repository and delete bintray version.  This can
either be done through their corresponding web interfaces, or by using the `sonatypeDrop` and `bintrayUnpublish` sbt
commands.

## Releasing Play

### Step 0 - release projects that Play depends on (play-json, play-ws, Twirl)

Prepare the branch for each project:

- Look for PRs that should be merged.
- Look at `status:needs-backport` issues/PRs (including closed ones).
- Look at issues/PRs tagged milestone version (including closed ones).
- Update any dependencies that are needed.
- Update any upstream projects (e.g make sure new play-ws is using new play-json)

May need to release these projects: play-json, play-ws, twirl

When ready:

```bash
cd deploy
./release --project <project> --branch <branch>
```

> Note: `play-ws` is already using [sbt-dynver](https://github.com/dwijnand/sbt-dynver), so when realising it you need to pass `tag` parameter, for example: `./release --project <project> --branch <branch> --tag vX.Y.Z`. Version must be prefixed by `v`.

### Step 1 - release Play itself

Prepare the branch:

- Look for PRs that should be merged.
- Look at [`status:needs-backport`](https://github.com/playframework/playframework/issues?utf8=%E2%9C%93&q=label%3Astatus%3Aneeds-backport+) issues/PRs (including closed ones). If you are releasing an older version of Play, look at the `status:needs-backport-x.x` label too.
- Look at issues/PRs tagged milestone version (including closed ones).
- Updated any dependencies that are needed (e.g. Dependencies.scala)

When ready:

```bash
cd deploy
./release --branch <branch>
```

Where branch is the branch of Play that you want to release, eg 2.7.x.  You will be prompted for the version
number and the next version number.  For minor releases, the default should usually be appropriate.

Once Play is released, you need to wait 10 minutes or so for a Maven central sync before you can perform any of
the remaining tasks.

**Verification**: You can check that the artifacts are available at Maven Central under play_\<scalaversion>.
<https://repo1.maven.org/maven2/com/typesafe/play/>

**Warning**: the play release will create a tag on the repository and push it to GitHub. That will trigger one
or two travis jobs that are granted to fail. The reason of the failure is a circular dependency with Omnidoc.
Play vX.Y.Z uses Omnidoc vX.Y.Z so those two travis jobs triggered by the tag and the changes in version.sbt
will be broken until you release Omnidoc few steps down this document.

**Warning**: If you are releasing a major version of Play which requires a new branch, for example, 2.7.0, you also need to configure MiMa before merging new features at this new branch. Ideally, do this as part of the release process.

### Step 2 - release external modules

This includes the modules:

- play-slick
- scalatestplus-play
- play-grpc

Only release these if they need to be
released, generally for minor Play releases, there's no reason to cut a new release of these, these libraries
are free to have their own release cycle.

**Note**: since we update omnidoc and the Play templates and seeds people reading the docs or starting a new
project will automatically see and use the latest minor versions of all modules, even if we don't patch all
modules directly to update dependencies.

You may need to bump the Play version in the external module, do this, commit, and depending on how major the
version bump is, push directly to the repo or go through a pull request.  Once that is done, to release:

- play-slick
- scalatestplus-play

Run the `release` script on vegemite:

```bash
cd deploy
./release --project <project> --branch <branch>
```

Where project is, for example, scalatestplus-play, play-slick,...

For play-grpc see its [Releasing](https://github.com/playframework/play-grpc/blob/master/RELEASING.md)
procedure.

Again, you will need to wait 10 minutes or so for a Maven central sync before you can perform any of the
remaining tasks. In the meantime you can see the staged artifacts here on OSS Sonatype. E.g. here is a search
for Play 2.6.3 artifacts:
<https://oss.sonatype.org/#nexus-search;gav~com.typesafe.play~~2.7.0~~>

**Verification**: You can check that the artifacts are available at Maven Central under
play-slick_\<scalaversion\>, etc.

- <https://repo1.maven.org/maven2/com/typesafe/play/play-slick_2.13/>
- <https://repo1.maven.org/maven2/com/lightbend/play/play-grpc-testkit_2.12/>
- <https://repo1.maven.org/maven2/org/scalatestplus/play/scalatestplus-play_2.13/>

**Verification**: when you run sbt new playframework/play-{scala,java}-seed.g8 it should pick up the new version
on Maven. Try the templates out. You may need to update them (possibly with templatecontrol?) if they don't work
with the new version of Play.

### Step 3 - release omnidoc

**Warning**: this is a compulsory step and the version vX.Y.Z of omnidoc released here must match the version
vX.Y.Z of Play released in step 1 above. There's a circular dependency.

Omnidoc builds Play's documentation from all the current versions of Play and its modules.
In the omnidoc build file for the branch of Play that you are releasing:

1 Update the Play version to the version of Play that you just released, and also
2. Update any external modules to their latest version that is compatible with that version of Play.

Here's an example update to the omnidoc 2.4.x branch.

```diff
$ git diff
diff --git a/project/OmnidocBuild.scala b/project/OmnidocBuild.scala
-  val playVersion       = sys.props.getOrElse("play.version",       "2.4.3")
+  val playVersion       = sys.props.getOrElse("play.version",       "2.4.4")
```

These changes can generally be pushed directly to GitHub.

To release omnidoc:

```bash
cd deploy/omnidoc
# make sure you're on the right branch
git checkout master
# make sure you have all upstream changes
git pull --ff-only
sbt 'release cross'
```

> Note: omnidoc does not have a `version.sbt` file and also does not use sbt-dynver. It gets its version from Play, so you must release using `sbt 'release cross'`.

**Verification**: check that the artifacts are available at Maven Central under play-omnidoc_<scalaversion>. It
may take a few minutes. <https://repo1.maven.org/maven2/com/typesafe/play/>

Once that is done, you can update the docs on playframework.com, by running: (use the branch to push to in
<https://github.com/playframework/play-generated-docs> and the tag id you want to create)

```bash
/home/play/deploy/omnidocDeploy --branch <branch> --tag <tag> /home/play/deploy/play-generated-docs
```

Verification: check there is a new tag `<tag>` at <https://github.com/playframework/play-generated-docs> project. It
should be on top of <https://github.com/playframework/play-generated-docs/releases>. The website should pick this
tagged version of the generated docs up to 10 minutes later. You can check that then using the following URL
pattern: `https://www.playframework.com/documentation/<tag>/Home`. For example
<https://www.playframework.com/documentation/2.7.2/Home>.

### Step 4 - update playframework templates and seeds

#### Docs

- Lightbend Platform ["Library build dependencies"](https://developer.lightbend.com/docs/lightbend-platform/introduction/getting-help/build-dependencies.html) page (sbt plugin & artifacts):
  - <https://github.com/lightbend/lightbend-platform-docs/blob/master/docs/modules/getting-help/examples/build.sbt>
  - <https://github.com/lightbend/lightbend-platform-docs/blob/master/docs/modules/getting-help/pages/build-dependencies.adoc>

#### Other

The "core" playframework templates describe Play's overall feature set -- they are maintained by the Play team
and in the <https://github.com/playframework/> GitHub organisation.  Historically, there were issues with out of date Activator
templates, and confusion between what was supported and current, and what was third-party and out of date.

The sample templates are run through <https://github.com/playframework/templatecontrol> -- this contains a list of
the example projects and seeds.

Running TemplateControl will look for version numbers and automatically create a pull request with any needed
upgrades.  It's safe to run, but you will need local forks of all the template projects: see
<https://github.com/lightbend/templatecontrol/blob/master/scripts/README.md>.

Create a PR from the version you want so that your changes are captured. For example:

```bash
git checkout -b upgrade-2.7.2
vim src/main/resources/<play-branch-name>.conf
```

After updating the `.conf` files, commit and push your changes, submit a new pull request to templatecontrol
project, merge it and then run templatecontrol:

Note the prerequisites detailed in <https://github.com/playframework/templatecontrol#prerequisites>.

```bash
sbt run
```

**Verification**: Check out a local copy of play-scala-starter-example and play-java-starter-example locally and
smoke test them locally:

1. $ git clone https://github.com/<yourname>/play-samples.git
2. $ cd play-samples
3. $ git fetch origin templatecontrol-2.7.x
4. $ git checkout templatecontrol-2.7.x
5. $ git diff HEAD^
6. $ cd play-scala-starter-example
7. $ sbt
8. \> show dependencyClasspath
9. \> test
10. \> run

Once you're done, commit your changes and merge the updated version into the project:

```bash
git commit -am "upgrade section to play 2.7.2"
git push origin upgrade-2.7.2
hub pull-request --browse
```

So that everything is up to date.

**Verification**: The sample repository builds can be seen at <https://travis-ci.com/playframework/play-samples>. Make sure the build is green and then merge the pull request (it is possible Mergify will do that automatically).

There is an integration through webhook to example-code-service: see
<https://github.com/lightbend/example-code-service/> for what is on their end for packaging `./sbt` in a zip file.

### Step 5 - Update Example Code Service

The Example Code Service will need to be updated to point to the new versions.  All the play templates are in
play-templates.conf:

<https://github.com/lightbend/example-code-service/blob/master/example-code-service/conf/play-templates.conf>

You will need to have unique names for each template, so rename the current templates so it follows the
templates with the version explicitly appended (see bold bits):

i.e. when you come out with 2.7.0, you rename the current one to

```conf
{
  display-name = "Play Java using Dagger 2 for Compile Time DI (2.6.x)"
  name = "play-java-dagger2-example-2.6.x"
  url = "git@github.com:playframework/play-java-dagger2-example.git"
  github-repo = "playframework/play-java-dagger2-example"
  branch = "2.6.x"
  keywords = [ "play", "java", "di", "dagger2", "2.6.x"
  summary = "Play Application using Dagger 2 for Compile Time DI"
},
```

Create a pull request with all the updates, test it, run some examples with "sbt run", then submit that to
example-code-service and push.  The deploy should be automatic, but you may want to check with "tooling team"
that you did it right and it is updated.

**Verification**: visit <https://developer.lightbend.com/start/?group=play> and download a template and check that
it's up-to-date and works properly.

### Step 6 - Update playframework.com

If you are releasing a MAJOR version, review the contents of [`.version` in `play-generated-docs`](https://github.com/playframework/play-generated-docs/blob/master/.version) before  updating the site.

Update `playReleases.json` and `changelog.md` in [playframework.com website git repository](https://github.com/playframework/playframework.com/).

Note that the changelog should be updated describing all milestone/release candidates for the same release
collectively. In addition the `playReleases.json` should be updated with the development releases in place i.e.
you don't need to publish information on both an M1 and an M2 release.  If the release is for the latest stable
or development version, upgrade the website itself to use that version.

You will also want to check that the downloads page has the right branch tag (i.e. 2.7.x) for the example code
service branches that you just updated.  This is defined in playframework/playframework.com, under the
application.conf setting:

```conf
examples.playVersions = [ "2.7.x", "2.6.x" ]
```

**Verification**: Run playframework.com locally on your machine and check the /download and /changelog pages.

Check that you have the right version numbers and that kickstartr / example-code-service is up to date.

Commit and push your changes.

**NOTE**: you will need a distinct SSH public key for this.  Talk to James or Ed if you don't have access.

To set up your public key:

1. Ask someone else from Play team to add your pub key to the website instance.
2. Edit ~/.ssh/config and add the following:

```
 Host www.playframework.com
  User ubuntu
  Hostname 54.173.126.144
```

3. You can now log in: `ssh ubuntu@www.playframework.com`

ssh into `ubuntu@www.playframework.com`, and run `./deploy.sh`

**Verification**: Check that <https://www.playframework.com/download#alternatives> contains the new release.

### Step 7 - Announce

* If it is a major or minor release, write a blog post on <https://playframework.ghost.io/ghost/24/> about the release (not necessary for patch releases).
* Write a topic on <https://discuss.lightbend.com/>
* Write a release on <https://github.com/playframework/playframework/releases>
* Send an internal email to eng-updates
* Tweet about the new release.

**Tip**: To get a list of authors:

```bash
git fetch --tags && git shortlog -s 2.7.1..2.7.2 | cut -c8- | sort
```

**Verification**: Go to <https://discuss.lightbend.com/> and <https://twitter.com/playframework> look for the message and tweet.

### Step 8 - Post release tasks

1. Close the milestone for the release (for example 2.7.1)
2. Create a new milestone for the next release (for example 2.7.2)
3. Move issues and pull requests from the old milestone to the new one if necessary
