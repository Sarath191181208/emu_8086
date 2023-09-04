import "./App.css";
import "./index.css";
import { Editor } from "@monaco-editor/react";

function App() {
  // The language of the editor is assembly

  return (
    <div className="App">
      {/* create a css header example */}
      <header>
        <h1 className="text-sm text-blue-400">Assembly Editor</h1>
      </header>

      <Editor
        height="90vh"
        theme="vs-dark"
        defaultLanguage="x86_64-assembly"
        defaultValue="// some comment"
        options={{ minimap: { enabled: false } }}
      />
    </div>
  );
}

export default App;
