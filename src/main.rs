use lazy_static::*;

struct Terminal {
    tid: u8,
    name: String,
}
impl Terminal {
    fn show(&self, tab: usize, tln: usize, nln: usize) {
        println!("{:>width$} tid={tid}, tln={tln}, nln={nln}", self.name,
                    width=tab + self.name.len(),
                    tid=self.tid, tln=tln, nln=nln);
    }
}

type Terminals = Vec<Terminal>;
trait TerminalTrait {
    fn get_by_name(&self, name: &str) -> &Terminal;
}
impl TerminalTrait for Terminals {
    fn get_by_name(&self, name: &str) -> &Terminal {
        if let Some(trm) = self.iter().find(|&t| name.eq(&t.name)) {
            return trm
        } else {
            panic!("Terminals::get - invalid terminal name")
        }
    }
}

struct Function {
    fid: u8,
    name: String,
}
trait FunctionsTrait {
    fn get_by_name(&self, name: &str) -> &Function;
}
impl FunctionsTrait for Functions {
    fn get_by_name(&self, name: &str) -> &Function {
        if let Some(fnc) = self.iter().find(|&f| name.eq(&f.name)) {
            fnc
        } else {
            panic!("Functions::get - invalid function name")
        }
    }
}

type Functions = Vec<Function>;
lazy_static! {
    static ref FUNCTIONS: Functions = vec![
        Function {
            fid: 0,
            name: "Pat".to_string(),
        },
        Function {
            fid: 1, 
            name: "John".to_string(),
        },
        Function {
            fid: 2,
            name: "Anne".to_string(),
        },
        Function {
            fid: 3,
            name: "Chris".to_string(),
        },
        Function {
            fid: 4,
            name: "Stan".to_string(),
        },
    ];

    static ref TERMINALS: Terminals = vec![
        Terminal{
            tid: 0,
            name: "Julia".to_string(),
        },
        Terminal {
            tid: 1,
            name: "Helena".to_string(),
        },
        Terminal {
            tid: 2,
            name: "Peter".to_string(),
        },
        Terminal {
            tid: 3,
            name: "Mary".to_string(),
        },
        Terminal {
            tid: 4,
            name: "Eric".to_string(),
        },
    ];
}

struct FunctionNode {
    fnc: &'static Function,
    branch: Vec<Node>, 
}
impl<'a> FunctionNode {
    fn new(name: &'a str, branch: Vec<Node>) -> Node {
        FNode(
            FunctionNode {
                fnc: FUNCTIONS.get_by_name(name),
                branch
            })
    }
    fn show(&self, tab: usize, fln: usize, nln: usize) {
        println!("{:>width$} fid={fid}, fln={fln}, nln={nln}", self.fnc.name,
                    width=tab + self.fnc.name.len(),
                    fid=self.fnc.fid, fln=fln, nln={nln});
    }
    fn show_r(&self, tab: usize, fln: &mut usize, tln: &mut usize) {
        self.show(tab, *fln, *fln+*tln);
        *fln += 1;
        for c in self.branch.iter() {
            match c {
                FNode(fnc) => fnc.show_r(tab + 4, fln, tln),
                TNode(trm) => {
                    trm.show(tab + 4, *tln, *fln+*tln);
                    *tln += 1;
                }
            }
        }
    }
}

enum Node {
    TNode(&'static Terminal),
    FNode(FunctionNode),
}
use Node::*;
impl Node {
    fn show(&self) {
        match self {
            FNode(fnc) => {
                let mut fln: usize = 0;
                let mut tln: usize = 0;
                fnc.show_r(0, &mut fln, &mut tln);
            }
            TNode(trm) => println!("[{}]", trm.name),
        }
    }
}

struct TreeBranch {
    root: Node,
}

struct Tree {
    rb: TreeBranch,
}
impl Tree {
    fn show(&self, tag: &str) {
        println!("------------------------\nshow tree: {}", tag);
        self.rb.root.show();
    }
}

/*
fn perform_crossover(t1: &mut Tree, t2: &mut Tree) {
    let swap_target1 = t1.get_node(1);
    let swap_target2 = t2.get_node(2);
    
    mem::swap(swap_target1, swap_target2);
}
*/

fn main() {
    let t1 = Tree{
        rb: TreeBranch {
            root: FunctionNode::new("Pat", vec![
                    FunctionNode::new("Chris", vec![
                        TNode(TERMINALS.get_by_name("Julia"))])]),
        }};
    
    t1.show("t1");

    let t2 = Tree{
        rb: TreeBranch {
            root: FunctionNode::new("John", vec![
                    FunctionNode::new("Anne", vec![
                        TNode(TERMINALS.get_by_name("Peter")),
                        TNode(TERMINALS.get_by_name("Helena")),]),
                    FunctionNode::new("Stan", vec![
                        TNode(TERMINALS.get_by_name("Mary")),
                        TNode(TERMINALS.get_by_name("Eric")),]),]),
        }};
    
    t2.show("t2");

//    perform_crossover(t1, t2);
}
