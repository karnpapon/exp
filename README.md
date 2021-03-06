# `exp`
[![Build Status](https://travis-ci.com/karnpapon/exp.svg?branch=main)](https://travis-ci.com/karnpapon/exp) [![Build status](https://ci.appveyor.com/api/projects/status/thgy06kf78wooksu/branch/main?svg=true)](https://ci.appveyor.com/project/karnpapon/exp/branch/main)

<strong> CLI to create an opinionated temporary folder structure, automatically cleanup after they're expired. </strong>

<img src="./public/exp.gif"></img>

- `exp` will create temp folder `explore` and to soon-to-be-deleted folder `expire`. 
- files/folders live inside `explore` will be moved to `expire` 7 days after its last opened. 
- and then after next 7 days all files/folder will be deleted from `expire` folder. 
- The config file `.exp` will be located at `EXP_PATH` When you run `exp init` for the first time it will be created automatically. normally you don't have to edit this file. it is being used only for checking `EXP_PATH`.

## installation

```
## download binary first
$ curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git karnpapon/exp


## check whether exp is installed.
exp -h

```

## usage
- `cd to-your-target-path && exp init`
- put returned string (eg. `export EXP_PATH=/your/target/path`) into `.profile` or `.zprofile` (for [zsh](https://ohmyz.sh/) users) depends on what shell you're using, normally default would be `.profile`.
- add `exp` command to your `.profile` or `.zprofile`.
- everytime terminal is opened, `exp` will manage to check if any folder/file should be moved or deleted.

## command
- `exp init` = to create `explore` and `expire` folder and `.exp` (config file).
- `exp` = to check if file/folder is valid (ready to move/delete).

## todo
- [x] add cli for setting dir.
- [x] create config file.
- [x] push noti.
- [ ] cli for manually remove init file.
- [ ] details screenshot after folder has been deleted.
- [ ] expire date configurable.
- [x] cleaning up setup folder when EXP_PATH change.
- [ ] error handling.

## Optional
- [ ] config folder based on .gitignore

inspired by [cargo-temp](https://github.com/yozhgoor/cargo-temp)