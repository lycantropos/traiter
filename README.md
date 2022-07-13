traiter
=======

[![](https://github.com/lycantropos/traiter/workflows/CI/badge.svg)](https://github.com/lycantropos/traiter/actions/workflows/ci.yml "Github Actions")
[![](https://codecov.io/gh/lycantropos/traiter/branch/master/graph/badge.svg)](https://codecov.io/gh/lycantropos/traiter "Codecov")
[![](https://docs.rs/traiter/badge.svg)](https://docs.rs/traiter/ "docs.rs")
[![](https://img.shields.io/github/license/lycantropos/traiter.svg)](https://github.com/lycantropos/traiter/blob/master/LICENSE "License")
[![](https://img.shields.io/crates/v/traiter.svg)](https://crates.io/crates/traiter "crates.io")

Development
-----------

### Bumping version

#### Preparation

Install
[bump2version](https://github.com/c4urself/bump2version#installation).

#### Pre-release

Choose which version number category to bump following [semver
specification](http://semver.org/).

Test bumping version
```bash
bump2version --dry-run --verbose $CATEGORY
```

where `$CATEGORY` is the target version number category name, possible
values are `patch`/`minor`/`major`.

Bump version
```bash
bump2version --verbose $CATEGORY
```

This will set version to `major.minor.patch-alpha`. 

#### Release

Test bumping version
```bash
bump2version --dry-run --verbose release
```

Bump version
```bash
bump2version --verbose release
```

This will set version to `major.minor.patch`.

### Running tests

Plain
```bash
cargo test
```

Inside `Docker` container:
```bash
docker-compose up
```

`Bash` script:
```bash
./run-tests.sh
```

`PowerShell` script:
```powershell
.\run-tests.ps1
```
