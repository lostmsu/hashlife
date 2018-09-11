use std::rc::Rc;

struct QuadTreeInnerNode<T, InnerData> {
    top_left: Rc<QuadTreeNode<T, InnerData>>,
    top_right: Rc<QuadTreeNode<T, InnerData>>,
    bottom_left: Rc<QuadTreeNode<T, InnerData>>,
    bottom_right: Rc<QuadTreeNode<T, InnerData>>,
    data: InnerData,
}

enum QuadTreeNode<T, InnerData> {
    Leaf(T),
    Inner(QuadTreeInnerNode<T, InnerData>),
}

type CellState = bool;

struct LifeInnerData {
    level: u8,
    alive: bool,
}
type LifeNode = QuadTreeNode<CellState, LifeInnerData>;
type LifeInnerNode = QuadTreeInnerNode<CellState, LifeInnerData>;

const dead: Rc<LifeNode> = Rc::new(QuadTreeNode::Leaf(false));
const alive: Rc<LifeNode> = Rc::new(QuadTreeNode::Leaf(true));


impl LifeInnerNode {
    fn getLevel(&self) -> u8 { self.data.level }
    fn isAlive(&self) -> bool { self.data.alive }
}

impl LifeNode {
    fn getLevel(&self) -> u8 {
        match self {
            QuadTreeNode::Leaf(_) => 0,
            QuadTreeNode::Inner(inner) => inner.getLevel(),
        }
    }

    fn isAlive(&self) -> bool {
        match self {
            QuadTreeNode::Leaf(isAlive) => *isAlive,
            QuadTreeNode::Inner(inner) => inner.isAlive(),
        }
    }
}

fn make_next_level(top_left: Rc<LifeNode>, top_right: Rc<LifeNode>,
                   bottom_left: Rc<LifeNode>, bottom_right: Rc<LifeNode>) -> LifeInnerNode {
    if top_left.getLevel() != top_right.getLevel()
     || top_left.getLevel() != bottom_left.getLevel()
     || top_left.getLevel() != bottom_right.getLevel() {
         panic!("invalid levels");
    }

    return QuadTreeInnerNode{
        data: LifeInnerData {
            level: top_left.getLevel() + 1,
            alive: top_left.isAlive() && top_right.isAlive()
                && bottom_left.isAlive() && bottom_right.isAlive() },
        top_left: top_left,
        top_right: top_right,
        bottom_left: bottom_left,
        bottom_right: bottom_right,
    }
}

impl LifeInnerNode {
    fn nextGeneration(&self) -> Rc<LifeNode> {
        // if the whole area is dead, next step the inside will still be dead
        if !self.isAlive() {
            return self.top_left;
        }

        if self.getLevel() == 2 {
            
        } else {

        }
    }
}

fn main() {
    println!("Hello, world!");
}
