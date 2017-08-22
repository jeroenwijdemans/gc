
### description
Tool that can quickly put a list of repositories on your computer.

Intention: easily and automatically reinstall repo;s after wiping a computer.

Reads a csv with 2 columns: location and git repo's. 
If the location cannot be found it will be cloned.

TODO: add possibility to check which repo's still need to be pushed


### example csv

```csv
location	repo
./tools/util-linux	git://git.kernel.org/pub/scm/utils/util-linux/util-linux.git
```