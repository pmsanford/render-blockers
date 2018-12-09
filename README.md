# render-blockers
Renders clusters of Jira blockers (in the currently open sprints) using graphviz.

First, build the graphviz definition:
```bash
cargo run me@work.com SoMEkIndOfapiKeY123 https://work.atlassian.net
```

For the next step, you'll need to [get graphviz](https://graphviz.gitlab.io/download/). Run:
```bash
dot -Tpng -oblockers.png blockers.dot
```
or just run `render.sh` from the repository. Then gaze at the generated `blockers.png` balefully.

Nodes are colored by the following rules:
* Closed: green
* Review: goldenrod
* In Progress: blue
* Cancelled: firebrick (dark red)
* All Others: no color
