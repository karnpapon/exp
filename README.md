# `exp`

<strong> CLI to create an opinionated temporary folder structure, automatically cleanup after they're expired. </strong>

`exp` will create `temp_queues` folder and `delete_queues` folder automatically by setup target path.
any files/folder live inside `temp_queues` will be moved to `delete_queues` after it has been created 7 days ( slightly similar to how email services work) and then after next 7 days all files/folder will be deleted from `delete_queues` folder. 

## install
- `exp init <YOUR-TARGET-PATH>`
- put returned string (eg. `export EXP_PATH=/your/target/path`) after setup init-path to `.bash_profile` or `.bashrc` 
- `source .bash_profile` or `source .bashrc` to reload path.

## todo
- [x] add cli for setting dir.
- [x] create config file.
- [x] push noti.
- [ ] details screenshot after folder has been deleted.
- [ ] expire date configurable.
- [ ] cleaning up setup folder when EXP_PATH change.

## Optional
- [ ] config folder based on .gitignore