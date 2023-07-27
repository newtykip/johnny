<div align="center">
<img src="assets/developer.jpg" height="250">
<h3>johnny developer guide</h3>
thank you for showing interest in contributing (:<br/><br/>
</div>

## before we start

i am grateful that you are considering helping out, it really does mean a lot! johnny is a bot with a lot of moving components and due to the fact it is conditionally compiled extra care must be taken while implementing new features! in order to make the development lifecycle as beginner friendly as possible, we use a couple of tools while developing johnny. these are as follows:

- [just](https://github.com/casey/just) - a task runner
- [taplo](https://github.com/tamasfe/taplo) - a [toml](https://en.wikipedia.org/wiki/TOML) toolkit, formats all of the toml files
- [sea-orm-cli](https://github.com/SeaQL/sea-orm/tree/master/sea-orm-cli) - cli to generate files related to [sea-orm](https://github.com/SeaQL/sea-orm), the [ORM](https://en.wikipedia.org/wiki/Object%E2%80%93relational_mapping) we use
- [cargo-generate](https://github.com/cargo-generate/cargo-generate) - scaffolding tool for new packages
- [cargo-all-features](https://github.com/frewsxcv/cargo-all-features) - build/test all feature flag combinations
- [cargo-clean-recursive](https://crates.io/crates/cargo-clean-recursive) - clean target directories recursively

You can install all of the above using the following command:

```
cargo install just taplo-cli sea-orm-cli cargo-generate cargo-all-features
```

All workflows are defined in our [justfile](justfile) and each task can be run using `just <task>`. All available tasks are documented below.

- **clean** - cleans all target directories recursively
- **format** - formats all code using rustfmt, and toml files using taplo
- **new-package** - generates a new package using the [template](template)
- **new-migration \<name\>** - generates a new database migration
- **build-all** - build every feature combination

## our community

we adopt the [contributor covenant code of conduct](code_of_conduct.md), so please make all of our lives easier and follow it [:

as development ramps up, we may have a development hub on discord. watch this space.

## technical details

we use the stable toolchain. please make sure you are not using the nightly toolchain, it will cause us many headaches.

there is a special **dev** compiler feature that you will likely want to have enabled which we do not mention in the [consumer guide](consumer.readme.md). it adds some extra debugging tools to help you develop easier!

<sub>licensed with the <a href="license.md">opinionated queer license v1.1</a> - tl;dr see <a href="https://oql.avris.it/">here</a> :]</sub>
