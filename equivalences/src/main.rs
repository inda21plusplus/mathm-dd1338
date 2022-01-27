use std::{
    collections::HashSet,
    fmt::Write,
    io::{stdin, Read},
    usize,
};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input
        .trim()
        .split(|c| c == ' ' || c == '\n')
        .map(|s| s.parse::<usize>().unwrap());
    let mut output = String::new();

    for _case in 0..ints.next().unwrap() {
        let (n, m) = (ints.next().unwrap(), ints.next().unwrap());
        const N: usize = 20000;
        const _M: usize = 50000;

        let mut adjf = vec![vec![]; N];
        let mut adjr = vec![vec![]; N];
        for _ in 0..m {
            let (a, b) = (ints.next().unwrap() - 1, ints.next().unwrap() - 1);
            adjf[a].push(b);
            adjr[b].push(a);
        }

        // https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
        let mut visited = [false; N];
        let mut l = vec![];
        fn visit(u: usize, visited: &mut [bool], adjf: &[Vec<usize>], l: &mut Vec<usize>) {
            if visited[u] {
                return;
            }
            visited[u] = true;
            for &v in &adjf[u] {
                visit(v, visited, adjf, l);
            }
            l.push(u);
        }
        for u in 0..n {
            visit(u, &mut visited, &adjf, &mut l);
        }
        let mut visited = [false; N];
        let mut root = [usize::MAX; N];
        let mut roots = HashSet::new();
        let mut stack = l.into_iter().map(|u| (u, u)).collect::<Vec<_>>();
        while let Some((r, u)) = stack.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            root[u] = r;
            roots.insert(r);
            for &v in &adjr[u] {
                stack.push((r, v));
            }
        }

        // https://en.wikipedia.org/wiki/Strong_connectivity_augmentation#Unweighted_version

        let mut has_outgoing = HashSet::new();
        let mut has_ingoing = HashSet::new();

        let mut visited = [false; N];
        for &r in &roots {
            let mut stack = vec![r];
            'dfs: while let Some(u) = stack.pop() {
                if visited[u] {
                    continue;
                }
                visited[u] = true;
                for &v in &adjf[u] {
                    if root[v] != r {
                        has_outgoing.insert(r);
                        break 'dfs;
                    }
                    stack.push(v);
                }
            }
        }
        let mut visited = [false; N];
        for &r in &roots {
            let mut stack = vec![r];
            'dfs2: while let Some(u) = stack.pop() {
                if visited[u] {
                    continue;
                }
                visited[u] = true;
                for &v in &adjr[u] {
                    if root[v] != r {
                        has_ingoing.insert(r);
                        break 'dfs2;
                    }
                    stack.push(v);
                }
            }
        }

        let sources = has_outgoing.difference(&has_ingoing).count();
        let sinks = has_ingoing.difference(&has_outgoing).count();
        let isolated = roots
            .iter()
            .filter(|&u| !has_ingoing.contains(u) && !has_outgoing.contains(u))
            .count();

        writeln!(output, "{}", (sources + isolated).max(sinks + isolated)).unwrap();
    }

    print!("{}", output);
}
