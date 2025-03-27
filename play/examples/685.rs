// 在本问题中，有根树指满足以下条件的 有向 图。该树只有一个根节点，所有其他节点都是该根节点的后继。该树除了根节点之外的每一个节点都有且只有一个父节点，而根节点没有父节点。

// 输入一个有向图，该图由一个有着 n 个节点（节点值不重复，从 1 到 n）的树及一条附加的有向边构成。附加的边包含在 1 到 n 中的两个不同顶点间，这条附加的边不属于树中已存在的边。

// 结果图是一个以边组成的二维数组 edges 。 每个元素是一对 [ui, vi]，用以表示 有向 图中连接顶点 ui 和顶点 vi 的边，其中 ui 是 vi 的一个父节点。

// 返回一条能删除的边，使得剩下的图是有 n 个节点的有根树。若有多个答案，返回最后出现在给定二维数组的答案。

struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        let parent = (0..=n).collect();
        Self { parent }
    }

    fn merge(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        self.parent[root_x as usize] = root_y;
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] != x {
            self.parent[x as usize] = self.find(self.parent[x as usize]);
        }
        self.parent[x as usize]
    }
}

pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut parent: Vec<_> = (0..=edges.len()).collect();
    let mut uf = UnionFind::new(edges.len() as i32);

    let mut conflict = -1_i32;
    let mut cycle = -1_i32;
    for i in 0..edges.len() {
        let edge = &edges[i];
        let u = edge[0];
        let v = edge[1];
        if parent[v as usize] != v as usize {
            conflict = i as i32;
        } else {
            parent[v as usize] = u as usize;
            if uf.find(u) == uf.find(v) {
                cycle = i as i32;
            } else {
                uf.merge(u, v);
            }
        }
    }

    if conflict == -1 {
        vec![edges[cycle as usize][0], edges[cycle as usize][1]]
    } else if cycle == -1 {
        vec![edges[conflict as usize][0], edges[conflict as usize][1]]
    } else {
        let x = edges[conflict as usize][1] ;
        vec![parent[x as usize] as i32, edges[conflict as usize][1]]
    }
}

fn main() {}
