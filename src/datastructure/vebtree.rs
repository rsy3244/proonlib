use std::mem;

pub struct VEBTree {
    magnitude: usize,
    size: usize,
    node: VEBNode,
}

enum VEBNode {
    InnerNode(InnerNode),
    Leaf(Leaf),
}

impl VEBNode {
    fn new(magnitude: usize, elem: usize) -> Self {
        match magnitude {
            0 => VEBNode::Leaf(Leaf::new(elem)),
            _ => VEBNode::InnerNode(InnerNode::new(magnitude, elem)),
        }
    }

    fn empty(&self) -> bool {
        match self {
            InnerNode(n) => n.empty(),
            Leaf(n) => n.empty(),
        }
    }

    fn min(&self) -> bool {
        match self {
            InnerNode(n) => n.min(),
            Leaf(n) => n.min(),
        }
    }

    fn max(&self) -> bool {
        match self {
            InnerNode(n) => n.max(),
            Leaf(n) => n.max(),
        }
    }

    fn member(&self, x: usize) -> usize {
        match self {
            InnerNode(n) => n.member(x),
            Leaf(n) => n.member(x),
        }
    }

    fn successor(&self, x: usize) -> usize {
        match self {
            InnerNode(n) => n.successor(x),
            Leaf(n) => n.successor(x),
        }
    }

    fn predecessor(&self, x: usize) -> usize {
        match self {
            InnerNode(n) => n.predecessor(x),
            Leaf(n) => n.predecessor(x),
        }
    }
}

struct InnerNode {
    magnitude: usize,
    min: usize,
    max: usize,
    cluster: Vec<VEBNode>,
    summary: Option<VEBNode>,
}

impl InnerNode {
    fn new(magnitude: usize, elem: usize) -> Self {
        Self {
            magnitude,
            min: elem,
            max: elem,
            cluster: Vec::new(),
            summary: None,
        }
    }

    fn high(&self, x: usize) -> usize {
        x >> (self.magnitude / 2)
    }

    fn low(&self, x: usize) -> usize {
        x & (2usize.pow(self.magnitude / 2) - 1)
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x << (self.magnitude / 2) + y
    }

    fn empty(&self) -> bool {
        self.max < self.min
    }

    fn min(&self) -> usize {
        self.min
    }

    fn max(&self) -> usize {
        self.max
    }

    fn member(&self, x: usize) -> bool {
        if self.empty() {
            return false;
        }
        if x == self.min() || x == self.max() {
            return true;
        }
        if let Some(s) = self.summary {
            self.summary.member(high(x)) && self.cluster[high(x)].member(low(x))
        } else {
            false
        }
    }

    fn successor(&self, x: usize) -> Option<usize> {
        if self.empty() {
            return None;
        }
        if x < self.min() {
            return Some(self.min());
        }
        if let Some(s) = self.summary {
            if self.summary.member(high(x)) && self.low(x) < self.cluster[self.high(x)].max() {
                let offset = self.cluster[high(x)].successor(self.low(x));
                Some(self.index(high(x), offset))
            } else {
                if let Some(suc_c) = self.summary.successor(self.high(x)) {
                    let offset = self.cluster[suc_c].min();
                    Some(self.index(suc_c, offset))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn predecessor(&self, x: usize) -> Option<usize> {
        if self.empty() {
            return None;
        }
        if x > self.max() {
            return Some(self.max());
        }
        if let Some(&s) = self.summary {
            if self.summary.member(high(x)) && self.cluster.min() < self.low(x) {
                let offset = self.cluster[high(x)].predecessor(self.low(x));
                Some(self.index(high(x), offset))
            } else {
                if let Some(suc_c) = self.summary.predecessor(self.high(x)) {
                    let offset = self.cluster[suc_c].max();
                    Some(self.index(suc_c, offset))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    /*
    fn insert(&mut self, x: usize) {
        if self.empty() {
            self.min = x;
            self.max = x;
            return;
        }
        let (x, min) = if x < self.min() {
            (self.min(), x)
        } else {
            (x, self.min())
        };
        self.min = min;
        if let Some(s) = self.summary
    }
    */
}

struct Leaf {
    min: usize,
    max: usize,
}

impl Leaf {
    fn new(elem: usize) -> Self {
        Self {
            min: elem,
            max: elem,
        }
    }

    fn empty(&self) -> bool {
        self.max < self.min
    }

    fn min(&self) -> usize {
        self.min
    }

    fn max(&self) -> usize {
        self.max
    }

    fn member(&self, x: usize) -> bool {
        if self.empty() {
            return false;
        }
        if x == self.min() || x == self.max() {
            return true;
        }
        return false;
    }

    fn successor(&self, x: usize) -> Option<usize> {
        if self.empty() {
            return None;
        }
        if x < self.min() {
            return Some(self.min());
        }
    }

    fn predecessor(&self, x: usize) -> Option<usize> {
        if self.empty() {
            return None;
        }
        if x > self.max() {
            return Some(self.max());
        }
        if let Some(&s) = self.summary {
            if self.summary.member(high(x)) && self.cluster.min() < self.low(x) {
                let offset = self.cluster[high(x)].predecessor(self.low(x));
                Some(self.index(high(x), offset))
            } else {
                if let Some(suc_c) = self.summary.predecessor(self.high(x)) {
                    let offset = self.cluster[suc_c].max();
                    Some(self.index(suc_c, offset))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    /*
    fn insert(&mut self, x: usize) {
        if self.empty() {
            self.min = x;
            self.max = x;
            return;
        }
        let (x, min) = if x < self.min() {
            (self.min(), x)
        } else {
            (x, self.min())
        };
        self.min = min;
        if let Some(s) = self.summary
    }
    */
}
