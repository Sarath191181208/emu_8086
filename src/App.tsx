import "./App.css";
import { Editor } from "@monaco-editor/react";

function App() {
  // The language of the editor is assembly

  return (
    <div className="App">
      <header className="App-header">
        <Editor
          height="90vh"
          theme="vs-dark"
          defaultLanguage="x86_64-assembly"
          defaultValue="// some comment"
          options={{ minimap: { enabled: false } }}
        />
      </header>
    </div>
  );
}

export default App;
