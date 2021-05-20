# `exp`
[![Build Status](https://travis-ci.com/karnpapon/exp.svg?branch=main)](https://travis-ci.com/karnpapon/exp)

<strong> CLI to create an opinionated temporary folder structure, automatically cleanup after they're expired. </strong>

- `exp` will create temp folder `explore` and to soon-to-be-deleted folder `expired`. 
- files/folders live inside `explore` will be moved to `expired` 7 days after its last opened. 
- and then after next 7 days all files/folder will be deleted from `expired` folder. 

## install
- `cd to-your-target-path && exp init`
- put returned string (eg. `export EXP_PATH=/your/target/path`) after setup init-path to `.bash_profile` or `.bashrc` 
- add `exp` command to your `.bashrc` or `.bash_profile`.
- `source .bash_profile` or `source .bashrc` to reload path.

## todo
- [x] add cli for setting dir.
- [x] create config file.
- [x] push noti.
- [ ] details screenshot after folder has been deleted.
- [ ] expire date configurable.
- [x] cleaning up setup folder when EXP_PATH change.
- [ ] error handling.

## Optional
- [ ] config folder based on .gitignore