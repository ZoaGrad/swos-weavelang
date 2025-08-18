/**
 * Represents a position in the source code.
 */
export interface SourceSpan {
  start: number;
  end: number;
}

/**
 * Represents the provenance of a glyph or edge, tracking its origin.
 */
export interface Provenance {
  sourceSpan?: SourceSpan;
  ruleHistory?: string[]; // Names of rules that created/modified this
  semiringTrace?: any;   // Data from the semiring analysis
}

/**
 * Visual cues for the Witness Studio renderer.
 */
export interface ViewHints {
  color?: string;
  layout?: { x?: number; y?: number; fixed?: boolean };
}

/**
 * A generic key-value store for metadata.
 */
export interface Tag {
  key: string;
  value: any;
}

/**
 * The core "atom" of the WeaveLang graph.
 */
export interface Glyph {
  id: string;          // Unique identifier for the glyph
  label: string;       // The operator or symbol
  ache?: number;       // The "ache" or cost, from semiring analysis
  provenance?: Provenance;
  viewHints?: ViewHints;
  tags?: Tag[];
}

/**
 * A directed connection between two glyphs.
 */
export interface Edge {
  id: string;          // Unique identifier for the edge
  source: string;      // ID of the source glyph
  target: string;      // ID of the target glyph
  label?: string;      // Optional label for the edge (e.g., child index)
  provenance?: Provenance;
  viewHints?: ViewHints;
}

/**
 * The complete Intermediate Representation of a WeaveLang program.
 */
export interface GlyphIR {
  glyphs: Glyph[];
  edges: Edge[];
}

/**
 * Serializes a GlyphIR object to a JSON string.
 * @param ir The GlyphIR object to serialize.
 * @returns A JSON string representation.
 */
export function serializeIR(ir: GlyphIR): string {
  return JSON.stringify(ir, null, 2);
}

/**
 * Deserializes a JSON string into a GlyphIR object.
 * @param json The JSON string to deserialize.
 * @returns A GlyphIR object.
 */
export function deserializeIR(json: string): GlyphIR {
  return JSON.parse(json);
}
