const svg = d3.select("#viz"), g = svg.append("g");
const linkG = g.append("g").attr("stroke-linecap","round");
const nodeG = g.append("g");
let sim, graph, trace=[], nbest=[];
const iterSlider = document.getElementById("iter");
const iterVal = document.getElementById("iterVal");
const loadBtn = document.getElementById("load");
const candSelect = document.getElementById("candidate");
const exprBox = document.getElementById("exprBox");
const ov = { wrap:document.getElementById("overlay"), close:document.getElementById("overlayClose"),
  iter:ovId("iter"), rewrite:ovId("rewrite"), eclass:ovId("eclass"),
  lhs:ovId("lhs"), rhs:ovId("rhs"), before:ovId("before"), after:ovId("after") };
function ovId(k){ return document.getElementById("ov-"+k); }
function resize(){ const {innerWidth:w,innerHeight:h}=window; svg.attr("width",w).attr("height",Math.max(280,h-128)); }
window.addEventListener("resize",resize); resize();
svg.call(d3.zoom().scaleExtent([0.2,3]).on("zoom",e=>g.attr("transform",e.transform)));

loadBtn.onclick = async () => {
  const gp=document.getElementById("graphPath").value||"graph.json";
  const tp=document.getElementById("tracePath").value||"trace.json";
  const np=document.getElementById("nbestPath").value||"nbest.json";
  graph = await (await fetch(gp)).json();
  try{ trace = await (await fetch(tp)).json(); }catch{ trace = []; }
  try{ nbest = await (await fetch(np)).json(); }catch{ nbest = []; }
  initGraph(graph); initTrace(trace); initCandidates(nbest);
};

function initGraph(data){
  const nodes = data.nodes.map(d=>({...d}));
  const links = data.links.map(d=>({...d}));

  sim = d3.forceSimulation(nodes)
    .force("charge", d3.forceManyBody().strength(-220))
    .force("link", d3.forceLink(links).id(d=>d.id).distance(80).strength(0.9))
    .force("center", d3.forceCenter(innerWidth/2, Math.max(240,(innerHeight-128)/2)));

  linkG.selectAll("line").data(links, d=>d.source+"->"+d.target).join("line")
    .attr("class","link").attr("stroke-width",0.8)
    .append("title").text(d=>`${d.op}  c${idOf(d.source)}→c${idOf(d.target)}`);

  const acheExtent = d3.extent(nodes, d=>d.ache);
  const color = d3.scaleLinear().domain(acheExtent).range(["#1a1f3b","#3ea3ff"]);

  nodeG.selectAll("g").data(nodes, d=>d.id).join(enter=>{
    const g = enter.append("g").attr("class","node");
    g.append("circle").attr("r",14);
    g.append("text").attr("text-anchor","middle").attr("dy","0.35em").text(d=>`c${d.id}`);
    g.append("title").text(d=>`e-class c${d.id}\nlabel: ${d.label}\nache≈${d.ache.toFixed(2)}`);
    return g;
  }).select("circle").attr("fill", d=>color(d.ache)).attr("stroke","#9ad1ff");

  sim.on("tick", ()=>{
    linkG.selectAll("line")
      .attr("x1", d=>pt(nodes,d.source).x).attr("y1", d=>pt(nodes,d.source).y)
      .attr("x2", d=>pt(nodes,d.target).x).attr("y2", d=>pt(nodes,d.target).y);
    nodeG.selectAll("g").attr("transform", d=>`translate(${d.x},${d.y})`);
  });
}
function idOf(x){ return (typeof x==="object") ? (x.id ?? x) : x; }
function pt(nodes,id){ id=idOf(id); return nodes.find(n=>n.id===id)||nodes[0]; }

function initTrace(events){
  iterSlider.max = Math.max(0, events.length); iterSlider.value = 0; iterVal.textContent = "0";
  iterSlider.oninput = ()=>{ const k=+iterSlider.value; iterVal.textContent=`${k}`; highlightUpTo(events,k); if(k>0) showEventOverlay(events[k-1]); };
  highlightUpTo(events,0);
}
function highlightUpTo(events,k){
  nodeG.selectAll("g").classed("highlight",false);
  linkG.selectAll("line").classed("highlight",false);
  for(let i=0;i<k && i<events.length;i++){
    const ev = events[i];
    nodeG.selectAll("g").filter(d=>d.id===ev.eclass).classed("highlight",true);
  }
}
function showEventOverlay(ev){
  ov.iter.textContent=ev.iter; ov.rewrite.textContent=ev.rewrite; ov.eclass.textContent=ev.eclass;
  ov.before.textContent=ev.before_expr||"—"; ov.after.textContent=ev.after_expr||"—";
  ov.lhs.textContent=(ev.lhs_inst&&ev.lhs_inst[0])?ev.lhs_inst[0]:"—";
  ov.rhs.textContent=(ev.rhs_inst&&ev.rhs_inst[0])?ev.rhs_inst[0]:"—";
  ov.wrap.classList.remove("hidden");
}
ov.close.onclick=()=>ov.wrap.classList.add("hidden");
ov.wrap.addEventListener("click",(e)=>{ if(e.target===ov.wrap) ov.wrap.classList.add("hidden"); });

function initCandidates(list){
  const sel=candSelect; sel.innerHTML="";
  const empty=document.createElement("option"); empty.value=""; empty.textContent=list.length?"(select candidate)":"(no nbest.json)"; sel.appendChild(empty);
  list.forEach((c,idx)=>{ const o=document.createElement("option"); o.value=idx; o.textContent=`${c.name}  λ=${(+c.lambda).toFixed(2)}  cost=${(+c.cost).toFixed(3)}`; sel.appendChild(o); });
  sel.onchange=()=>{
    nodeG.selectAll("g").classed("cand",false); linkG.selectAll("line").classed("cand",false); exprBox.textContent="";
    if(sel.value==="") return; const cand=list[+sel.value]; exprBox.textContent=wrapExpr(cand.expr);
    if(cand.coverage && cand.coverage.length){
      const covered=new Set(cand.coverage.map(c=>c.eclass).filter(Number.isFinite));
      nodeG.selectAll("g").each(function(d){ if(covered.has(d.id)) d3.select(this).classed("cand",true); });
      linkG.selectAll("line").each(function(d){ const s=idOf(d.source), t=idOf(d.target); if(covered.has(s)&&covered.has(t)) d3.select(this).classed("cand",true); });
    } else {
      const ops=new Set((cand.expr.match(/[a-zA-Z_][a-zA-Z0-9_\-]*/g)||[]));
      nodeG.selectAll("g").each(function(d){ if(ops.has(d.label)) d3.select(this).classed("cand",true); });
      linkG.selectAll("line").each(function(d){ if(ops.has(d.op)) d3.select(this).classed("cand",true); });
    }
  };
}
function wrapExpr(s){ return String(s).replace(/\)\s*\(/g,")\n(").replace(/\s{2,}/g," "); }
document.getElementById("load").click();
