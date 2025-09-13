import React, { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import type { GlyphIR, Glyph, Edge } from 'weavelang-core/src/ir';

type GraphCanvasProps = {
  graph: GlyphIR;
  highlight?: { nodes?: string[]; edges?: string[] };
  onSelect?: (ids: string[]) => void;
};

// D3 needs nodes to have x, y properties.
type SimulationNode = Glyph & { x?: number; y?: number; fx?: number | null; fy?: number | null; };
type SimulationLink = { source: string; target: string; };

export const GraphCanvas: React.FC<GraphCanvasProps> = ({ graph }) => {
  const ref = useRef<SVGSVGElement>(null);

  useEffect(() => {
    if (!graph || !ref.current) {
      return;
    }

    const svg = d3.select(ref.current);
    svg.selectAll("*").remove(); // Clear previous render

    const width = svg.node()?.getBoundingClientRect().width ?? 800;
    const height = svg.node()?.getBoundingClientRect().height ?? 600;

    const nodes: SimulationNode[] = graph.glyphs.map(d => ({ ...d }));
    const links: SimulationLink[] = graph.edges.map(d => ({ source: d.source, target: d.target }));

    const simulation = d3.forceSimulation(nodes)
      .force("link", d3.forceLink(links).id((d: any) => d.id).distance(100))
      .force("charge", d3.forceManyBody().strength(-300))
      .force("center", d3.forceCenter(width / 2, height / 2));

    const link = svg.append("g")
      .attr("stroke", "#999")
      .attr("stroke-opacity", 0.6)
      .selectAll("line")
      .data(links)
      .join("line")
      .attr("stroke-width", 2);

    const node = svg.append("g")
      .attr("stroke", "#fff")
      .attr("stroke-width", 1.5)
      .selectAll("g")
      .data(nodes)
      .join("g");

    node.append("circle")
      .attr("r", 15)
      .attr("fill", "#333");

    node.append("text")
      .attr("x", 20)
      .attr("y", "0.31em")
      .text(d => d.label)
      .attr("fill", "#fff")
      .attr("stroke", "none")
      .attr("font-size", "12px");

    node.append("title")
      .text(d => d.label);

    simulation.on("tick", () => {
      link
        .attr("x1", d => (d.source as SimulationNode).x!)
        .attr("y1", d => (d.source as SimulationNode).y!)
        .attr("x2", d => (d.target as SimulationNode).x!)
        .attr("y2", d => (d.target as SimulationNode).y!);

      node
        .attr("transform", d => `translate(${d.x}, ${d.y})`);
    });

  }, [graph]);

  return (
    <svg ref={ref} style={{ width: '100%', height: 'calc(100vh - 50px)', background: '#111' }}></svg>
  );
};
