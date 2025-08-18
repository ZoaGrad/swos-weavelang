use anyhow::Result;
use egg::{rewrite as rw, *};
use serde::{Serialize, Deserialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub type Lang = egg::SymbolLang;
pub type Rec = egg::RecExpr<Lang>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcheVec(pub Vec<f32>);
impl AcheVec {
    pub fn max_merge(a: &Self, b: &Self) -> Self {
        let n = a.0.len().max(b.0.len());
        let mut out = vec![0.0; n];
        for i in 0..n {
            let ai = *a.0.get(i).unwrap_or(&0.0);
            let bi = *b.0.get(i).unwrap_or(&0.0);
            out[i] = ai.max(bi);
        }
        AcheVec(out)
    }
    pub fn sum(&self) -> f32 { self.0.iter().copied().sum() }
}
#[derive(Debug, Clone, Default)]
pub struct AcheAnalysis;
#[derive(Debug, Clone, Default)]
pub struct AcheData { pub ache: AcheVec }

impl Analysis<Lang> for AcheAnalysis {
    type Data = AcheData;
    fn make(_egraph: &EGraph<Lang, Self>, e: &Lang) -> Self::Data {
        let op = e.to_string();
        let local = match op.as_str() {
            "compose" | "and" => AcheVec(vec![0.7, 0.7]),
            "filter"          => AcheVec(vec![0.5, 0.2]),
            "map"             => AcheVec(vec![0.2, 0.4]),
            "normalize"       => AcheVec(vec![0.1, 0.1]),
            "glyph"           => AcheVec(vec![0.3, 0.3]),
            "seq"             => AcheVec(vec![0.05, 0.05]),
            _                 => AcheVec(vec![0.0, 0.0]),
        };
        AcheData { ache: local }
    }
    fn merge(&mut self, a: &mut Self::Data, b: Self::Data) -> DidMerge {
        let before = a.ache.0.clone();
        a.ache = AcheVec::max_merge(&a.ache, &b.ache);
        DidMerge(before != a.ache.0, false)
    }
    fn modify(_egraph: &mut EGraph<Lang, Self>, _id: Id) {}
}

pub fn default_rewrites() -> Vec<Rewrite<Lang, AcheAnalysis>> {
    vec![
        rw!("map-fusion"; "(map ?f (map ?g ?x))" => "(map (compose ?f ?g) ?x)"),
        rw!("filter-fusion"; "(filter ?p (filter ?q ?x))" => "(filter (and ?p ?q) ?x)"),
        rw!("filter-push-map"; "(filter (lift ?p) (map ?f ?x))" => "(map ?f (filter ?p ?x))"),
        rw!("normalize-idem"; "(normalize (normalize ?x))" => "(normalize ?x)"),
        // (optional) a few algebraic identities if inputs contain +/*
        rw!("add-comm"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rw!("mul-comm"; "(* ?a ?b)" => "(* ?b ?a)"),
    ]
}

#[derive(Debug, Clone, Serialize)]
pub struct TraceEvent {
    pub iter: usize,
    pub rewrite: String,
    pub eclass: u32,
    pub before_expr: String,
    pub after_expr: String,
    pub lhs_inst: Vec<String>,
    pub rhs_inst: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Coverage { pub ast: usize, pub eclass: u32 }

#[derive(Debug, Clone, Serialize)]
pub struct Candidate {
    pub name: String,
    pub lambda: f64,
    pub cost: f64,
    pub expr: String,
    pub coverage: Vec<Coverage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphExport { pub nodes: Vec<GraphNode>, pub links: Vec<GraphLink> }
#[derive(Debug, Clone, Serialize)]
pub struct GraphNode { pub id: u32, pub label: String, pub ache: f32 }
#[derive(Debug, Clone, Serialize)]
pub struct GraphLink { pub source: u32, pub target: u32, pub op: String, pub iter: usize }

#[derive(Clone, Copy)]
pub struct AcheBiCost<'a> { pub eg: &'a EGraph<Lang, AcheAnalysis>, pub lambda: f64 }
impl<'a> CostFunction<Lang> for AcheBiCost<'a> {
    type Cost = f64;
    fn cost<C>(&mut self, enode: &Lang, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let mut sz = 1.0;
        for c in enode.children().iter().copied() {
            sz += costs(c)
        }
        let id = self.eg.lookup(enode.clone()).unwrap();
        let ache = self.eg[id].data.ache.sum() as f64;
        sz - self.lambda * ache
    }
}

pub fn run_optimization(
    input_path: &std::path::Path,
    max_iters: usize,
    lambda: f64,
    nbest_count: usize,
) -> Result<(String, String, String)> {
    let expr_src = std::fs::read_to_string(input_path)?;
    let expr: Rec = expr_src.parse()?; // parse IR S-expr

    let mut eg: EGraph<Lang, AcheAnalysis> = EGraph::default();
    let root = eg.add_expr(&expr);
    let rewrites = default_rewrites();

    let mut trace: Vec<TraceEvent> = Vec::new();
    let mut iter = 0usize;

    loop {
        iter += 1;
        let mut applied_something = false;
        let egraph_before_iter = eg.clone();

        for rw in rewrites.iter() {
            let matches = rw.search(&eg);
            if matches.is_empty() { continue; }

            let size_extractor = Extractor::new(&egraph_before_iter, AstSize);
            let lhs_pat = rw.searcher.ast().unwrap();
            let rhs_pat_pretty = rw.applier.pretty(80);

            let mut before_map: BTreeMap<u32, String> = BTreeMap::new();
            let mut lhs_inst: Vec<String> = Vec::new();
            let mut rhs_inst: Vec<String> = Vec::new();

            for m in matches.iter() {
                for subst in m.substs.iter() {
                    lhs_inst.push(render_pattern(lhs_pat, subst, &size_extractor));
                    rhs_inst.push(rhs_pat_pretty.clone());
                }
                let eid = usize::from(m.eclass) as u32;
                if !before_map.contains_key(&eid) {
                    let (_, best) = size_extractor.find_best(m.eclass);
                    before_map.insert(eid, best.to_string());
                }
            }

            let applied = rw.apply(&mut eg, &matches).len();
            if applied > 0 { applied_something = true; }
            eg.rebuild();

            let size_extractor_after = Extractor::new(&eg, AstSize);
            for (eid, before_expr) in before_map {
                let id = Id::from(eid as usize);
                let after_expr = if eg.find(id) == id {
                    let (_, best_after) = size_extractor_after.find_best(id);
                    best_after.to_string()
                } else { "<merged>".to_string() };
                trace.push(TraceEvent {
                    iter,
                    rewrite: rw.name.to_string(),
                    eclass: eid,
                    before_expr,
                    after_expr,
                    lhs_inst: lhs_inst.clone(),
                    rhs_inst: rhs_inst.clone(),
                });
            }
        }

        if !applied_something || iter >= max_iters { break; }
    }

    // Build candidate set
    let mut nbest = Vec::<Candidate>::new();
    let mut seen = BTreeSet::<String>::new();

    // size-only
    let size_extractor = Extractor::new(&eg, AstSize);
    let (size_cost, size_expr) = size_extractor.find_best(root);
    let size_str = size_expr.to_string();
    if seen.insert(size_str.clone()) {
        nbest.push(Candidate {
            name: "size".into(), lambda: 0.0, cost: size_cost as f64,
            expr: size_str, coverage: coverage_for(&eg, &size_expr),
        });
    }

    // λ samples
    let mut i = 0usize;
    while nbest.len() < nbest_count {
        let lam = lambda * (1.0 + i as f64 * 0.4);
        let mut costfn = AcheBiCost { eg: &eg, lambda: lam };
        let extractor = Extractor::new(&eg, costfn);
        let (c, e) = extractor.find_best(root);
        let s = e.to_string();
        if seen.insert(s.clone()) {
            nbest.push(Candidate {
                name: format!("lambda_{}", i), lambda: lam, cost: c,
                expr: s, coverage: coverage_for(&eg, &e),
            });
        }
        i += 1;
    }

    let graph = export_graph(&eg);

    Ok((
        serde_json::to_string(&graph)?,
        serde_json::to_string(&trace)?,
        serde_json::to_string(&nbest)?,
    ))
}

fn render_pattern(pat: &PatternAst<Lang>, subst: &Subst, extractor: &Extractor<AstSize, Lang, AcheAnalysis>) -> String {
    let mut s = String::new(); write!(&mut s, "{}", pat.pretty(80)).ok();
    for (var, id) in subst.iter() {
        let (_, best) = extractor.find_best(*id);
        let needle = format!("?{}", var.to_string());
        s = s.replace(&needle, &best.to_string());
    }
    s
}

fn coverage_for(eg: &EGraph<Lang, AcheAnalysis>, expr: &Rec) -> Vec<Coverage> {
    let mut class_of: Vec<Option<Id>> = vec![None; expr.as_ref().len()];
    for (i, node) in expr.as_ref().iter().enumerate() {
        let mut enode = node.clone();
        for ch in enode.children_mut() {
            let idx = usize::from(*ch);
            let cid = class_of[idx].expect("child class missing");
            *ch = cid;
        }
        let eid = eg.lookup(enode).unwrap_or_else(|| Id::from(usize::MAX));
        class_of[i] = Some(eid);
    }
    class_of.into_iter().enumerate().map(|(ast, e)| Coverage {
        ast, eclass: e.map(|id| usize::from(id) as u32).unwrap_or(u32::MAX),
    }).collect()
}

fn export_graph(eg: &EGraph<Lang, AcheAnalysis>) -> GraphExport {
    let mut nodes = Vec::<GraphNode>::new();
    let mut links = Vec::<GraphLink>::new();
    for ec in eg.classes() {
        let eid = usize::from(ec.id) as u32;
        let label = ec.nodes.get(0).map(|n| n.to_string()).unwrap_or_else(|| "ε".into());
        let ache = ec.data.ache.sum();
        nodes.push(GraphNode { id: eid, label, ache });
        for n in &ec.nodes {
            for ch in n.children() {
                links.push(GraphLink { source: eid, target: usize::from(*ch) as u32, op: n.to_string(), iter: 0 });
            }
        }
    }
    GraphExport { nodes, links }
}
