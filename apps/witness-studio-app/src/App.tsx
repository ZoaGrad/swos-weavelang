import React, { useEffect, useState } from 'react';
import { GraphCanvas } from './components/GraphCanvas';
import type { GlyphIR } from 'weavelang-core/src/ir';

function App() {
  const [ir, setIr] = useState<GlyphIR | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // In a dev server, files in `public` are served from the root.
    // This path should correspond to `examples/hello.json` being available.
    // We'll need to make sure `examples` is copied to the app's public dir.
    // For now, we assume it's available at /hello.json
    fetch('/hello.json')
      .then(res => {
        if (!res.ok) {
          throw new Error(`HTTP error! status: ${res.status}`);
        }
        return res.json();
      })
      .then(data => setIr(data))
      .catch(err => {
        console.error("Failed to load IR:", err);
        setError("Failed to load IR data. See console for details.");
      });
  }, []);

  if (error) {
    return <div>Error: {error}</div>;
  }

  if (!ir) {
    return <div>Loading IR...</div>;
  }

  return (
    <div className="App">
      <header>
        <h1>Witness Studio</h1>
      </header>
      <main>
        <GraphCanvas graph={ir} />
      </main>
    </div>
  );
}

export default App;
